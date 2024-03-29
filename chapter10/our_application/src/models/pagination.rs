use rocket::serde::Serialize;

use super::our_date_time::OurDateTime;

pub const DEFAULT_LIMIT: usize = 10;

#[derive(FromForm)]
pub struct Pagination {
    pub next: OurDateTime,
    pub limit: usize,
}

#[derive(Serialize)]
pub struct PaginationContext {
    pub next: i64,
    pub limit: usize,
}
impl Pagination {
    pub fn to_context(&self) -> PaginationContext {
        PaginationContext {
            next: self.next.0.timestamp_nanos(),
            limit: self.limit,
        }
    }
}