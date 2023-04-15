# Account API
This is the Account API for The Auction Games. It provides functionality for creating, updating, quering, and deleting accounts. It utilizes Dapr for service discovery and state management.

## Project Structure
The project is structured as follows:
```
.
├── src/                'Source files'
│   ├── data/           'Data access layer'
│   ├── services/       'Business logic layer'
│   ├── main.rs         'Entry point'
│   ├── test.rs         'Integration tests'
├── Cargo.toml          'Manifest file'
└── README.md           'This file'
```

## API Documentation
https://app.swaggerhub.com/apis/JOELSMITH2019/account-api/1.0.0

## Testing
Run the command `cargo test -- --test-threads=1` to test the API. Tests must take place sequentially.
