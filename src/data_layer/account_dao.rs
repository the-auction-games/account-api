use super::account_entity::AccountEntity;
use rocket::async_trait;

#[async_trait]
pub trait AccountDao {
    async fn get_accounts(&self) -> Vec<AccountEntity>;

    async fn get_account(&self, id: String) -> Option<AccountEntity>;

    async fn validate_account(&self, email: String, password: String) -> Option<AccountEntity>;

    async fn create_account(&self, account: AccountEntity) -> bool;

    async fn update_account(&self, account: AccountEntity) -> bool;

    async fn delete_account(&self, id: String) -> bool;
}
