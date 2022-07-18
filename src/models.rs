use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub did: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub did: &'a str,
}
