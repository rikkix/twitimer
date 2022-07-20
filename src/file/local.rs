use crate::err;
use std::{fs, path};

pub struct LocalFile<'a> {
    path: &'a path::Path,
    mime: Option<mime::Mime>,
}

impl LocalFile {
    pub fn new(path: &str) -> Result<LocalFile, err::Error> {
        let opt_type = infer::get_from_path(path)?;
        let mut m: Option<mime::Mime> = None;
        if opt_type.is_some() {
            m = opt_type.unwrap().mime_type().parse()?;
        }

        Ok(LocalFile {
            path: path::Path::new(path),
            mime: m,
        })
    }
}

impl super::File for LocalFile {
    fn data(&self) -> Result<Vec<u8>, err::Error> {
        Ok(fs::read(self.path)?)
    }
    fn mime(&self) -> Option<mime::Mime> {
        self.mime.clone()
    }
}
