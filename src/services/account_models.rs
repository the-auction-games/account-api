use rocket::serde::{Deserialize, Serialize};

use crate::data_layer::AccountEntity;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl AccountModel {
    pub fn _from_entity(entity: &AccountEntity) -> Self {
        AccountModel {
            id: entity.id.clone(),
            name: entity.name.clone(),
            email: entity.email.clone(),
            password: entity.password.clone(),
        }
    }
}

// TODO: REMOVE
// #[async_trait]
// impl<'a> FromData<'a> for AccountModel {
//     type Error = String;

//     async fn from_data(req: &'a Request<'_>, data: Data<'a>) -> Outcome<'a, Self> {

//         let x = data.open(Limits::JSON).into_string().await;
//         x.
//     }
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountDetails {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl AccountDetails {
    pub fn from_entity(entity: &AccountEntity) -> Self {
        AccountDetails {
            id: entity.id.clone(),
            name: entity.name.clone(),
            email: entity.email.clone(),
        }
    }

    pub fn _from_model(model: &AccountModel) -> Self {
        AccountDetails {
            id: model.id.clone(),
            name: model.name.clone(),
            email: model.email.clone(),
        }
    }
}
