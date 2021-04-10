use actix_web::{web, Error, HttpRequest, HttpResponse};
use model::animalevent::animaleventmodel::AnimalEventAddRequest;
use service::animaleventservice::{AnimalEventService, IAnimalEventService};

pub async fn get(web::Path((id,)): web::Path<(String,)>, _req: HttpRequest) -> HttpResponse {
    let service = AnimalEventService::new().await;
    let response = service.getanimaleventlist(id).await;
    HttpResponse::Ok().json(response)
}

pub async fn post(
    web::Path((id,)): web::Path<(String,)>,
    item: Option<web::Json<AnimalEventAddRequest>>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let service = AnimalEventService::new().await;
    let mut rep = AnimalEventAddRequest::default();
    match item {
        Some(i) => {
            rep = i.into_inner();
            rep.animalid = id;
        }
        None => {}
    };
    let response = service.addnewevent(rep).await;
    Ok(HttpResponse::Ok().json(response))
}
