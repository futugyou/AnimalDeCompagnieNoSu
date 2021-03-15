impl From<crate::graphql::animalmodel::AnimalSearchRequest>
    for crate::model::animal::animalsearchmodel::AnimalSearchRequest
{
    fn from(request: crate::graphql::animalmodel::AnimalSearchRequest) -> Self {
        Self {
            name: request.name,
            animal_type: request.animal_type,
        }
    }
}

impl From<crate::model::animal::animalsearchmodel::AnimalSearchResponse>
    for crate::graphql::animalmodel::AnimalSearchResponse
{
    fn from(res: crate::model::animal::animalsearchmodel::AnimalSearchResponse) -> Self {
        Self {
            idcard: res.idcard,
            id: res.id,
            name: res.name,
            birthday: res.birthday,
            sub_type: res.sub_type,
            animal_type: res.animal_type,
            avatar: res.avatar,
            photoes: res.photoes,
        }
    }
}
impl From<crate::graphql::animalmodel::AnimalInsertRequest>
    for crate::model::animal::animalinsertmodel::AnimalInsertRequest
{
    fn from(req: crate::graphql::animalmodel::AnimalInsertRequest) -> Self {
        Self {
            name: req.name,
            birthday: req.birthday,
            sub_type: req.sub_type,
            animal_type: req.animal_type,
            avatar: req.avatar,
            photoes: req.photoes,
            rescue_date: req.rescue_date,
        }
    }
}

impl From<crate::model::animal::animalinsertmodel::AnimalInsertResponse>
    for crate::graphql::animalmodel::AnimalInsertResponse
{
    fn from(res: crate::model::animal::animalinsertmodel::AnimalInsertResponse) -> Self {
        Self {
            idcard: res.idcard,
            id: res.id,
            name: res.name,
            birthday: res.birthday,
            sub_type: res.sub_type,
            animal_type: res.animal_type,
            avatar: res.avatar,
            photoes: res.photoes,
        }
    }
}

impl From<crate::graphql::animalmodel::AnimalUpdateRequest>
    for crate::model::animal::animalupdatemodel::AnimalUpdateRequest
{
    fn from(req: crate::graphql::animalmodel::AnimalUpdateRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            birthday: req.birthday,
            sub_type: req.sub_type,
            animal_type: req.animal_type,
            avatar: req.avatar,
            photoes: req.photoes,
            rescue_date: req.rescue_date,
        }
    }
}

impl From<crate::model::animal::animalupdatemodel::AnimalUpdateResponse>
    for crate::graphql::animalmodel::AnimalUpdateResponse
{
    fn from(res: crate::model::animal::animalupdatemodel::AnimalUpdateResponse) -> Self {
        Self {
            idcard: res.idcard,
            id: res.id,
            name: res.name,
            birthday: res.birthday,
            sub_type: res.sub_type,
            animal_type: res.animal_type,
            avatar: res.avatar,
            photoes: res.photoes,
            rescue_date: res.rescue_date,
        }
    }
}
