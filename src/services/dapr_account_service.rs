use super::account_service::AccountService;
use super::account_model::AccountModel;
use super::credentials_model::CredentialsModel;
use crate::data_layer::{AccountEntity, AccountDao};

pub struct DaprAccountService {
    account_dao: Box<dyn AccountDao>,
}

impl DaprAccountService {
    pub fn new(account_dao: Box<dyn AccountDao>) -> Self {
        DaprAccountService { account_dao }
    }
}

impl AccountService for DaprAccountService {
    //  todo: make sure the methods are actually correct

    fn get_accounts(&self) -> Vec<AccountModel> {
        let entities = self.account_dao.get_accounts();

        let mut models = Vec::new();

        for entity in entities {
            models.push(AccountModel::from_entity(entity));
        }

        models
    }

    fn validate(&self, credentials: CredentialsModel) -> Option<AccountModel> {
        let entity = self.account_dao.get_account(credentials.email, credentials.password);

        match entity {
            Some(account_entity) => Some(AccountModel::from_entity(account_entity)),
            None => None,
        }
    }

    fn create_account(&self, account: AccountModel) -> bool {
        self.account_dao.create_account(AccountEntity::from_model(account))
    }

    fn update_account(&self, account: AccountModel) -> bool {
        self.account_dao.update_account(AccountEntity::from_model(account))
    }

    fn delete_account(&self, id: String) -> bool {
        self.account_dao.delete_account(id)
    }
}