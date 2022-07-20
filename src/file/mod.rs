pub mod local;
pub mod s3;

use crate::err;

pub trait File {
    fn data(&self) -> Result<Vec<u8>, err::Error>;
    fn mime(&self) -> Option<mime::Mime>;
}
