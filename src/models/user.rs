use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUserData {
    pub username: String,
    pub password: String,
}

impl User {

    pub fn get_all(conn: &PgConnection) -> Vec<User> {
        all_users.order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert(user: NewUserData, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get(
        id: i32, conn: &PgConnection
    ) -> Vec<User> {
        all_users.filter(users::id.eq(id))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn remove(
        id: i32, conn: &PgConnection
    ) -> bool {
        diesel::delete(all_users.filter(users::id.eq(id)))
            .execute(conn)
            .is_ok()
    }

    pub fn get_by_username(
        name: &str, conn: &PgConnection
    ) -> Vec<User> {
        all_users.filter(users::username.eq(name))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn remove_by_username(
        name: &str, conn: &PgConnection
    ) -> bool {
        diesel::delete(all_users.filter(users::username.eq(name)))
            .execute(conn)
            .is_ok()
    }

}
