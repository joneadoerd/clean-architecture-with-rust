use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound,
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::Thought;
    use application::usecase::thought::find_by_id as uc;
    use std::convert::TryFrom;

    impl From<uc::Response> for Thought {
        fn from(from: uc::Response) -> Self {
            let uc::Response { id, title } = from;
            let id = id.to_u64().into();
            Self { id, title }
        }
    }

    impl TryFrom<uc::Error> for Error {
        type Error = ();
        fn try_from(from: uc::Error) -> Result<Self, Self::Error> {
            match from {
                uc::Error::NotFound => Ok(Self::NotFound),
                uc::Error::Repo => Err(()),
            }
        }
    }
}
