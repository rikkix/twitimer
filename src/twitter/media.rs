use super::Credential;
use crate::{err, file};
use egg_mode::media;

impl Credential {
    pub async fn upload(&self, f: Box<dyn file::File>) -> Result<media::MediaHandle, err::Error> {
        Ok(media::upload_media(
            f.data()?.as_slice(),
            &f.mime().unwrap_or(mime::APPLICATION_OCTET_STREAM),
            &self.token(),
        )
        .await?)
    }
}
