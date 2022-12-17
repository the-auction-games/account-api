use rocket::serde::{Deserialize, Serialize};

/// The credentials model.
///
/// This model is used to transfer credentials data between
/// the presentation layer and the service layer.
///
/// # Fields
/// * `email` - The email of the account
/// * `password` - The password of the account
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CredentialsModel {
    pub email: String,
    pub password: String,
}
