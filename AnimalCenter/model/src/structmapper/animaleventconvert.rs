use entity::animaeventlentity::AnimalEventEntity;

use crate::animalevent::animaleventmodel::AnimalEventAddRequest;

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
