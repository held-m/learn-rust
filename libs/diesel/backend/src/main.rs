use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::insert_into;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::Post;


#[get("/")]
async fn index() -> impl Responder {

    use schema::posts::dsl::*;

    // use schema::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    HttpResponse::Ok().body("hi".to_string())
}

#[get("/add")]
async fn add() -> impl Responder {
    use schema::posts::dsl::*;
    println!("try add post");

    insert_into(posts)
        .values((title.eq("name1"), body.eq("body1")))
        .execute(&establish_connection()).unwrap();
    println!("posr added");
    HttpResponse::Ok().body("post added")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = get_db_url();//.expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn get_db_url() -> String {
        format!("mysql://{user}:{password}@{host}:{port}/{database}",
                user = env::var("MYSQL_USER").unwrap(),
                password = env::var("MYSQL_PASSWORD").unwrap(),
                host = env::var("MYSQL_HOST").unwrap(),
                port = env::var("MYSQL_PORT").unwrap(),
                database = env::var("MYSQL_DATABASE").unwrap(),
        )
    }
