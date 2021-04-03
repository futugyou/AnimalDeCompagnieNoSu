impl From<crate::animal::animalmodel::AnimalSearchRequest>
    for crate::animal::animalsearchmodel::AnimalSearchRequest
{
    fn from(request: crate::animal::animalmodel::AnimalSearchRequest) -> Self {
        Self {
            name: request.name,
            animal_type: request.animal_type,
            paging: None,
        }
    }
}

impl From<crate::animal::animalsearchmodel::AnimalSearchResponse>
    for crate::animal::animalmodel::AnimalSearchResponse
{
    fn from(res: crate::animal::animalsearchmodel::AnimalSearchResponse) -> Self {
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
impl From<crate::animal::animalmodel::AnimalInsertRequest>
    for crate::animal::animalinsertmodel::AnimalInsertRequest
{
    fn from(req: crate::animal::animalmodel::AnimalInsertRequest) -> Self {
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

impl From<crate::animal::animalinsertmodel::AnimalInsertResponse>
    for crate::animal::animalmodel::AnimalInsertResponse
{
    fn from(res: crate::animal::animalinsertmodel::AnimalInsertResponse) -> Self {
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

impl From<crate::animal::animalmodel::AnimalUpdateRequest>
    for crate::animal::animalupdatemodel::AnimalUpdateRequest
{
    fn from(req: crate::animal::animalmodel::AnimalUpdateRequest) -> Self {
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

impl From<crate::animal::animalupdatemodel::AnimalUpdateResponse>
    for crate::animal::animalmodel::AnimalUpdateResponse
{
    fn from(res: crate::animal::animalupdatemodel::AnimalUpdateResponse) -> Self {
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
