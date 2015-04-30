extern crate type_printer;
mod users;

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

#[test]
fn name_returns_the_users_full_name() {
    let subject = User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };
    assert_eq!(subject.name(), "David Begin".to_string());
}

#[test]
fn email_returns_the_users_email() {
    let subject = User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };
    assert_eq!(subject.email(), "david@example.com".to_string());
}
