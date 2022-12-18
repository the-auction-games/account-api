use super::account_models::{AccountDetails, AccountModel};
use super::account_service::AccountService;
use super::credentials_model::CredentialsModel;
use crate::data::{AccountDao, AccountEntity, DaprAccountDao};
use rocket::async_trait;

/// The Dapr Account Service.
///
/// This service is used to access the account data.
///
/// # Fields
/// * `account_dao` - The account data access object
///
/// # Methods
/// * `new` - Creates a new account service
/// * `to_account_details` - Converts an account entity to an account details
/// * `get_accounts` - Gets all accounts
/// * `get_account_by_id` - Gets an account by id
/// * `get_account_by_email` - Gets an account by email
/// * `create_account` - Creates an account
/// * `update_account` - Updates an account
/// * `delete_account` - Deletes an account
/// * `validate_account` - Validates an account
///
/// # Traits
/// * `AccountService` - The account service trait
pub struct DaprAccountService {
    account_dao: DaprAccountDao,
}

/// The Dapr Account Service implementation.
impl DaprAccountService {
    /// Creates a new account service.
    ///
    /// # Arguments
    /// * `account_dao` - The account data access object
    ///
    /// # Returns
    /// The new account service
    pub fn new(account_dao: DaprAccountDao) -> Self {
        DaprAccountService { account_dao }
    }

    /// Converts an account entity to an account details.
    ///
    /// # Arguments
    /// * `entity` - The account entity to convert
    ///
    /// # Returns
    /// The account details
    fn to_account_details(&self, entity: &Option<AccountEntity>) -> Option<AccountDetails> {
        match entity {
            Some(found_entity) => Some(AccountDetails::from_entity(&found_entity)),
            None => None,
        }
    }
}

/// The Account Service implementation.
#[async_trait]
impl AccountService for DaprAccountService {
    /// Gets all accounts.
    ///
    /// # Returns
    /// The list of accounts
    async fn get_accounts(&self) -> Vec<AccountDetails> {
        // Get all accounts and map to account details
        self.account_dao
            .get_accounts()
            .await
            .iter()
            .map(|entity| AccountDetails::from_entity(entity))
            .collect()
    }

    /// Gets an account by id.
    ///
    /// # Arguments
    /// * `id` - The id of the account
    ///
    /// # Returns
    /// The account details
    async fn get_account_by_id(&self, id: String) -> Option<AccountDetails> {
        // Get the account and map to account details
        let entity: Option<AccountEntity> = self.account_dao.get_account_by_id(id).await;
        self.to_account_details(&entity)
    }

    /// Gets an account by email.
    ///
    /// # Arguments
    /// * `email` - The email of the account
    ///
    /// # Returns
    /// The account details
    async fn get_account_by_email(&self, email: String) -> Option<AccountDetails> {
        // Get the account and map to account details
        let entity: Option<AccountEntity> = self.account_dao.get_account_by_email(email).await;
        self.to_account_details(&entity)
    }

    /// Validates an account.
    ///
    /// # Arguments
    /// * `credentials` - The credentials of the account
    ///
    /// # Returns
    /// The account details
    async fn validate_account(&self, credentials: CredentialsModel) -> Option<AccountDetails> {
        // Get the account with the given credentials and map to account details
        let entity: Option<AccountEntity> = self
            .account_dao
            .validate_account(credentials.email, credentials.password)
            .await;
        self.to_account_details(&entity)
    }

    /// Creates an account.
    ///
    /// # Arguments
    /// * `account` - The account to create
    ///
    /// # Returns
    /// True if the account was created, false otherwise
    async fn create_account(&self, account: AccountModel) -> bool {
        // Create the account
        self.account_dao
            .create_account(AccountEntity::from_model(&account))
            .await
    }

    /// Updates an account.
    ///
    /// # Arguments
    /// * `account` - The account to update
    ///
    /// # Returns
    /// True if the account was updated, false otherwise
    async fn update_account(&self, account: AccountModel) -> bool {
        // Update the account
        self.account_dao
            .update_account(AccountEntity::from_model(&account))
            .await
    }

    /// Deletes an account.
    ///
    /// # Arguments
    /// * `id` - The id of the account
    ///
    /// # Returns
    /// True if the account was deleted, false otherwise
    async fn delete_account(&self, id: String) -> bool {
        // Delete the account
        self.account_dao.delete_account(id).await
    }
}
