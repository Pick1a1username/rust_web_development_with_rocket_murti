use std::fmt;

use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket::serde::{Serialize, Serializer};
use rocket_db_pools::{sqlx::FromRow};
use uuid::{Uuid};


// uuid::Uuid cannot be used with rocket 0.5.0-rc.2.
// so, try to do something like OurDateTime.
#[derive(sqlx::Type, Debug, FromRow)]
#[sqlx(transparent)]
pub struct OurUuid(
    pub Uuid
);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurUuid {
    fn from_value(_: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
    async fn from_data(_: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}

impl fmt::Display for OurUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let OurUuid(uuid) = self;
        write!(f, "{}", uuid)
    }
}

impl Serialize for OurUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("OurUuid", &self.to_string())
    }
}