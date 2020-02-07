#[macro_use] 
extern crate diesel;
extern crate dotenv;


use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
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

// pub struct JWTToken {
//     header: serde_json::Value,
//     payload: serde_json::Value,
//     signature: String
// }

// impl JWTToken {

//     pub fn new(header: &str, payload: &str) -> JWTToken {
//         JWTToken{
//             header: serde_json::from_str(header).unwrap(),
//             payload: serde_json::from_str(payload).unwrap(),
//             signature: String::new()
//         }
//     }

//     pub fn get_encoded_token(&self) -> String {
//         const SECRET_KEY: &str = "my-super-secret-key";

//         let encoded_header = base64::encode_config(&self.header.to_string(), base64::URL_SAFE);
//         let encoded_payload = base64::encode_config(&self.payload.to_string(), base64::URL_SAFE);

//         let mut hasher = Sha256::new();
//         hasher.input(
//             format!(
//                 "{}{}{}",
//                 encoded_header,
//                 encoded_payload,
//                 SECRET_KEY
//             )
//         );

//         format!("{:x}", hasher.result())
//     }
// }

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error Connecting to {}", database_url))
}



pub fn create_user<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str, rusty_password: &'a str, rusty_role: &'a str) -> User {
    use schema::rusty_users;

    let new_user = NewUser {
        first_name,
        last_name,
        rusty_password,
        rusty_role
    };

    diesel::insert_into(rusty_users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error Inserting User")
}