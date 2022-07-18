use crate::err;
use crate::version::Version;

pub fn compatible(current_ver: &Version, db_ver: &Version) -> Result<(), err::Error> {
    if current_ver.clone().to_store() < db_ver.clone().to_store() {
        return Err(err::Error::new(
            None,
            "Database version can NOT be higher than current twitimer version.".to_string(),
        ));
    }

    // Exceptions:
    // not exceptions now

    if current_ver.major != db_ver.major {
        return Err(err::Error::new(
            None,
            "Major version between database and twitimer MUST be equal.".to_string(),
        ));
    }
    Ok(())
}
