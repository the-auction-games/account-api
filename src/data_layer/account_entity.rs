use rocket::serde::{Deserialize, Serialize};
use crate::services::AccountModel;

///
/// The Account Entity.
///
/// This entity is used to directly store account data
/// in the database.
///
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl AccountEntity {
    pub fn from_model(account: &AccountModel) -> Self {
        AccountEntity {
            id: account.id.clone(),
            name: account.name.clone(),
            email: account.email.clone(),
            password: account.password.clone(),
        }
    }
}
