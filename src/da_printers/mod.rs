use users;
extern crate type_printer;

pub fn print_stats<T>(user: T) where T: users::FullName + users::Email {
    println!("^^^^^ Type {:?}", type_printer::print_type_of(&user));
    print_name(&user);
    print_email(&user);
}

pub fn print_name<T>(user: &T) where T: users::FullName {
    println!("Name: {}", user.name());
}

pub fn print_email<T>(user: &T) where T: users::Email {
    println!("Email: {}", user.email());
}

pub fn print_title() {
    println!("\n\nMessin' with traits");
    println!("===================\n\n");
}

pub fn seperator() {
    println!("\n");
}
