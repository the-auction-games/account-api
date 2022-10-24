use uuid::Uuid;

struct Account {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn new(name: &str, email: &str, password: &str) -> Account {
        Account {
            id: Uuid::new_v4(),
            name: String::from(name),
            email: String::from(email),
            password: String::from(password),
        }
    }
}

impl ToString for Account {
    fn to_string(&self) -> String {
        format!("Account #{}: {} ({})", self.id, self.name, self.email)
    }
}