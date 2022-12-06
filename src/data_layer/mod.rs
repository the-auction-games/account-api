mod account_entity;
mod account_dao;
mod dapr_account_dao;

pub use account_entity::AccountEntity;
pub use account_dao::AccountDao;
use dapr_account_dao::DaprAccountDao;

// get account dao
pub fn get_account_dao() -> Box<dyn AccountDao> {
    Box::new(DaprAccountDao {})
}

// BASICALLY DEPENDENCY INJECT ALL METHODS WE WANT FOR PUBLIC USE HERE