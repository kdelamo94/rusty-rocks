#[macro_use] extern crate diesel;
extern crate rusty_rocks;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, Result};

use rusty_rocks::establish_connection;
use rusty_rocks::jwt::JWTToken;
use serde_json::json;



#[get("/rocks")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

// TODO: Good job, now use the database
// TODO: Take in a user name and password from post body, perform a database lookup, return the token if authed, else, forbid
// TODO: Diesel vs SQL Alchemy for database interface
#[get("/auth")]
async fn authorize() -> Result<HttpResponse> {

    let header = json!({
        "alg": "HS256",
        "typ": "JWT"
    });

    let payload = json!({
        "name": "kevin delamo",
        "claims" : [
            "admin",
            "user"
        ]
    });

    let secret_key = "my-super-secret-key";

    let my_token: JWTToken = JWTToken::generate_token(header, payload, secret_key);
    println!("show me it's working");
    Ok(HttpResponse::Ok().json(json!({ "token": my_token.encode()})))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let connection = establish_connection();

    HttpServer::new(|| App::new().service(index).service(authorize))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
