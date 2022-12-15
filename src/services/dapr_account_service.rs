use rocket::async_trait;
use super::account_models::{AccountDetails, AccountModel};
use super::account_service::AccountService;
use super::credentials_model::CredentialsModel;
use crate::data_layer::{AccountDao, AccountEntity, DaprAccountDao};

pub struct DaprAccountService {
    account_dao: DaprAccountDao,
}

impl DaprAccountService {
    pub fn new(account_dao: DaprAccountDao) -> Self {
        DaprAccountService { account_dao }
    }
}

#[async_trait]
impl AccountService for DaprAccountService {
    async fn get_accounts(&self) -> Vec<AccountDetails> {
        self
            .account_dao
            .get_accounts()
            .await
            .iter()
            .map(|entity| AccountDetails::from_entity(entity))
            .collect()
    }
    
    async fn get_account(&self, id: String) -> Option<AccountDetails> {
        let entity: Option<AccountEntity> = self.account_dao.get_account(id).await;

        match entity {
            Some(found_entity) => Some(AccountDetails::from_entity(&found_entity)),
            None => None,
        }
    }

    async fn validate_account(&self, credentials: CredentialsModel) -> Option<AccountDetails> {
        let entity: Option<AccountEntity> = self
            .account_dao
            .validate_account(credentials.email, credentials.password)
            .await;

        match entity {
            Some(found_entity) => Some(AccountDetails::from_entity(&found_entity)),
            None => None,
        }
    }

    async fn create_account(&self, account: AccountModel) -> bool {
        self.account_dao
            .create_account(AccountEntity::from_model(&account))
            .await
    }

    async fn update_account(&self, account: AccountModel) -> bool {
        self.account_dao
            .update_account(AccountEntity::from_model(&account))
            .await
    }

    async fn delete_account(&self, id: String) -> bool {
        self.account_dao.delete_account(id).await
    }
}
