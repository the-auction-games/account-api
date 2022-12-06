mod account;
mod data_layer;
use data_layer::{AccountEntity, get_account_dao};

#[macro_use]
extern crate rocket;

// Get all accounts
#[get("/")]
fn get_accounts() -> String {

    // loop 10 times
    for i in 0..10 {
        get_account_dao().get_account(i.to_string(), i.to_string());
    }

    format!("All accounts")
}

// Get account by id
#[get("/<account_id>")]
fn get_account(account_id: u64) -> String {
    format!("Account #{}", account_id)
}

// Create new account
#[post("/", data = "<account>")]
fn create_account(account: String) -> String {
    format!("Created account {}", account)
}

// Update account by id
#[put("/", data = "<account>")]
fn update_account(account: String) -> String {
    format!("Updated account {}", account)
}

// Delete account by id
#[delete("/<account_id>")]
fn delete_account(account_id: u64) -> String {
    format!("Deleted account {}", account_id)
}

#[post("/validate", data = "<credentials>")]
fn validate_account(credentials: String) -> String {
    format!("Validated account {}", credentials)
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
            update_account,
            validate_account
        ],
    )
}
