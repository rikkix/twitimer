use crate::err;
use futures::executor::block_on;

pub struct Bucket {
    b: s3::Bucket,
}

struct S3File<'a> {
    bucket: &'a Bucket,
    path: String,
}

impl Bucket {
    fn new(
        region_name: &str,
        endpoint: &str,
        bucket_name: &str,
        cred: awscreds::Credentials,
    ) -> Result<Bucket, err::Error> {
        Ok(Bucket {
            b: s3::Bucket::new(
                bucket_name,
                s3::Region::Custom {
                    region: region_name.to_string(),
                    endpoint: endpoint.to_string(),
                },
                cred,
            )?,
        })
    }
}

impl super::File for S3File<'_> {
    fn data(&self) -> Result<Vec<u8>, err::Error> {
        let (data, code) = block_on(self.bucket.b.get_object(&self.path))?;
        //TODO: check code
        Ok(data)
    }

    fn mime(&self) -> Option<mime::Mime> {
        let r = block_on(self.bucket.b.head_object(&self.path));
        if r.is_err() {
            return None;
        }
        let (head, code) = r.unwrap();
        //TODO: check code
        let file_type = head.content_type;
        if file_type.is_some() {
            return Some(
                file_type
                    .unwrap()
                    .parse::<mime::Mime>()
                    .expect("Error when parsing mime type"),
            );
        }
        let (buf, code) = block_on(self.bucket.b.get_object_range(&self.path, 0, Some(32)))
            .expect("Error when getting the buf of object");
        //TODO: check code
        let infer_type = infer::get(buf.as_slice());
        infer_type.map(|t| {
            t.mime_type()
                .parse::<mime::Mime>()
                .expect("Error when parsing mime type")
        })
    }
}
