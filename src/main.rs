#[macro_use]
extern crate rocket;

// TODO: Use this
use rocket::serde::{json::Json, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Account {
    id: String,
    name: String,
    email: String,
    password: String,
}

// Get all accounts
#[get("/")]
fn get_accounts() -> String {
    format!("All accounts")
}

// Get account by id
#[get("/<account_id>")]
async fn get_account(account_id: u64) -> Json<Account> {
    let url: String = format!("http://localhost:3500/v1.0/state/postgres/{}", account_id);
    let account: Account = reqwest::get(&url).await.unwrap().json().await.unwrap();

    // let j = Json(account);
    // format!("Account: {}", j.email)
    Json(account)
}

// Create new account
#[post("/", data = "<account>")]
async fn create_account(account: String) -> String {
    format!("Created account {}", account)
}

// Update account by id
#[put("/<account_id>", data = "<account>")]
async fn update_account(account_id: u64, account: String) -> String {
    format!("Updated account {}: {}", account_id, account)
}

// Delete account by id
#[delete("/<account_id>")]
async fn delete_account(account_id: u64) -> String {
    format!("Deleted account {}", account_id)
}

// Start the server
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/account",
        routes![
            get_account,
            get_accounts,
            create_account,
            delete_account,
            update_account
        ],
    )
}
