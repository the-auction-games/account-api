use super::AccountDetails;
use super::AccountModel;
use super::CredentialsModel;
use rocket::async_trait;

/// The account service.
///
/// This trait defines the interface for the account service.
///
/// # Methods
/// * `get_accounts` - Gets all accounts
/// * `get_account_by_id` - Gets an account by id
/// * `get_account_by_email` - Gets an account by email
/// * `validate_account` - Validates an account
/// * `create_account` - Creates an account
/// * `update_account` - Updates an account
/// * `delete_account` - Deletes an account
#[async_trait]
pub trait AccountService {
    /// Gets all accounts.
    ///
    /// # Returns
    /// The list of accounts
    async fn get_accounts(&self) -> Vec<AccountDetails>;

    /// Gets an account by id.
    ///
    /// # Arguments
    /// * `id` - The id of the account
    ///
    /// # Returns
    /// The account details
    async fn get_account_by_id(&self, id: String) -> Option<AccountDetails>;

    /// Gets an account by email.
    ///
    /// # Arguments
    /// * `email` - The email of the account
    ///
    /// # Returns
    /// The account details
    async fn get_account_by_email(&self, email: String) -> Option<AccountDetails>;

    /// Validates an account.
    ///
    /// # Arguments
    /// * `credentials` - The credentials of the account
    ///
    /// # Returns
    /// The account details
    async fn validate_account(&self, credentials: CredentialsModel) -> Option<AccountDetails>;

    /// Creates an account.
    ///
    /// # Arguments
    /// * `account` - The account to create
    ///
    /// # Returns
    /// `true` if the account was created, otherwise `false`
    async fn create_account(&self, account: AccountModel) -> bool;

    /// Updates an account.
    ///
    /// # Arguments
    /// * `account` - The account to update
    ///
    /// # Returns
    /// `true` if the account was updated, otherwise `false`
    async fn update_account(&self, account: AccountModel) -> bool;

    /// Deletes an account.
    ///
    /// # Arguments
    /// * `id` - The id of the account to delete
    ///
    /// # Returns
    /// `true` if the account was deleted, otherwise `false`
    async fn delete_account(&self, id: String) -> bool;
}
