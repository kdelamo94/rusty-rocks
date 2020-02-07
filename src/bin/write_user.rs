extern crate diesel;

use rusty_rocks::*;
use std::io::{ stdin, Read };
use sha2::{ Sha256, Digest};

fn main(){
    let connection = establish_connection();

    let mut hasher = Sha256::new();
    
    println!("First Name? ");
    let mut first_name = String::new();
    stdin().read_line(&mut first_name).unwrap();

    let first_name = &first_name[..(first_name.len() - 1)];

    println!("Last Name? ");
    let mut last_name = String::new();

    stdin().read_line(&mut last_name).unwrap();
    let last_name = &last_name[..(last_name.len() -1 )];

    println!("Password? ");
    let mut password = String::new();
    stdin().read_line(&mut password);

    let password = &password[..(password.len() - 1)];

    hasher.input(password.as_bytes());
    let password = &format!("{:x}", hasher.result());

    let user_role = "user";

    let user = create_user(&connection, first_name, last_name, password, user_role);

    println!("Saved new user {} whose id is {}", user.first_name, user.id)

}