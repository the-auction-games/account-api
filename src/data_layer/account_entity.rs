///
/// The Account Entity.
///
/// This entity is used to directly store account data
/// in the database.
///
pub struct AccountEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
