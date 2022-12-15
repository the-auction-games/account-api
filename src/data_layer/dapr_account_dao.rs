use super::account_dao::AccountDao;
use super::account_entity::AccountEntity;
use reqwest::ClientBuilder;
use rocket::{
    async_trait,
    serde::json::serde_json::json,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct AllData {
    results: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Entry {
    data: AccountEntity,
}

pub struct DaprAccountDao {}

impl DaprAccountDao {
    pub fn new() -> Self {
        DaprAccountDao {}
    }
}

const STATE_STORE_NAME: &str = "postgres";

#[async_trait]
impl AccountDao for DaprAccountDao {
    async fn get_accounts(&self) -> Vec<AccountEntity> {
        let url = format!(
            "http://localhost:3500/v1.0-alpha1/state/{}/query",
            STATE_STORE_NAME
        );
        let client = ClientBuilder::new().build().unwrap();

        let mut entities: Vec<AccountEntity> = vec![];

        client
            .post(url)
            .body(json!({ "filter": {}, }).to_string())
            .send()
            .await
            .unwrap()
            .json::<AllData>()
            .await
            .unwrap()
            .results
            .iter()
            .for_each(|entry: &Entry| entities.push(entry.data.clone()));

        entities
    }

    async fn get_account(&self, id: String) -> Option<AccountEntity> {
        let url = format!("http://localhost:3500/v1.0/state/{}/{}", STATE_STORE_NAME, id);

        println!("url: {}", url);

        let client = ClientBuilder::new().build().unwrap();

        client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<AccountEntity>()
            .await
            .ok()
    }

    async fn validate_account(&self, _email: String, _password: String) -> Option<AccountEntity> {
        // TODO
        None
    }

    async fn create_account(&self, account: AccountEntity) -> bool {
        self.update_account(account).await
    }

    async fn update_account(&self, _account: AccountEntity) -> bool {
        // TODO
        false
    }

    async fn delete_account(&self, _id: String) -> bool {
        // TODO
        false
    }
}
