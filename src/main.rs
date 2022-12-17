mod data;
mod services;
use data::DaprAccountDao;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{
        serde_json::{json, Value},
        Json,
    },
    State,
};
use services::{AccountModel, AccountService, CredentialsModel, DaprAccountService};

// Set testing file
#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

/// API endpoint to get all accounts.
///
/// # Arguments
/// * `provider` - The service provider for account operations
///
/// # Returns
/// * `Custom<Value>` - The list of accounts
#[get("/")]
async fn get_accounts(provider: &State<ServiceProvider>) -> Custom<Value> {
    Custom(Status::Ok, json!(provider.service.get_accounts().await))
}

/// API endpoint to get an account by id.
///
/// # Arguments
/// * `provider` - The service provider for account operations
/// * `id` - The id of the account to get
///
/// # Returns
/// * `Custom<Value>` - The account
#[get("/id/<id>")]
async fn get_account_by_id(provider: &State<ServiceProvider>, id: String) -> Custom<Value> {
    match provider.service.get_account_by_id(id).await {
        Some(account) => Custom(Status::Ok, json!(account)),
        None => Custom(Status::NotFound, json!({})),
    }
}

/// API endpoint to get an account by email.
///
/// # Arguments
/// * `provider` - The service provider for account operations
/// * `email` - The email of the account to get
///
/// # Returns
/// * `Custom<Value>` - The account
#[get("/email/<email>")]
async fn get_account_by_email(provider: &State<ServiceProvider>, email: String) -> Custom<Value> {
    match provider.service.get_account_by_email(email).await {
        Some(account) => Custom(Status::Ok, json!(account)),
        None => Custom(Status::NotFound, json!({})),
    }
}

/// API endpoint to create an account.
///
/// # Arguments
/// * `provider` - The service provider for account operations
/// * `account` - The account to create
///
/// # Returns
/// * `Custom<Value>` - The created account
#[post("/", format = "application/json", data = "<account>")]
async fn create_account(
    provider: &State<ServiceProvider>,
    account: Json<AccountModel>,
) -> Custom<Value> {
    if provider.service.create_account(account.into_inner()).await {
        Custom(Status::Created, json!({}))
    } else {
        Custom(
            Status::Conflict,
            json!({ "error": "Account already exists" }),
        )
    }
}

/// API endpoint to update an account.
///
/// # Arguments
/// * `provider` - The service provider for account operations
/// * `account` - The account to update
///
/// # Returns
/// * `Status` - The status of the operation
#[put("/", format = "application/json", data = "<account>")]
async fn update_account(provider: &State<ServiceProvider>, account: Json<AccountModel>) -> Status {
    if provider.service.update_account(account.into_inner()).await {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

/// API endpoint to delete an account by id.
///
/// # Arguments
/// * `provider` - The service provider for account operations
/// * `account_id` - The id of the account to delete
#[delete("/id/<account_id>")]
async fn delete_account(provider: &State<ServiceProvider>, account_id: String) -> Status {
    if provider
        .service
        .delete_account(account_id.to_string())
        .await
    {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

/// API endpoint to get validate an account by email and password.
///
/// # Arguments
/// * `provider` - The service provider for account operations
/// * `credentials` - The credentials to validate
#[post("/validate", format = "application/json", data = "<credentials>")]
async fn validate_account(
    provider: &State<ServiceProvider>,
    credentials: Json<CredentialsModel>,
) -> Custom<Value> {
    match provider
        .service
        .validate_account(credentials.into_inner())
        .await
    {
        Some(account) => Custom(Status::Ok, json!(account)),
        None => Custom(Status::NotFound, json!({})),
    }
}

/// The service provider for account operations.
struct ServiceProvider {
    service: DaprAccountService,
}

/// Start the rocket server.
///
/// This method replaces the main method in a normal rust application.
///
/// # Returns
/// * `rocket::Rocket` - The rocket server
#[launch]
fn rocket() -> _ {

    // Notify console of starting server
    println!("Starting server...");

    // The dapr account service for account operations
    let service: ServiceProvider = ServiceProvider {
        service: services::DaprAccountService::new(DaprAccountDao::new()),
    };

    // Start the server
    rocket::build().manage(service).mount(
        "/accounts",
        routes![
            get_accounts,
            get_account_by_id,
            get_account_by_email,
            create_account,
            delete_account,
            update_account,
            validate_account
        ],
    )
}
