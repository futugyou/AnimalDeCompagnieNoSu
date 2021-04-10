use entity::animaeventlentity::AnimalEventEntity;
use tool::getutcnowwithformat;

use crate::animalevent::animaleventmodel::{AnimalEventAddRequest, AnimalEventSearchResponse};

impl From<AnimalEventAddRequest> for AnimalEventEntity {
    fn from(request: AnimalEventAddRequest) -> Self {
        let mut event_time = getutcnowwithformat();
        if let Some(data) = request.event_time {
            event_time = data;
        }
        Self {
            id: "".to_owned(),
            animalid: request.animalid,
            event: request.event,
            event_type: request.event_type,
            event_time: Some(event_time),
        }
    }
}

impl From<AnimalEventEntity> for AnimalEventSearchResponse {
    fn from(entity: AnimalEventEntity) -> Self {
        Self {
            event: entity.event,
            event_time: entity.event_time,
        }
    }
}
