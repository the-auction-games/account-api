mod account_entity;
mod account_dao;
mod dapr_account_dao;

pub use account_entity::AccountEntity;
pub use account_dao::AccountDao;

pub fn get_account_dao() -> Box<dyn AccountDao> {
    Box::new(dapr_account_dao::DaprAccountDao::new())
}