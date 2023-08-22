use std::io::Read;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use reqwest::blocking::Response;

pub struct ResponseProgress {
    progress: Option<ProgressBar>,
    response: Response,
}

fn make_progress_bar(size: u64, target: ProgressDrawTarget) -> ProgressBar {
    let bar = ProgressBar::with_draw_target(Some(size), target);

    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    bar
}

impl ResponseProgress {
    pub fn new(response: Response, target: ProgressDrawTarget) -> Self {
        Self {
            progress: response
                .content_length()
                .map(|len| make_progress_bar(len, target)),
            response,
        }
    }

    pub fn finish(&self) {
        if let Some(ref bar) = self.progress {
            bar.finish();
        }
    }
}

impl Read for ResponseProgress {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.response.read(buf)?;

        if let Some(ref bar) = self.progress {
            bar.inc(size as u64);
        }

        Ok(size)
    }
}

impl Drop for ResponseProgress {
    fn drop(&mut self) {
        self.finish();
    }
}

#[cfg(test)]
mod tests {
    use indicatif::ProgressDrawTarget;
    use reqwest::blocking::Response;
    use std::io::Read;

    use super::ResponseProgress;

    const CONTENT_LENGTH: usize = 100;

    #[test]
    fn test_reads_data() {
        let response: Response = http::Response::builder()
            .header("Content-Length", CONTENT_LENGTH)
            .body("a".repeat(CONTENT_LENGTH))
            .unwrap()
            .into();

        let mut buf = [0; CONTENT_LENGTH];

        let mut progress = ResponseProgress::new(response, ProgressDrawTarget::hidden());
        let size = progress.read(&mut buf[..]).unwrap();

        assert_eq!(size, CONTENT_LENGTH);
        assert_eq!(buf, "a".repeat(CONTENT_LENGTH).as_bytes());
    }
}
