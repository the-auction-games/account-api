use super::CredentialsModel;
use super::AccountModel;

pub trait AccountService {

    fn get_accounts(&self) -> Vec<AccountModel>;

    fn validate(&self, credentials: CredentialsModel) -> Option<AccountModel>;

    fn create_account(&self, account: AccountModel) -> bool;

    fn update_account(&self, account: AccountModel) -> bool;

    fn delete_account(&self, id: String) -> bool;
}