// TODO: UPDATE UMLS TO MATCH THIS
mod data_layer;
mod services;
use services::{AccountService, get_account_service, CredentialsModel, AccountModel};

#[macro_use]
extern crate rocket;

// Get all accounts
#[get("/")]
fn get_accounts() -> String {

    let service: Box<dyn AccountService> = services::get_account_service();

    // TODO: TEST THIS

    service.get_accounts().iter().map(|account| {
        format!("Account #{}: {}", account.id, account.name)
    }).collect::<Vec<String>>().join("")
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
