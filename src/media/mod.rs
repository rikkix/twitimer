use crate::err;
use egg_mode::media;

fn upload() -> Result<media::MediaId, err::Error> {
    media::upload_media()
}
