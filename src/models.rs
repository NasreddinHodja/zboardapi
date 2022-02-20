use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserNameData {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserIdData {
    pub id: i32,
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
        user: UserIdData, conn: &PgConnection
    ) -> Vec<User> {
        all_users.filter(users::id.eq(user.id))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn remove(
        user: UserIdData, conn: &PgConnection
    ) -> bool {
        diesel::delete(all_users.filter(users::id.eq(user.id)))
            .execute(conn)
            .is_ok()
    }

    pub fn get_by_username(
        user: UserNameData, conn: &PgConnection
    ) -> Vec<User> {
        all_users.filter(users::username.eq(user.username))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn remove_by_username(
        user: UserNameData, conn: &PgConnection
    ) -> bool {
        diesel::delete(all_users.filter(users::username.eq(user.username)))
            .execute(conn)
            .is_ok()
    }

}

use super::schema::posts;
use super::schema::posts::dsl::posts as all_posts;

#[derive(Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct PostIdData {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPostData {
    pub title: String,
    pub body: String,
}

impl Post {

    pub fn get_all(conn: &PgConnection) -> Vec<Post> {
        all_posts.order(posts::id.desc())
            .load::<Post>(conn)
            .expect("error!")
    }

    pub fn insert(post: NewPostData, conn: &PgConnection) -> bool {
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(conn)
            .is_ok()
    }

    pub fn get(
        post: PostIdData, conn: &PgConnection
    ) -> Vec<Post> {
        all_posts.filter(posts::id.eq(post.id))
            .load::<Post>(conn)
            .expect("error!")
    }

    pub fn remove(
        post: PostIdData, conn: &PgConnection
    ) -> bool {
        diesel::delete(all_posts.filter(posts::id.eq(post.id)))
            .execute(conn)
            .is_ok()
    }

}
