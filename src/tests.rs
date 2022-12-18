use super::rocket;
use crate::services::{AccountDetails, AccountModel, CredentialsModel};
use rocket::http::ContentType;
use rocket::serde::json::json;
use rocket::{http::Status, local::blocking::Client};

/// Test the get accounts endpoint.
#[test]
fn test_get_all() {
    // Create client
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Make request
    let response = client.get("/accounts").dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Ok);

    // Make sure parsing does not throw an error
    response.into_json::<Vec<AccountDetails>>().unwrap();
}

/// Test the create account endpoint
///
/// # Note
/// This will test creation and deletion.
#[test]
fn test_create_and_delete() {
    // Create client
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Get the size of the accounts before creation
    let before_size = client
        .get("/accounts")
        .dispatch()
        .into_json::<Vec<AccountDetails>>()
        .unwrap()
        .len();

    // Create account
    let account = AccountModel {
        id: "test_1".to_string(),
        name: "Test 1".to_string(),
        email: "test1@gmail.com".to_string(),
        password: "password".to_string(),
    };

    // Post the new account
    let response = client
        .post("/accounts")
        .header(ContentType::JSON)
        .body(json!(&account).to_string())
        .dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Created);

    // Assert the size has increased
    let after_size = client
        .get("/accounts")
        .dispatch()
        .into_json::<Vec<AccountDetails>>()
        .unwrap()
        .len();
    assert_eq!(after_size, before_size + 1);

    // Delete account
    let response = client.delete("/accounts/id/test_1").dispatch();

    // Assert response is no content
    assert_eq!(response.status(), Status::NoContent);

    // Assert the size has decreased
    let after_size = client
        .get("/accounts")
        .dispatch()
        .into_json::<Vec<AccountDetails>>()
        .unwrap()
        .len();
    assert_eq!(after_size, before_size);
}

/// Test the get account by id endpoint.
///
/// # Note
/// This will test creation, get by id, and deletion.
#[test]
fn test_get_by_id() {
    // Create client
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Create account
    let account = AccountModel {
        id: "test_1".to_string(),
        name: "Test 1".to_string(),
        email: "test1@gmail.com".to_string(),
        password: "password".to_string(),
    };

    // Post the new account
    let response = client
        .post("/accounts")
        .header(ContentType::JSON)
        .body(json!(&account).to_string())
        .dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Created);

    // Get account by id
    let response = client.get("/accounts/id/test_1").dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Ok);

    // Attempt to parse the response
    response.into_json::<AccountDetails>().unwrap();

    // Delete account
    let response = client.delete("/accounts/id/test_1").dispatch();

    // Assert response is no content
    assert_eq!(response.status(), Status::NoContent);
}

/// Test the get account by email endpoint.
///
/// # Note
/// This will test creation, get by email, and deletion.
#[test]
fn test_get_by_email() {
    // Create client
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Create account
    let account = AccountModel {
        id: "test_1".to_string(),
        name: "Test 1".to_string(),
        email: "test1@gmail.com".to_string(),
        password: "password".to_string(),
    };

    // Post the new account
    let response = client
        .post("/accounts")
        .header(ContentType::JSON)
        .body(json!(&account).to_string())
        .dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Created);

    // Get account by email
    let response = client.get("/accounts/email/test1@gmail.com").dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Ok);

    // Attempt to parse the response
    response.into_json::<AccountDetails>().unwrap();

    // Delete account
    let response = client.delete("/accounts/id/test_1").dispatch();

    // Assert response is no content
    assert_eq!(response.status(), Status::NoContent);
}

/// Test the update account endpoint.
///
/// # Note
/// This will test creation, update, and deletion.
#[test]
fn test_update() {
    // Create client
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Create account
    let mut account = AccountModel {
        id: "test_1".to_string(),
        name: "Test 1".to_string(),
        email: "test1@gmail.com".to_string(),
        password: "password".to_string(),
    };

    // Post the new account
    let response = client
        .post("/accounts")
        .header(ContentType::JSON)
        .body(json!(&account).to_string())
        .dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Created);

    // Update the account model
    account.email = "updated@gmail.com".to_string();

    // Put the updated account
    let response = client
        .put("/accounts")
        .header(ContentType::JSON)
        .body(json!(&account).to_string())
        .dispatch();

    // Assert response is no content
    assert_eq!(response.status(), Status::NoContent);

    // Get account by id
    let response = client.get("/accounts/id/test_1").dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Ok);

    // Attempt to parse the response
    let account_details = response.into_json::<AccountDetails>().unwrap();

    // Assert the email has been updated
    assert_eq!(account_details.email, "updated@gmail.com".to_string());

    // Delete account
    let response = client.delete("/accounts/id/test_1").dispatch();

    // Assert response is no content
    assert_eq!(response.status(), Status::NoContent);
}

/// Test the validate account endpoint.
///
/// # Note
/// This will test creation, validation, and deletion.
#[test]
fn test_validate_account() {
    // Create client
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // Create account
    let account = AccountModel {
        id: "test_1".to_string(),
        name: "Test 1".to_string(),
        email: "test1@gmail.com".to_string(),
        password: "password".to_string(),
    };

    // Post the new account
    let response = client
        .post("/accounts")
        .header(ContentType::JSON)
        .body(json!(&account).to_string())
        .dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Created);

    // Create credentails model
    let credentials = CredentialsModel {
        email: "test1@gmail.com".to_string(),
        password: "password".to_string(),
    };

    // Validate by email and password
    let response = client
        .post("/accounts/validate")
        .header(ContentType::JSON)
        .body(json!(&credentials).to_string())
        .dispatch();

    // Assert response is ok
    assert_eq!(response.status(), Status::Ok);

    // Attempt to parse the response
    response.into_json::<AccountDetails>().unwrap();

    // Delete account
    let response = client.delete("/accounts/id/test_1").dispatch();

    // Assert response is no content
    assert_eq!(response.status(), Status::NoContent);
}
