use super::account_dao::AccountDao;
use super::account_entity::AccountEntity;

pub struct DaprAccountDao {}

impl DaprAccountDao {
    pub fn new() -> Self {
        DaprAccountDao {}
    }
}

impl AccountDao for DaprAccountDao {
    // TODO: LOAD UP REQUESTS FROM DAPR
    fn get_accounts(&self) -> Vec<AccountEntity> {
        vec![AccountEntity {
            id: String::from("1"),
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
        }, AccountEntity {
            id: String::from("2"),
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
        }, AccountEntity {
            id: String::from("3"),
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
        }]
    }

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
