use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::users;
use super::schema::users::dsl::users as all_users;
// this is to get users from the database
#[derive(Serialize, Queryable)] 
pub struct User {
    pub id: i32,
    pub username: String,
    pub token: String, 
}

#[derive(Deserialize)] 
pub struct UserData { 
    pub username: String, 
}
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub url: String,
    pub token: String, 
}
impl User{
    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error!")
    }
    pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool { //new token
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }
    pub fn get_user_by_username(user: UserData, conn: &PgConnection) -> Vec<User> { //get url by
        all_users
            .filter(users::url.eq(user.username))
            .load::<User>(conn)
            .expect("Error")
    }
}