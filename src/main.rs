extern crate type_printer;

fn main() {
    print_title();

    let david = User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string(),
        email: "david@example.com".to_string()
    };

    let guest = Guest;

    print_stats(david);
    seperator();
    print_stats(guest);
}

fn print_stats<T>(user: T) where T: FullName + Email {
    println!("^^^^^ Type {:?}", type_printer::print_type_of(&user));
    println!("Name: {}", user.name());
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
