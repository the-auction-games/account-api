use super::account_entity::AccountEntity;

pub trait AccountDao {
    
    fn get_account(&self, email: String, password: String) -> Option<AccountEntity>;

    fn create_account(&self, account: AccountEntity) -> bool;

    fn update_account(&self, account: AccountEntity) -> bool;

    fn delete_account(&self, id: String) -> bool;
}