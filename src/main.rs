mod data_layer;
mod services;
use data_layer::DaprAccountDao;
use rocket::serde::json::{
    serde_json::{json, Value},
    Json,
};
use services::{AccountModel, AccountService, CredentialsModel};

#[macro_use]
extern crate rocket;

/**
 * TODO:
 * - CHECK IF WE CAN PASS A SERVICE INSTANCE TO THE ROUTES
 * - FINISH DAO METHOD IMPLEMENTATIONS
 * - RETURN STATUS CODES IN ROUTES
 * - COMMENTS
 */

// Get all accounts
#[get("/")]
async fn get_accounts() -> Value {
    let service = services::DaprAccountService::new(DaprAccountDao::new());
    json!(service.get_accounts().await)
}

// Get account by id
#[get("/<id>")]
async fn get_account(id: String) -> Value {
    let service = services::DaprAccountService::new(DaprAccountDao::new());

    let details = service.get_account(id).await;

    match details {
        Some(account) => json!(account),
        None => json!({ "error": "Account not found" }),
    }
}

// Create new account
#[post("/", format = "application/json", data = "<account>")]
async fn create_account(account: Json<AccountModel>) -> Value {
    let service = services::DaprAccountService::new(DaprAccountDao::new());

    let account = account.into_inner();

    println!("Account: {:?}", &account);

    if service.create_account(account).await {
        json!({"success": "Account created"})
    } else {
        json!({ "error": "Account not created" })
    }
}

// Update account by id
#[put("/", format = "application/json", data = "<account>")]
async fn update_account(account: Json<AccountModel>) -> Value {
    let service = services::DaprAccountService::new(DaprAccountDao::new());

    let account = account.into_inner();

    println!("Account: {:?}", &account);

    if service.update_account(account).await {
        json!({"success": "Account created"})
    } else {
        json!({ "error": "Account not found" })
    }
}

// Delete account by id
#[delete("/<account_id>")]
async fn delete_account(account_id: u64) -> Value {
    let service = services::DaprAccountService::new(DaprAccountDao::new());

    if service.delete_account(account_id.to_string()).await {
        json!({"success": "Account deleted"})
    } else {
        json!({ "error": "Account not found" })
    }
}

// Validate account by email and password
#[post("/validate", format = "application/json", data = "<credentials>")]
async fn validate_account(credentials: Json<CredentialsModel>) -> Value {
    let service = services::DaprAccountService::new(DaprAccountDao::new());

    let credentials = credentials.into_inner();

    println!("Credentials: {:?}", &credentials);

    let account = service.validate_account(credentials).await;

    match account {
        Some(account) => json!(account),
        None => json!({ "error": "Account not found" }),
    }
}

// Start the server
#[launch]
async fn rocket() -> _ {
    rocket::build().mount(
        "/accounts",
        routes![
            get_account,
            get_accounts,
            create_account,
            delete_account,
            update_account,
            validate_account
        ],
    )
}
