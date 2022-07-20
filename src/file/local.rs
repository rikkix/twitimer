use crate::err;
use std::{fs, path};

pub struct LocalFile<'a> {
    path: &'a path::Path,
    mime: Option<mime::Mime>,
}

impl LocalFile<'_> {
    pub fn new(path: &str) -> Result<LocalFile, err::Error> {
        let opt_type = infer::get_from_path(path)?;
        Ok(LocalFile {
            path: path::Path::new(path),
            mime: opt_type.map(|t| {
                t.mime_type()
                    .parse::<mime::Mime>()
                    .expect("Error when parsing mime type")
            }),
        })
    }
}

impl super::File for LocalFile<'_> {
    fn data(&self) -> Result<Vec<u8>, err::Error> {
        Ok(fs::read(self.path)?.to_vec())
    }
    fn mime(&self) -> Option<mime::Mime> {
        self.mime.clone()
    }
}
