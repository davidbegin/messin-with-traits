pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub struct Guest;

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

pub trait FullName {
    fn name(&self) -> String;
}

pub trait Email {
    fn email(&self) -> String;
}

