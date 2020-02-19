#[macro_use] 
extern crate diesel;
//extern crate dotenv;


use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use actix::MailboxError;
use actix::prelude::*;
use sha2::{ Sha256, Digest };
pub mod schema;
pub mod models;

use self::models::{ User, NewUser};

pub mod jwt {
    use serde_json::Value;
    use sha2::{Sha256, Digest};
    use std::error::Error;
    
    pub struct JWTToken {
        header: Value,
        payload: Value,
        signature: String
    }

    // TODO: Allow choice of HASH Function
    // TODO: Inject default headers
    // TODO: Allow some level of configuration
    impl JWTToken {


        pub fn decode(encoded_token: &str) -> Result<JWTToken, Box<dyn Error>> {
            let token_iterator: Vec<&str>= encoded_token.split('.').collect();
            let header = token_iterator[0];
            let payload = token_iterator[1];
            let signature = token_iterator[2];

            let decoded_header = String::from_utf8(base64::decode_config(header, base64::URL_SAFE_NO_PAD)?)?;
            let decoded_payload = String::from_utf8(base64::decode_config(payload, base64::URL_SAFE_NO_PAD)?)?;

            Ok(
                JWTToken {
                    header: serde_json::from_str(&decoded_header).unwrap(),
                    payload: serde_json::from_str(&decoded_payload).unwrap(),
                    signature: String::from(signature)
                }
            )
        }

        pub fn generate_token(header: Value, payload: Value, secret_key: &str) -> JWTToken {
            let signature = JWTToken::_build_signature_static(&header, &payload, secret_key);
            JWTToken {
                header,
                payload,
                signature
            }
        }

        pub fn encode(&self) -> String {
            let encoded_header = base64::encode_config(&self.header.to_string(), base64::URL_SAFE_NO_PAD);
            let encoded_payload = base64::encode_config(&self.payload.to_string(), base64::URL_SAFE_NO_PAD);
            
            format!(
                "{}.{}.{}",
                encoded_header,
                encoded_payload,
                self.signature
            )
        }
        
        pub fn verify_signature(&self, secret_key: &str) -> bool {
            let expected_signature = self._build_signature(secret_key);
            expected_signature == self.signature
        }

        fn _build_signature_static(header: &Value, payload: &Value, secret_key: &str) -> String { 
            let encoded_header = base64::encode_config(&header.to_string(), base64::URL_SAFE_NO_PAD);
            let encoded_payload = base64::encode_config(&payload.to_string(), base64::URL_SAFE_NO_PAD);

            let mut hasher = Sha256::new();
            hasher.input(
                format!(
                    "{}.{}.{}",
                    encoded_header,
                    encoded_payload,
                    secret_key
                )
            );

            format!("{:x}", hasher.result())
        }
        fn _build_signature(&self, secret_key: &str) -> String {
            let encoded_header = base64::encode_config(&self.header.to_string(), base64::URL_SAFE_NO_PAD);
            let encoded_payload = base64::encode_config(&self.payload.to_string(), base64::URL_SAFE_NO_PAD);

            let mut hasher = Sha256::new();
            hasher.input(
                format!(
                    "{}.{}.{}",
                    encoded_header,
                    encoded_payload,
                    secret_key
                )
            );

            format!("{:x}", hasher.result())
        }
    }
}

pub struct AuthenticateUser {
    pub username: String,
    pub password: String
}

impl Message for AuthenticateUser {
    type Result = Result<User, MailboxError>;
}

pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<AuthenticateUser> for DbExecutor {
    type Result = Result<User, MailboxError>;
    
    fn handle(&mut self, msg: AuthenticateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::rusty_users::dsl::*;
        
        println!("message: user {} password {}", msg.username, msg.password);

        let user: User = rusty_users.filter(rusty_user_name.eq(msg.username)).load::<User>(&self.0).unwrap()[0].clone();

        let mut hasher = Sha256::new();
        hasher.input(msg.password.as_bytes());
        let password = format!("{:x}", hasher.result());
        if user.rusty_password == password {
            Ok(user)
        } else {
            Err(MailboxError::from(MailboxError::Closed))
        }
    }
    
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error Connecting to {}", database_url))
}



pub fn create_user<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str, rusty_user_name: &'a str, rusty_password: &'a str, rusty_role: &'a str) -> User {
    use schema::rusty_users;

    let new_user = NewUser {
        first_name,
        last_name,
        rusty_user_name,
        rusty_password,
        rusty_role
    };

    diesel::insert_into(rusty_users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error Inserting User")
}