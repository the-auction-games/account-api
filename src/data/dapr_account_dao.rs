use std::env;

use super::account_dao::AccountDao;
use super::account_entity::AccountEntity;
use pwhash::bcrypt;
use reqwest::ClientBuilder;
use rocket::{
    async_trait,
    serde::json::serde_json::json,
    serde::{Deserialize, Serialize},
};

/// Get the the sidecar port.
/// 
/// # Returns
/// The sidecar port
fn get_sidecar_port() -> String {
    match env::var("STATE_STORE_PORT") {
        Ok(val) => val,
        Err(_e) => "3500".to_string(),
    }
}

/// Get the state store name.
/// 
/// # Returns
/// The state store name
fn get_state_store_name() -> String {
    match env::var("STATE_STORE_NAME") {
        Ok(val) => val,
        Err(_e) => "account-db".to_string(),
    }
}

/// Get the dapr url.
/// 
/// # Returns
/// The dapr url
fn get_sidecar_url() -> String {
    format!(
        "http://localhost:{}/v1.0/state/{}",
        get_sidecar_port(),
        get_state_store_name()
    )
}

/// Get the dapr query url.
/// 
/// # Returns
/// The dapr query url
fn get_sidecar_query_url() -> String {
    format!(
        "http://localhost:{}/v1.0-alpha1/state/{}/query",
        get_sidecar_port(),
        get_state_store_name()
    )
}

/// The dapr results model maps the results from the dapr state store.
///
/// # Fields
/// * `results` - The results from the dapr state store
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct DaprResults {
    results: Vec<Entry>,
}

/// The dapr entry model maps the results keys from the dapr state store.
///
/// # Fields
/// * `data` - A singular account entity
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Entry {
    data: AccountEntity,
}

/// The dapr account dao.
///
/// This dao is used to access the dapr state store.
///
/// # Methods
/// * `new` - Creates a new dapr account dao
/// * `get_accounts` - Gets all accounts from the dapr state store
/// * `get_account_by_id` - Gets an account by id from the dapr state store
/// * `get_account_by_email` - Gets an account by email from the dapr state store
/// * `validate_account` - Validates an account in the dapr state store
/// * `create_account` - Creates an account in the dapr state store
/// * `update_account` - Updates an account in the dapr state store
/// * `delete_account` - Deletes an account in the dapr state store
///
/// # Traits
/// * `AccountDao` - The account dao trait
pub struct DaprAccountDao {}

/// The dapr account dao implementation.
impl DaprAccountDao {
    /// Creates a new dapr account dao.
    ///
    /// # Returns
    /// The new dapr account dao
    pub fn new() -> Self {
        DaprAccountDao {}
    }

    /// Hash a password using bcrypt.
    ///
    /// # Arguments
    /// * `password` - The password to hash
    ///
    /// # Returns
    /// The hashed password
    pub fn hash_password(&self, password: String) -> String {
        bcrypt::hash(password).unwrap()
    }

    /// Validate a password using bcrypt.
    ///
    /// # Arguments
    /// * `password` - The password to validate
    /// * `hash` - The password hash
    ///
    /// # Returns
    /// True if the password is valid
    pub fn validate_password(&self, password: String, hash: &str) -> bool {
        bcrypt::verify(password, hash)
    }

    /// Save an account to the dapr state store.
    ///
    /// # Arguments
    /// * `account` - The account to save
    ///
    /// # Returns
    /// True if the account was saved successfully
    pub async fn save_account(&self, account: AccountEntity) -> bool {
        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Hash the password in the account
        let hashed_account = AccountEntity {
            password: self.hash_password(account.password),
            ..account
        };

        // Post if account creation is successful
        client
            // Post to the url
            .post(get_sidecar_url())
            // Add body to the post request
            .body(
                json!(
                    [
                        {
                            "key": hashed_account.id,
                            "value": hashed_account,
                        },
                    ]
                )
                .to_string(),
            )
            // Send the request
            .send()
            .await
            .unwrap()
            // Check if the request was successful
            .status()
            .is_success()
    }
}

/// The dapr account dao implementation.
#[async_trait]
impl AccountDao for DaprAccountDao {
    /// Gets all accounts from the dapr state store.
    ///
    /// # Returns
    /// A vector of account entities
    async fn get_accounts(&self) -> Vec<AccountEntity> {
        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Empty list of entities
        let mut entities: Vec<AccountEntity> = vec![];

        // Get all data from dapr and map to entities
        client
            // Post to the url
            .post(get_sidecar_query_url())
            // Add body to the post request
            .body(
                json!(
                    {
                        "filter": {},
                    }
                )
                .to_string(),
            )
            // Send the request
            .send()
            .await
            .unwrap()
            // Get the json response and map to DaprResults
            .json::<DaprResults>()
            .await
            .unwrap_or(DaprResults { results: vec![] })
            // Loop through all results and add to entities
            .results
            .iter()
            .for_each(|entry: &Entry| entities.push(entry.data.clone()));

        // Return entities
        entities
    }

    /// Gets an account by id from the dapr state store.
    ///
    /// # Arguments
    /// * `id` - The account id
    ///
    /// # Returns
    /// An optional account entity
    async fn get_account_by_id(&self, id: String) -> Option<AccountEntity> {
        // Create the url
        let url = format!("{}/{}", get_sidecar_url(), id);

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Get account from dapr
        client
            // Get request on the url
            .get(url)
            // Send the request
            .send()
            .await
            .unwrap()
            // Get the json response and map to AccountEntity
            .json::<AccountEntity>()
            .await
            // Return the entity
            .ok()
    }

    /// Gets an account by email from the dapr state store.
    ///
    /// # Arguments
    /// * `email` - The account email
    ///
    /// # Returns
    /// An optional account entity
    async fn get_account_by_email(&self, email: String) -> Option<AccountEntity> {
        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Get first entry from dapr
        let first_entry = client
            // Post to the query url
            .post(get_sidecar_query_url())
            // Add body to the post request
            .body(
                json!(
                    {
                        "filter": {
                            "EQ": { "email": email }
                        }
                    }
                )
                .to_string(),
            )
            // Send the request
            .send()
            .await
            .unwrap()
            // Get the json response and map to DaprResults
            .json::<DaprResults>()
            .await
            .unwrap_or(DaprResults { results: vec![] })
            // Get the first entry
            .results
            .first()
            // Clone the entry
            .cloned();

        // Return account entity if exists
        match first_entry {
            Some(entry) => Some(entry.data),
            None => None,
        }
    }

    /// Validates an account in the dapr state store.
    ///
    /// # Arguments
    /// * `email` - The account email
    /// * `password` - The account password
    ///
    /// # Returns
    /// An optional account entity
    async fn validate_account(&self, email: String, password: String) -> Option<AccountEntity> {
        // Get an account by email
        match self.get_account_by_email(email).await {
            // Check if account exists
            Some(account) => {
                // Check if password matches
                if self.validate_password(password, account.password.clone().as_str()) {
                    // Return valid account
                    Some(account)
                } else {
                    // Invalid credentials, return none
                    None
                }
            }
            // Account with email does not exist
            None => None,
        }
    }

    /// Creates an account in the dapr state store.
    ///
    /// # Arguments
    /// * `account` - The account entity
    ///
    /// # Returns
    /// A boolean indicating if the account was created
    async fn create_account(&self, account: AccountEntity) -> bool {
        // Check if account exists with email
        if self
            .get_account_by_email(account.email.clone())
            .await
            .is_some()
        {
            return false;
        }

        self.save_account(account).await
    }

    /// Updates an account in the dapr state store.
    ///
    /// # Arguments
    /// * `account` - The account entity
    ///
    /// # Returns
    /// A boolean indicating if the account was updated
    async fn update_account(&self, account: AccountEntity) -> bool {
        // Return false if account not found
        if self.get_account_by_id(account.id.clone()).await.is_none() {
            return false;
        }

        self.save_account(account).await
    }

    /// Deletes an account in the dapr state store.
    ///
    /// # Arguments
    /// * `id` - The account id
    ///
    /// # Returns
    /// A boolean indicating if the account was deleted
    async fn delete_account(&self, id: String) -> bool {
        // Create the url
        let url = format!("{}/{}", get_sidecar_url(), id);

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // return false if account not found
        if self.get_account_by_id(id.clone()).await.is_none() {
            return false;
        }

        // Delete account if exists
        client
            // Delete to the url
            .delete(url)
            // Send the request
            .send()
            .await
            .unwrap()
            // Check if the request was successful
            .status()
            .is_success()
    }
}
