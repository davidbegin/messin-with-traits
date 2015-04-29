extern crate type_printer;

fn main() {
    print_title();

    let david = User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };

    let guest = Guest;

    print_stats(david); seperator(); print_stats(guest);
}

fn print_stats<T>(user: T) where T: FullName + Email {
    println!("^^^^^ Type {:?}", type_printer::print_type_of(&user));
    print_name(&user);
    print_email(&user);
}

fn print_name<T>(user: &T) where T: FullName {
    println!("Name: {}", user.name());
}

fn print_email<T>(user: &T) where T: Email {
    println!("Email: {}", user.email());
}

struct User {
    first_name: String,
    last_name: String,
    email: String,
}

struct Guest;

impl FullName for User {
    fn name(&self) -> String {
        format!(
            "{} {}",
            self.first_name.to_string(),
            self.last_name.to_string()
        )
    }
}

impl Email for User {
    fn email(&self) -> String {
        self.email.to_string()
    }
}

impl Email for Guest {
    fn email(&self) -> String {
        "guest@example.com".to_string()
    }
}

impl FullName for Guest {
    fn name(&self) -> String {
        "Guest".to_string()
    }
}

trait FullName {
    fn name(&self) -> String;
}

trait Email {
    fn email(&self) -> String;
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
