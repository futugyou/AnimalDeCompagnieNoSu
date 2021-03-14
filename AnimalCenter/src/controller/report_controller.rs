use crate::model::report::rescueh_classification::RescuehClassificationRequest;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn get_rescue_classification(
    item: Option<web::Json<RescuehClassificationRequest>>,
    _req: HttpRequest,
) -> HttpResponse {
    println!("{:#?}", item);
    HttpResponse::Ok().json("")
}
