use entity::animaeventlentity::AnimalEventEntity;

use crate::animalevent::animaleventmodel::{AnimalEventAddRequest, AnimalEventSearchResponse};

impl From<AnimalEventAddRequest> for AnimalEventEntity {
    fn from(request: AnimalEventAddRequest) -> Self {
        Self {
            id: "".to_owned(),
            animalid: request.animalid,
            event: request.event,
            event_type: request.event_type,
            event_time: request.event_time,
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
