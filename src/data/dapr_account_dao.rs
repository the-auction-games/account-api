use super::account_dao::AccountDao;
use super::account_entity::AccountEntity;
use reqwest::ClientBuilder;
use rocket::{
    async_trait,
    serde::json::serde_json::json,
    serde::{Deserialize, Serialize},
};

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
}

/// The dapr state storage name
const STATE_STORE_NAME: &str = "postgres";

/// The dapr account dao implementation.
#[async_trait]
impl AccountDao for DaprAccountDao {
    /// Gets all accounts from the dapr state store.
    ///
    /// # Returns
    /// A vector of account entities
    async fn get_accounts(&self) -> Vec<AccountEntity> {
        // Dapr query url
        let url = format!(
            "http://localhost:3500/v1.0-alpha1/state/{}/query",
            STATE_STORE_NAME
        );

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Empty list of entities
        let mut entities: Vec<AccountEntity> = vec![];

        // Get all data from dapr and map to entities
        client
            .post(url)
            .body(
                json!(
                    {
                        "filter": {},
                    }
                )
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .json::<DaprResults>()
            .await
            .unwrap()
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
        // Dapr sidecar url
        let url = format!(
            "http://localhost:3500/v1.0/state/{}/{}",
            STATE_STORE_NAME, id
        );

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Get account from dapr
        client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<AccountEntity>()
            .await
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
        // Dapr query url
        let query_url = format!(
            "http://localhost:3500/v1.0-alpha1/state/{}/query",
            STATE_STORE_NAME
        );

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Get first entry from dapr
        let first_entry = client
            .post(query_url)
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
            .send()
            .await
            .unwrap()
            .json::<DaprResults>()
            .await
            .unwrap_or(DaprResults { results: vec![] })
            .results
            .first()
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
        // Dapr sidecar url
        let url = format!(
            "http://localhost:3500/v1.0-alpha1/state/{}/query",
            STATE_STORE_NAME
        );

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Collect the first entry in the state storage
        // with a matching email and password
        let first_entry = client
            .post(url)
            .body(
                json!(
                    {
                        "filter": {
                            "AND": [
                                {
                                    "EQ": { "email": email }
                                },
                                {
                                    "EQ": { "password": password }
                                }
                            ]
                        }
                    }
                )
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .json::<DaprResults>()
            .await
            .unwrap_or(DaprResults { results: vec![] })
            .results
            .first()
            .cloned();

        // Return account entity if exists
        match first_entry {
            Some(entry) => Some(entry.data),
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

        // Dapr sidecar url
        let url = format!("http://localhost:3500/v1.0/state/{}", STATE_STORE_NAME);

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // Post if account creation is successful
        client
            .post(url)
            .body(
                json!(
                    [
                        {
                            "key": account.id,
                            "value": account,
                            // {
                            //     "id": account.id,
                            //     "name": account.name,
                            //     "email": account.email,
                            //     "password": account.password,
                            // },
                        },
                    ]
                )
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .status()
            .is_success()
    }

    /// Updates an account in the dapr state store.
    ///
    /// # Arguments
    /// * `account` - The account entity
    ///
    /// # Returns
    /// A boolean indicating if the account was updated
    async fn update_account(&self, account: AccountEntity) -> bool {
        // Dapr sidecar url
        let url = format!("http://localhost:3500/v1.0/state/{}", STATE_STORE_NAME);

        // Request client
        let client = ClientBuilder::new().build().unwrap();

        // return false if account not found
        if self.get_account_by_id(account.id.clone()).await.is_none() {
            return false;
        }

        // Post if account is found
        client
            .post(url)
            .body(
                json!(
                    [
                        {
                            "key": account.id,
                            "value": account,
                        },
                    ]
                )
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .status()
            .is_success()
    }

    /// Deletes an account in the dapr state store.
    ///
    /// # Arguments
    /// * `id` - The account id
    ///
    /// # Returns
    /// A boolean indicating if the account was deleted
    async fn delete_account(&self, id: String) -> bool {
        // The dapr sidecar url
        let url = format!(
            "http://localhost:3500/v1.0/state/{}/{}",
            STATE_STORE_NAME, id
        );

        // Reqwest client
        let client = ClientBuilder::new().build().unwrap();

        // return false if account not found
        if self.get_account_by_id(id.clone()).await.is_none() {
            return false;
        }

        // Delete account if exists
        client
            .delete(url)
            .send()
            .await
            .unwrap()
            .status()
            .is_success()
    }
}
