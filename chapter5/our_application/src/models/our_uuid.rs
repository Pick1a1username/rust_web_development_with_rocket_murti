use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket_db_pools::{sqlx::FromRow};
use uuid::Uuid;

// uuid::Uuid cannot be used with rocket 0.5.0-rc.2.
// so, try to do something like OurDateTime.
#[derive(Debug, FromRow)]
pub struct OurUuid(Uuid);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurUuid {
    fn from_value(_: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
    async fn from_data(_: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}