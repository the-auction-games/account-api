use rocket::serde::{Deserialize, Serialize};

use crate::data::AccountEntity;

/// The Account Model.
///
/// This model is used to transfer account data between
/// the service layer and the data layer.
///
/// # Fields
/// * `id` - The id of the account
/// * `name` - The name of the account
/// * `email` - The email of the account
/// * `password` - The password of the account
///
/// # Methods
/// * `from_entity` - Creates a new account model from an account entity
///
/// # Note
/// This model is not used in the presentation layer. It is only used
/// in the service layer.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

///
/// The account model implementation.
///
impl AccountModel {
    ///
    /// Creates a new account model from an account entity.
    ///
    /// # Arguments
    /// * `entity` - The account entity to convert
    ///
    /// # Returns
    /// The new account model
    ///
    pub fn _from_entity(entity: &AccountEntity) -> Self {
        AccountModel {
            id: entity.id.clone(),
            name: entity.name.clone(),
            email: entity.email.clone(),
            password: entity.password.clone(),
        }
    }
}

/// The Account Details.
///
/// This model is used to transfer account data between
/// the service layer and the presentation layer.
///
/// # Fields
/// * `id` - The id of the account
/// * `name` - The name of the account
/// * `email` - The email of the account
///
/// # Methods
/// * `from_entity` - Creates a new account details from an account entity
/// * `from_model` - Creates a new account details from an account model
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountDetails {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// The account details implementation.
impl AccountDetails {
    /// Creates a new account details from an account entity.
    ///
    /// # Arguments
    /// * `entity` - The account entity to convert
    ///
    /// # Returns
    /// The new account details
    pub fn from_entity(entity: &AccountEntity) -> Self {
        AccountDetails {
            id: entity.id.clone(),
            name: entity.name.clone(),
            email: entity.email.clone(),
        }
    }

    /// Creates a new account details from an account model.
    ///
    /// # Arguments
    /// * `model` - The account model to convert
    ///
    /// # Returns
    /// The new account details
    pub fn _from_model(model: &AccountModel) -> Self {
        AccountDetails {
            id: model.id.clone(),
            name: model.name.clone(),
            email: model.email.clone(),
        }
    }
}
