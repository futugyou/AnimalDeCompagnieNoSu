use crate::model::report::rescue_classification::RescueClassificationRequest;
use crate::service::reportservice::*;
use actix_web::{web::Query, HttpRequest, HttpResponse};

pub async fn get_rescue_classification(
    Query(classic): Query<RescueClassificationRequest>,
    _req: HttpRequest,
) -> HttpResponse {
    let service = ReportService::new().await;
    let result = service.get_rescue_classification(classic).await;
    HttpResponse::Ok().json(result)
}
