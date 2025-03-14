use std::io::Read;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use reqwest::blocking::Response;

pub struct ResponseProgress {
    progress: Option<ProgressBar>,
    response: Response,
}

#[derive(Default, Clone, Debug, clap::ValueEnum)]
pub enum ProgressConfig {
    #[default]
    Auto,
    Never,
    Always,
}

impl ProgressConfig {
    pub fn enabled(&self, config: &crate::config::FnmConfig) -> bool {
        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Auto => config
                .log_level()
                .is_writable(crate::log_level::LogLevel::Info),
        }
    }
}

fn make_progress_bar(size: u64, target: ProgressDrawTarget) -> ProgressBar {
    let bar = ProgressBar::with_draw_target(Some(size), target);

    bar.set_style(
        ProgressStyle::with_template(
            "{elapsed_precise:.white.dim} {wide_bar:.cyan} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
        )
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  "),
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
        eprintln!();
    }
}

#[cfg(test)]
mod tests {
    use indicatif::{ProgressDrawTarget, TermLike};
    use reqwest::blocking::Response;
    use std::{
        io::Read,
        sync::{Arc, Mutex},
    };

    use super::ResponseProgress;

    const CONTENT_LENGTH: usize = 100;

    #[derive(Debug)]
    struct MockedTerm {
        pub buf: Arc<Mutex<String>>,
    }

    impl TermLike for MockedTerm {
        fn width(&self) -> u16 {
            80
        }

        fn move_cursor_up(&self, _n: usize) -> std::io::Result<()> {
            Ok(())
        }

        fn move_cursor_down(&self, _n: usize) -> std::io::Result<()> {
            Ok(())
        }

        fn move_cursor_right(&self, _n: usize) -> std::io::Result<()> {
            Ok(())
        }

        fn move_cursor_left(&self, _n: usize) -> std::io::Result<()> {
            Ok(())
        }

        fn write_line(&self, s: &str) -> std::io::Result<()> {
            self.buf.lock().unwrap().push_str(s);
            Ok(())
        }

        fn write_str(&self, s: &str) -> std::io::Result<()> {
            self.buf.lock().unwrap().push_str(s);
            Ok(())
        }

        fn clear_line(&self) -> std::io::Result<()> {
            Ok(())
        }

        fn flush(&self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_reads_data_and_shows_progress() {
        let response: Response = http::Response::builder()
            .header("Content-Length", CONTENT_LENGTH)
            .body("a".repeat(CONTENT_LENGTH))
            .unwrap()
            .into();

        let mut buf = [0; CONTENT_LENGTH];

        let out_buf = Arc::new(Mutex::new(String::new()));

        let mut progress = ResponseProgress::new(
            response,
            ProgressDrawTarget::term_like(Box::new(MockedTerm {
                buf: out_buf.clone(),
            })),
        );
        let size = progress.read(&mut buf[..]).unwrap();

        drop(progress);

        assert_eq!(size, CONTENT_LENGTH);
        assert_eq!(buf, "a".repeat(CONTENT_LENGTH).as_bytes());
        assert!(out_buf.lock().unwrap().contains(&"█".repeat(40)));
    }
}
