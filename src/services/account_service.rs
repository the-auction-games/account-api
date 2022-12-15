use rocket::async_trait;
use super::AccountDetails;
use super::AccountModel;
use super::CredentialsModel;

#[async_trait]
pub trait AccountService {
    async fn get_accounts(&self) -> Vec<AccountDetails>;

    async fn get_account(&self, id: String) -> Option<AccountDetails>;

    async fn validate_account(&self, credentials: CredentialsModel) -> Option<AccountDetails>;

    async fn create_account(&self, account: AccountModel) -> bool;

    async fn update_account(&self, account: AccountModel) -> bool;

    async fn delete_account(&self, id: String) -> bool;
}
