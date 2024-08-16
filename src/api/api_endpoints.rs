use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;

use crate::api::app_state::AppState;
use crate::api::shared_models::{CodeSubmission, SyntheticDataSetup, AnalysisResult};

fn validate_server_app_id(req: &HttpRequest, app_state: &web::Data<AppState>) -> Result<(), HttpResponse> {
    let server_app_id = req.headers().get("Server-App-ID")
        .and_then(|id| id.to_str().ok())
        .ok_or_else(|| HttpResponse::BadRequest().json(json!({"error": "Missing Server-App-ID header"})))?;

    if !app_state.data_manager.validate_server_app_id(server_app_id) {
        return Err(HttpResponse::Unauthorized().json(json!({"error": "Invalid Server-App-ID"})));
    }

    Ok(())
}

pub async fn submit_code(
    req: HttpRequest,
    submission: web::Json<CodeSubmission>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = validate_server_app_id(&req, &app_state) {
        return response;
    }

    let job_id = app_state.code_storage.store_submission(submission.into_inner());
    HttpResponse::Ok().json(json!({ "job_id": job_id }))
}

pub async fn get_code_submission(
    req: HttpRequest,
    job_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = validate_server_app_id(&req, &app_state) {
        return response;
    }

    match app_state.code_storage.get_submission(&job_id) {
        Some(submission) => HttpResponse::Ok().json(submission),
        None => HttpResponse::NotFound().json(json!({ "error": "Code submission not found" })),
    }
}

pub async fn get_cohort_summary(
    req: HttpRequest,
    cohort_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = validate_server_app_id(&req, &app_state) {
        return response;
    }

    match app_state.cohort_manager.get_cohort_summary(&cohort_id) {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to retrieve cohort info: {}", e)
        }))
    }
}

pub async fn setup_synthetic_data(
    req: HttpRequest,
    setup: web::Json<SyntheticDataSetup>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = validate_server_app_id(&req, &app_state) {
        return response;
    }

    match app_state.synthetic_data_generator.setup_synthetic_data(&setup) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to setup synthetic data: {}", e)
        }))
    }
}

pub async fn submit_analysis_result(
    req: HttpRequest,
    result: web::Json<AnalysisResult>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = validate_server_app_id(&req, &app_state) {
        return response;
    }

    match app_state.data_manager.store_analysis_result(&result) {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "result stored successfully" })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to store analysis result: {}", e)
        }))
    }
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/submit_code", web::post().to(submit_code))
            .route("/code/{job_id}", web::get().to(get_code_submission))
            .route("/cohort/{cohort_id}", web::get().to(get_cohort_summary))
            .route("/synthetic/setup", web::post().to(setup_synthetic_data))
            .route("/result", web::post().to(submit_analysis_result))
    );
}