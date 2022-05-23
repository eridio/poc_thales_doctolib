use actix_web::{web, web::ServiceConfig, HttpResponse};
mod signin;
mod authentication;
use crate::errors::AppError;

use signin::{signin,verify_file_1,verify_file_2,verify_file_3};

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;


pub fn app_config(config : &mut ServiceConfig) {

    let signin = web::resource("/signin").route(web::post().to(signin));
    let verify_file_1 = web::resource("/verify_file1").route(web::get().to(verify_file_1));
    let verify_file_2 = web::resource("/verify_file2").route(web::get().to(verify_file_2));
    let verify_file_3 = web::resource("/verify_file3").route(web::get().to(verify_file_3));
    config.service(signin).service(verify_file_1).service(verify_file_2).service(verify_file_3);
}