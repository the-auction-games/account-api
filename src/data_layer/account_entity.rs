use crate::services::AccountModel;

///
/// The Account Entity.
///
/// This entity is used to directly store account data
/// in the database.
///
pub struct AccountEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl AccountEntity {
    pub fn from_model(account: AccountModel) -> Self {
        AccountEntity {
            id: account.id,
            name: account.name,
            email: account.credentials.email,
            password: account.credentials.password,
        }
    }
}
