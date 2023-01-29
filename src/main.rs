mod data;
mod services;

use data::DaprAccountDao;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    response::status::Custom,
    serde::json::{
        serde_json::{json, Value},
        Json,
    },
    Request, Response, State,
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

/// API endpoint to handle the OPTIONS request.
/// 
/// This is a requirement for CORS. This method lets
/// the client know CORS is acceptable.
#[options("/<_..>")]
fn options() {
    /* Left blank. This will trigger the Cors fairing response. */
}

/// The service provider for account operations.
struct ServiceProvider {
    service: DaprAccountService,
}

/// The CORS fairing for the server.
pub struct Cors;

/// The CORS fairing for the server.
///
/// # Methods
/// * `info` - The info for the fairing
/// * `on_response` - The response for the fairing
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
    rocket::build()
        .attach(Cors)
        .manage(service)
        .mount(
            "/api/v1/accounts",
            routes![
                options,
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
