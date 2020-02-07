use super::schema::rusty_users;
use diesel::Queryable;
use diesel::Insertable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub rusty_password: String,
    pub rusty_role: String
}

#[derive(Insertable)]
#[table_name="rusty_users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub rusty_password: &'a str,
    pub rusty_role: &'a str
}