use actix_web::web;

use crate::manager::handlers::job;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/job")
            .service(
                web::resource("")
                    .route(web::get().to(job::get_jobs))
                    .route(web::post().to(job::add_job)),
            ).service(
            web::scope("/{job_id}")
                .service(
                    web::resource("")
                        .route(web::get().to(job::get_job_detail))
                        .route(web::delete().to(job::remove_job)),
                )
        )
    );
}
