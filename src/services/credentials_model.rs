use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CredentialsModel {
    pub email: String,
    pub password: String,
}