mod account_model;
mod credentials_model;
mod account_service;
mod dapr_account_service;

use crate::data_layer::get_account_dao;


pub use account_model::AccountModel;
pub use credentials_model::CredentialsModel;
pub use account_service::AccountService;

pub fn get_account_service() -> Box<dyn AccountService> {
    Box::new(dapr_account_service::DaprAccountService::new(get_account_dao()))
}
