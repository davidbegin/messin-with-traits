#![feature(convert)]

extern crate type_printer;
extern crate regex;
mod users;
mod tests;
use users::FullName;
use users::Email;
use users::ObfuscatedEmail;

fn main() {
    print_title();

    let david = users::User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };

    let guest = users::Guest;

    print_stats(david); seperator(); print_stats(guest);
}

fn print_stats<T>(user: T) where T: users::FullName + users::Email {
    println!("^^^^^ Type {:?}", type_printer::print_type_of(&user));
    print_name(&user);
    print_email(&user);
}

fn print_name<T>(user: &T) where T: users::FullName {
    println!("Name: {}", user.name());
}

fn print_email<T>(user: &T) where T: users::Email {
    println!("Email: {}", user.email());
}

fn print_title() {
    println!("\n\nMessin' with traits");
    println!("===================\n\n");
}

fn seperator() {
    println!("\n");
}
