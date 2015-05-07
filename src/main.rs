#![feature(convert)]

extern crate regex;
mod users;
mod tests;
mod da_printers;

fn main() {
    da_printers::print_title();

    let david = users::User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };

    let guest = users::Guest;

    da_printers::print_stats(david);
    da_printers::seperator();
    da_printers::print_stats(guest);
}

