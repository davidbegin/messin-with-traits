Messin' with Traits
---

```rust
fn main() {
    print_title();

    let david = User {
        first_name: "David".to_string(),
        last_name: "Begin".to_string()
    };

    let guest = Guest {
        default_name: "Guest User".to_string()
    };

    name_printer(david);
    name_printer(guest);
}

fn name_printer<T: FullName>(user: T) {
    println!("Name: {}", user.name());
}

struct User {
    first_name: String,
    last_name: String,
}

struct Guest {
    default_name: String
}

impl FullName for User {
    fn name(&self) -> String {
        format!(
            "{} {}",
            self.first_name.to_string(),
            self.last_name.to_string()
        )
    }
}

impl FullName for Guest {
    fn name(&self) -> String {
        self.default_name.to_string()
    }
}

trait FullName {
    fn name(&self) -> String;
}

fn print_title() {
    println!("\n\nMessin' with traits");
    println!("===================\n\n");
}
```
