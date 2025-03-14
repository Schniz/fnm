use super::extract::{Error, Extract};
use log::debug;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use tempfile::tempfile;
use zip::read::ZipArchive;

pub struct Zip<R: Read> {
    response: R,
}

impl<R: Read> Zip<R> {
    #[allow(dead_code)]
    pub fn new(response: R) -> Self {
        Self { response }
    }
}

impl<R: Read> Extract for Zip<R> {
    fn extract_into(mut self: Box<Self>, path: &Path) -> Result<(), Error> {
        let mut tmp_zip_file = tempfile().expect("Can't get a temporary file");

        debug!("Created a temporary zip file");
        io::copy(&mut self.response, &mut tmp_zip_file)?;
        debug!(
            "Wrote zipfile successfully. Now extracting into {}.",
            path.display()
        );

        let mut archive = ZipArchive::new(&mut tmp_zip_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = path.join(file.mangled_name());

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    debug!("File {} comment: {}", i, comment);
                }
            }

            if file.name().ends_with('/') {
                debug!(
                    "File {} extracted to \"{}\"",
                    i,
                    outpath.as_path().display()
                );
                fs::create_dir_all(&outpath)?;
            } else {
                debug!(
                    "Extracting file {} to \"{}\" ({} bytes)",
                    i,
                    outpath.as_path().display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_zip_extraction() {
        let temp_dir = &tempfile::tempdir().expect("Can't create a temp directory");
        let response = crate::http::get("https://nodejs.org/dist/v12.0.0/node-v12.0.0-win-x64.zip")
            .expect("Can't make request to Node v12.0.0 zip file");
        Box::new(Zip::new(response))
            .extract_into(temp_dir.as_ref())
            .expect("Can't unzip files");
        let node_file = temp_dir
            .as_ref()
            .join("node-v12.0.0-win-x64")
            .join("node.exe");
        assert!(node_file.exists());
    }
}
