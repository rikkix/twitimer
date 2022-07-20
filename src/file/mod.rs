pub mod local;
pub mod s3;

use crate::err;

trait File {
    fn data(&self) -> Result<&[u8], err::Error>;
    fn mime(&self) -> Option<mime::Mime>;
}
