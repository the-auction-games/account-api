use super::account_entity::AccountEntity;
use super::account_dao::AccountDao;

// TODO: LOAD UP REQUESTS FROM DAPR


pub struct DaprAccountDao {

}

impl AccountDao for DaprAccountDao {

    fn get_account(&self, email: String, password: String) -> Option<AccountEntity> {

        None
    }

    fn create_account(&self, account: AccountEntity) -> bool {
        false
    }

    fn update_account(&self, account: AccountEntity) -> bool {
        false
    }

    fn delete_account(&self, id: String) -> bool {
        false
    }
}