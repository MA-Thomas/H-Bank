use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::api::shared_models::{CohortInfo, SyntheticDataSetup, CodeSubmission, AnalysisResult};
use crate::api::app_state::AppState;
use crate::api::data_manager::DataManagerTrait;
use crate::api::execution_engine::ExecutionEngineTrait;

use actix_web::http::header::{HeaderName, HeaderValue, TryIntoHeaderValue};
use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;


/*
This file handles communication (receiving requests) 
from HBroker_ServerApp/.../hbank_communication.rs
*/

pub struct ServerAppIdHeader(pub String);

impl actix_web::FromRequest for ServerAppIdHeader {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("Server-App-ID") {
            Some(header) => match header.to_str() {
                Ok(v) => std::future::ready(Ok(ServerAppIdHeader(v.to_string()))),
                Err(_) => std::future::ready(Err(ErrorBadRequest("Invalid Server-App-ID header"))),
            },
            None => std::future::ready(Err(ErrorBadRequest("Server-App-ID header is missing"))),
        }
    }
}

pub async fn get_cohort_info(
    cohort_id: web::Path<String>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.data_manager.get_cohort_info(&cohort_id, &server_app_id.0).await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to retrieve cohort info: {}", e)
        }))
    }
}

pub async fn setup_synthetic_data_dir(
    info: web::Json<SyntheticDataSetup>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.data_manager.setup_synthetic_data(&info, &server_app_id.0).await {
        Ok(setup) => HttpResponse::Ok().json(setup),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to setup synthetic data directory: {}", e)
        }))
    }
}

pub async fn request_synthetic_cohort_data(
    info: web::Json<SyntheticDataSetup>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.data_manager.generate_synthetic_data(&info, &server_app_id.0).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "data_generated"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to generate synthetic data: {}", e)
        }))
    }
}

pub async fn run_on_synthetic_data(
    submission: web::Json<CodeSubmission>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    let job_id = Uuid::new_v4().to_string();
    
    if let Err(e) = state.archive_system.store_submission(&job_id, &submission) {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to archive submission: {}", e)
        }));
    }

    let mut execution_engine = state.execution_engine.lock().unwrap();
    if let Err(e) = execution_engine.queue_job(&job_id, &submission).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to queue job: {}", e)
        }));
    }

    HttpResponse::Accepted().json(json!({
        "job_id": job_id,
        "status": "queued"
    }))
}

pub async fn run_on_sensitive_data(
    submission: web::Json<CodeSubmission>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    let job_id = Uuid::new_v4().to_string();
    
    if let Err(e) = state.archive_system.store_submission(&job_id, &submission) {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to archive submission: {}", e)
        }));
    }

    let mut execution_engine = state.execution_engine.lock().unwrap();
    if let Err(e) = execution_engine.queue_job(&job_id, &submission).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to queue job: {}", e)
        }));
    }

    HttpResponse::Accepted().json(json!({
        "job_id": job_id,
        "status": "queued"
    }))
}

pub async fn get_job_status(
    job_id: web::Path<String>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    let execution_engine = state.execution_engine.lock().unwrap();
    match execution_engine.get_job_status(&job_id).await {
        Some(status) => HttpResponse::Ok().json(status),
        None => HttpResponse::NotFound().json(json!({"error": "Job not found"})),
    }
}

pub async fn get_job_result(
    job_id: web::Path<String>,
    server_app_id: ServerAppIdHeader,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.result_processor.get_processed_result(&job_id) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to retrieve result: {}", e)
        })),
    }
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/cohort/{cohort_id}", web::get().to(get_cohort_info))
            .route("/synthetic/setup", web::post().to(setup_synthetic_data_dir))
            .route("/synthetic/data", web::post().to(request_synthetic_cohort_data))
            .route("/run/synthetic", web::post().to(run_on_synthetic_data))
            .route("/run/sensitive", web::post().to(run_on_sensitive_data))
            .route("/job/{job_id}/status", web::get().to(get_job_status))
            .route("/job/{job_id}/result", web::get().to(get_job_result))
    );
}