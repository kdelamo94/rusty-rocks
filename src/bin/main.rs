#[macro_use] extern crate diesel;
extern crate rusty_rocks;
use actix_cors::Cors;
use chrono::DateTime;
use actix_web::{
    http,
    get,
    post,
    middleware, 
    web, 
    App, 
    HttpServer, 
    Responder, 
    HttpResponse, 
    HttpRequest,
    Result
};

use actix::Addr;
use actix::SyncArbiter;
use rusty_rocks::DbExecutor;
use rusty_rocks::establish_connection;
use rusty_rocks::jwt::JWTToken;
use rusty_rocks::AuthenticateUser;
use serde_json::json;
use serde::Deserialize;



#[get("/rocks")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

// TODO: Good job, now use the database
// TODO: Take in a user name and password from post body, perform a database lookup, return the token if authed, else, forbid
// TODO: Diesel vs SQL Alchemy for database interface
#[derive(Deserialize)]
struct AuthorizationRequest {
    pub username: String,
    pub password: String
}

#[post("/auth")]
async fn authorize(datum: web::Data<State>, body: web::Json<AuthorizationRequest>) -> Result<HttpResponse> {



    let secret_key = "my-super-secret-key";

    let db = &datum.addr;

    let user = db.send(AuthenticateUser{
        username: body.username.clone(),
        password: body.password.clone()
    }).await??;
    
    let timestamp = chrono::Utc::now();

    let header = json!({
        "alg": "HS256",
        "typ": "JWT"
    });

    let payload = json!({
        "sub": user.id,
        "name": user.rusty_user_name,
        "iat": timestamp.to_rfc3339(),
        "claims" : [
            "admin",
            "user"
        ]
    });

    let my_token: JWTToken = JWTToken::generate_token(header, payload, secret_key);

    Ok(HttpResponse::Ok().json(json!({ "token": my_token.encode()})))
}

struct State {
    addr: Addr<DbExecutor>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {


    let db_actors = SyncArbiter::start(3, move || {
        DbExecutor(establish_connection())
    });

    HttpServer::new(move || App::new()
        .data(State { addr: db_actors.clone()})
        .wrap(
            Cors::new()
                .allowed_origin("*")
                .allowed_methods(vec!["GET","POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish()
        ).service(index).service(authorize)
    ).bind("127.0.0.1:8080")?
        .run()
        .await

}
