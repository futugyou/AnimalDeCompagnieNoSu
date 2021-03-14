use crate::model::report::rescueh_classification::RescuehClassificationRequest;
use crate::service::reportservice::*;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn get_rescue_classification(
    item: Option<web::Json<RescuehClassificationRequest>>,
    _req: HttpRequest,
) -> HttpResponse {
    let mut paramter = RescuehClassificationRequest::default();
    match item {
        Some(r) => paramter = r.into_inner(),
        None => (),
    }
    let service = ReportService::new().await;
    let result = service.get_rescue_classification(paramter).await;
    HttpResponse::Ok().json(result)
}
