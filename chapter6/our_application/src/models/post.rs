use super::our_date_time::OurDateTime;
use super::our_uuid::OurUuid;
use super::post_type::PostType;
use rocket::form::FromForm;

#[derive(FromForm)]
pub struct Post {
    pub uuid: OurUuid,
    pub user_uuid: OurUuid,
    pub post_type: PostType,
    pub content: String,
    pub created_at: OurDateTime,
}