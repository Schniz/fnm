use std::io::Read;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Response;

pub struct ResponseProgress {
    progress: Option<ProgressBar>,
    response: Response,
}

fn make_progress_bar(size: u64) -> ProgressBar {
    let bar = ProgressBar::new(size);

    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    bar
}

impl ResponseProgress {
    pub fn new(response: Response) -> Self {
        Self {
            progress: response.content_length().map(make_progress_bar),
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
        self.finish()
    }
}
