use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use super::{
    data_manager::DataManager,
    execution_engine::ExecutionEngine,
    archive_system::ArchiveSystem,
    result_processor::ResultProcessor,
    models::{AnalysisRequest, JobStatus, AppState},  
};

use crate::models::{AnalysisRequest, JobStatus, AppState};

pub async fn submit_analysis(
    data: web::Json<AnalysisRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let job_id = Uuid::new_v4().to_string();

    if let Err(e) = state.archive_system.store_submission(&job_id, &data) {
        return HttpResponse::InternalServerError().json(format!("Failed to archive submission: {}", e));
    }

    if let Err(e) = state.data_manager.prepare_data(&data.cohort_id, &data.data_type).await {
        return HttpResponse::InternalServerError().json(format!("Failed to prepare data: {}", e));
    }

    if let Err(e) = state.execution_engine.lock().unwrap().queue_job(&job_id, &data) {
        return HttpResponse::InternalServerError().json(format!("Failed to queue job: {}", e));
    }

    state.jobs.lock().unwrap().insert(job_id.clone(), JobStatus {
        status: "Queued".to_string(),
        result_url: None,
    });

    HttpResponse::Ok().json(job_id)
}

pub async fn get_job_status(
    job_id: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let jobs = state.jobs.lock().unwrap();
    match jobs.get(&job_id) {
        Some(status) => HttpResponse::Ok().json(status),
        None => HttpResponse::NotFound().json("Job not found"),
    }
}

pub async fn get_result(
    job_id: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let jobs = state.jobs.lock().unwrap();
    match jobs.get(&job_id) {
        Some(status) if status.status == "Completed" => {
            match state.result_processor.get_processed_result(&job_id) {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(e) => HttpResponse::InternalServerError().json(format!("Failed to retrieve result: {}", e)),
            }
        },
        Some(_) => HttpResponse::BadRequest().json("Job not completed yet"),
        None => HttpResponse::NotFound().json("Job not found"),
    }
}

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/submit", web::post().to(submit_analysis))
            .route("/status/{job_id}", web::get().to(get_job_status))
            .route("/result/{job_id}", web::get().to(get_result))
    );
}