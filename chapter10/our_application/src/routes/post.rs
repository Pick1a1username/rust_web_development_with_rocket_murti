use std::io::Cursor;
use std::ops::Deref;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use image::error::ImageError;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageEncoder};
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket_dyn_templates::{context, Template};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, post::{NewPost, Post, ShowPost}, post_type::PostType, user::User};

#[get("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
pub async fn get_post(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
) -> HtmlResponse {
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, user_uuid)
        .await
        .map_err(|e| e.status)?;
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let post = Post::find(connection, uuid).await.map_err(|e| e.status)?;
    if post.user_uuid.to_string() != user.uuid.to_string() {
        return Err(Status::InternalServerError);
    }
    let context = context! { user, post: &(post.to_show_post())};
    Ok(Template::render("posts/show", context))
}

#[get("/users/<user_uuid>/posts?<pagination>", format = "text/html")]
pub async fn get_posts(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    pagination: Option<Pagination>,
    _flash: Option<FlashMessage<'_>>,
) -> HtmlResponse {
    let user = User::find(&mut db, user_uuid).await.map_err(|e| e.status)?;
    let (posts, new_pagination) = Post::find_all(&mut db, user_uuid, pagination)
        .await
        .map_err(|e| e.status)?;
    let show_posts: Vec<ShowPost> = posts
        .into_iter()
        .map(|post| post.to_show_post())
        .collect();
    let context = context! {user, posts: &show_posts, pagination: new_pagination.map(|pg|pg.to_context())};
    Ok(Template::render("posts/index", context))
}

#[post("/users/<user_uuid>/posts", format = "multipart/form-data", data = "<upload>", rank = 1)]
pub async fn create_post<'r>(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    mut upload: Form<NewPost<'r>>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let create_err = || {
        Flash::error(
            Redirect::to(format!("/users/{}/posts", user_uuid)),
            "Something went wrong when uploading file",
        )
    };
    let file_uuid = uuid::Uuid::new_v4().to_string();
    if upload.file.content_type().is_none() {
        return Err(create_err());
    }
    let ext = upload.file.content_type().unwrap().extension().unwrap();
    let tmp_filename = format!("/tmp/{}.{}", &file_uuid, &ext);
    upload
        .file
        .persist_to(tmp_filename)
        .await
        .map_err(|_| create_err())?;
    let mut content = String::new();
    let mut post_type = PostType::Text;
    let mt = upload.file.content_type().unwrap().deref();
    if mt.is_text() {
        let orig_path = upload.file.path().unwrap().to_string_lossy().to_string();
        let mut text_content = vec![];
        let _ = File::open(orig_path)
            .await
            .map_err(|_| create_err())?
            .read_to_end(&mut text_content)
            .await
            .map_err(|_| create_err())?;
        content.push_str(std::str::from_utf8(&text_content).unwrap());
    } else if mt.is_bmp() || mt.is_jpeg() || mt.is_png() || mt.is_gif() {
        post_type = PostType::Photo;
        let orig_path = upload.file.path().unwrap().to_string_lossy().to_string();
        let dest_filename = format!("{}.jpg", file_uuid);
        content.push_str("/assets/");
        content.push_str(&dest_filename);
    
        let orig_file = tokio::fs::read(orig_path).await.map_err(|_| create_err())?;
        let read_buffer = Cursor::new(orig_file);
        let encoded_result: Result<DynamicImage, ()> = tokio::task::spawn_blocking(|| {
            Ok(ImageReader::new(read_buffer)
                .with_guessed_format()
                .map_err(|_| ())?
                .decode()
                .map_err(|_| ())?)
        })
            .await
            .map_err(|_| create_err())?;
        let image = encoded_result.map_err(|_| create_err())?;
        
        let write_result: Result<Vec<u8>, ImageError> = tokio::task::spawn_blocking(move || {
            let mut write_buffer: Vec<u8> = vec![];
            let mut write_cursor = Cursor::new(&mut write_buffer);
            let _ = JpegEncoder::new_with_quality(&mut write_cursor, 75).write_image(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color(),
            )?;
            Ok(write_buffer)
        })
            .await
            .map_err(|_| create_err())?;
        let write_bytes = write_result.map_err(|_| create_err())?;
        let dest_path = Path::new(rocket::fs::relative!("static")).join(&dest_filename);
        tokio::fs::write(dest_path, &write_bytes)
            .await
            .map_err(|_| create_err())?;
    } else if mt.is_svg() {
        post_type = PostType::Photo;
        let dest_filename = format!("{}.svg", file_uuid);
        content.push_str("/assets/");
        content.push_str(&dest_filename);
        let dest_path = Path::new(rocket::fs::relative!("static")).join(&dest_filename);
        upload
            .file
            .move_copy_to(&dest_path)
            .await
            .map_err(|_| create_err())?;
    } else {
        return Err(create_err());
    }

    let connection = db.acquire().await.map_err(|_| create_err())?;
    Post::create(connection, user_uuid, post_type, &content)
        .await
        .map_err(|_| create_err())?;
    Ok(Flash::success(
        Redirect::to(format!("/users/{}/posts", user_uuid)),
        "Successfully created post",
    ))
}

#[delete("/users/<_user_uuid>/posts/<_uuid>", format = "text/html")]
pub async fn delete_post(
    mut _db: Connection<DBConnection>,
    _user_uuid: &str,
    _uuid: &str
) -> HtmlResponse {
    todo!("will implement later")
}