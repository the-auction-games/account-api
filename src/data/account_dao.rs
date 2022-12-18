use super::account_entity::AccountEntity;
use rocket::async_trait;

/// The Account Data Access Object.
///
/// This data access object is used to access the account data.
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
pub trait AccountDao {
    /// Gets all accounts.
    ///
    /// # Returns
    /// The list of accounts
    async fn get_accounts(&self) -> Vec<AccountEntity>;

    /// Gets an account by id.
    ///
    /// # Arguments
    /// * `id` - The id of the account
    ///
    /// # Returns
    /// The account entity
    async fn get_account_by_id(&self, id: String) -> Option<AccountEntity>;

    /// Gets an account by email.
    ///
    /// # Arguments
    /// * `email` - The email of the account
    ///
    /// # Returns
    /// The account entity
    async fn get_account_by_email(&self, email: String) -> Option<AccountEntity>;

    /// Validates an account.
    ///
    /// # Arguments
    /// * `email` - The email of the account
    /// * `password` - The password of the account
    ///
    /// # Returns
    /// The account entity
    async fn validate_account(&self, email: String, password: String) -> Option<AccountEntity>;

    /// Creates an account.
    ///
    /// # Arguments
    /// * `account` - The account to create
    ///
    /// # Returns
    /// `true` if the account was created, otherwise `false`
    async fn create_account(&self, account: AccountEntity) -> bool;

    /// Updates an account.
    ///
    /// # Arguments
    /// * `account` - The account to update
    ///
    /// # Returns
    /// `true` if the account was updated, otherwise `false`
    async fn update_account(&self, account: AccountEntity) -> bool;

    /// Deletes an account.
    ///
    /// # Arguments
    /// * `id` - The id of the account
    ///
    /// # Returns
    /// `true` if the account was deleted, otherwise `false`
    async fn delete_account(&self, id: String) -> bool;
}
