use actix::{ Addr, Syn};
use actix_web::{fs::NamedFile, HttpRequest, HttpResponse, multipart, 
                error, Error, Json, Result, FutureResponse, HttpMessage};
use model::db::ConnDsl;

use std::fs;
use std::io::Write;
use futures::future;
use futures::{Future, Stream};

pub struct AppState {
    pub db: Addr<Syn, ConnDsl>
}

#[derive(Deserialize, Serialize)]
struct Picture {
    size: i64,
    path: String,
}

pub fn home(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

pub fn path(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

pub fn upload(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let file_path = "res/video/upload.mp4".to_string();
    Box::new(
        req.clone()
            .multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(
                Picture{
                    size: sizes[0],
                    path: file_path,
                }))
            .map_err(|e| {
                println!("failed: {}", e);
                e
            }),
    )
}

pub fn handle_multipart_item(item: multipart::MultipartItem<HttpRequest<AppState>>) -> Box<Stream<Item = i64, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(save_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}

pub fn save_file(field: multipart::Field<HttpRequest<AppState>>) -> Box<Future<Item = i64, Error = Error>> {
    let file_path_string = "resource/video/upload.mp4";
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };
    Box::new(
        field
            .fold(0i64, move |acc, bytes| {
                let rt = file
                    .write_all(bytes.as_ref())
                    .map(|_| acc + bytes.len() as i64)
                    .map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        error::MultipartError::Payload(error::PayloadError::Io(e))
                    });
                future::result(rt)
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}



