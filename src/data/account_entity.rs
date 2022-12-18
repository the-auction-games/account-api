use crate::services::AccountModel;
use rocket::serde::{Deserialize, Serialize};

/// The Account Entity.
///
/// This entity is used to directly store account data
/// in the database.
///
/// # Fields
/// * `id` - The id of the account
/// * `name` - The name of the account
/// * `email` - The email of the account
/// * `password` - The password of the account
///
/// # Methods
/// * `from_model` - Creates a new account entity from an account model
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

/// The account entity implementation.
impl AccountEntity {
    /// Creates a new account entity from an account model.
    ///
    /// # Arguments
    /// * `account` - The account model to convert
    ///
    /// # Returns
    /// The new account entity
    pub fn from_model(account: &AccountModel) -> Self {
        AccountEntity {
            id: account.id.clone(),
            name: account.name.clone(),
            email: account.email.clone(),
            password: account.password.clone(),
        }
    }
}
