use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;

#[derive(Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
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
        id: i32, conn: &PgConnection
    ) -> Vec<Post> {
        all_posts.filter(posts::id.eq(id))
            .load::<Post>(conn)
            .expect("error!")
    }

    pub fn remove(
        id: i32, conn: &PgConnection
    ) -> bool {
        diesel::delete(all_posts.filter(posts::id.eq(id)))
            .execute(conn)
            .is_ok()
    }

}
