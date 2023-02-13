use actix_web::{web, Error, HttpResponse};
use crate::job::Job;


pub async fn get_jobs(_query: web::Query<Option<Job>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_job(_new_part: web::Json<Job>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}