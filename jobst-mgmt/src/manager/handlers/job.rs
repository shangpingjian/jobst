use uuid;
use actix_web::{
    App, Either,
    error,
    Error, get, http::{
        header::{self, ContentType},
        Method, StatusCode,
    }, HttpRequest, HttpResponse, HttpServer, middleware, Responder, Result, web,
};
use jobst_common::job::{Job, JobType, State, ExecStatus};
use jobst_common::adapter;


pub async fn get_jobs(req: HttpRequest) -> HttpResponse {

    let ap = adapter::new(adapter::AdapterType::Etcd);
    let r = ap.get_job_list().await.unwrap();
    let response_body = serde_json::to_string(&r).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(response_body)
}

pub async fn add_job(req: HttpRequest, info: web::Json<Job>) -> Result<HttpResponse, Error> {
    let ap = adapter::new(adapter::AdapterType::Etcd);
    let job = Job {
        job_id: uuid::Uuid::new_v4().to_string(),
        job_name: info.job_name.to_string(),
        job_type: info.job_type.clone(),
        state: State::Valid,
        exec_status: ExecStatus::Pending,
    };
    let r = ap.create_job(job).await;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_job_detail(req: HttpRequest, job_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let ap =adapter ::new(adapter::AdapterType::Etcd);
    let r = ap.get_job_detail(job_id.to_string()).await.unwrap();
    let response_body = serde_json::to_string(&r).unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(response_body))
}

pub async fn remove_job(req: HttpRequest, job_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let ap = adapter::new(adapter::AdapterType::Etcd);
    let r = ap.delete_job(job_id.to_string()).await.unwrap();

    Ok(HttpResponse::Ok().finish())
}
