use crate::err;
use egg_mode::tweet::Tweet;
use egg_mode::{KeyPair, Response};

#[derive(Clone, Debug)]
pub struct Credential {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

impl Credential {
    fn token(&self) -> egg_mode::Token {
        egg_mode::Token::Access {
            consumer: KeyPair::new(self.consumer_key.clone(), self.consumer_secret.clone()),
            access: KeyPair::new(self.access_key.clone(), self.access_secret.clone()),
        }
    }
    pub fn post(&self, draft: String) -> Result<u64, err::Error> {
        let resp = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(egg_mode::tweet::DraftTweet::new(draft).send(&self.token()))?;
        Ok(resp.response.id)
    }

    pub fn delete(&mut self, tweet_id: u64) -> Result<(), err::Error> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(egg_mode::tweet::delete(tweet_id, &self.token()))?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct CredentialOpt {
    pub consumer_key: Option<String>,
    pub consumer_secret: Option<String>,
    pub access_key: Option<String>,
    pub access_secret: Option<String>,
}

impl CredentialOpt {
    pub(crate) fn to_credential(self) -> Result<Credential, err::Error> {
        if self.consumer_key.is_none() {
            return Err(err::Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: consumer_key is none".to_string(),
            ));
        }
        if self.consumer_secret.is_none() {
            return Err(err::Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: consumer_secret is none"
                    .to_string(),
            ));
        }
        if self.access_key.is_none() {
            return Err(err::Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: access_key is none".to_string(),
            ));
        }
        if self.access_secret.is_none() {
            return Err(err::Error::new(
                None,
                "Unable to convert CredentialOpt to Credential: access_secret is none".to_string(),
            ));
        }

        Ok(Credential {
            consumer_key: self.consumer_key.unwrap(),
            consumer_secret: self.consumer_secret.unwrap(),
            access_key: self.access_key.unwrap(),
            access_secret: self.access_secret.unwrap(),
        })
    }
}
