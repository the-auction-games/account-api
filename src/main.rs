mod account;

#[macro_use] extern crate rocket;

// Get all accounts
#[get("/")]
fn get_accounts() -> String {
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
#[put("/<account_id>", data = "<account>")]
fn update_account(account_id: u64, account: String) -> String {
    format!("Updated account {}: {}", account_id, account)
}

// Delete account by id
#[delete("/<account_id>")]
fn delete_account(account_id: u64) -> String {
    format!("Deleted account {}", account_id)
}

// Start the server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/account", routes![get_account, get_accounts, create_account, delete_account, update_account])
}
