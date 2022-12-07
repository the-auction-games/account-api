use crate::data_layer::AccountEntity;
use super::CredentialsModel;

pub struct AccountModel {
    pub id: String,
    pub name: String,
    pub credentials: CredentialsModel,
}

impl AccountModel {
    pub fn from_entity(entity: AccountEntity) -> Self {
        AccountModel {
            id: entity.id,
            name: entity.name,
            credentials: CredentialsModel {
                email: entity.email,
                password: entity.password,
            },
        }
    }
}