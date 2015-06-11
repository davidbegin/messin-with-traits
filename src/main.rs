#![allow(dead_code, unused_variables)]
#![feature(convert)]

extern crate regex;
mod users;
mod tests;
mod da_printers;

fn main() {
    da_printers::print_title();

    // let david = users::User {
    //     first_name: "David".to_string(),
    //     last_name: "Begin".to_string(),
    //     email: "david@example.com".to_string()
    // };
    //
    // let guest = users::Guest;
    //
    // da_printers::print_stats(david);
    // da_printers::seperator();
    // da_printers::print_stats(guest);

    trait_madness();
}

fn trait_madness() {
    let c1 = Circle { x: 5.1f64, y: 6.4f64, radius: 8.2f64 };
    area_printer(&c1);

    let s1 = Square { side: 10.1, title: "Square".to_owned() };
    area_printer(&s1);


    // these can not be borrowed, because when
    // the title needs to be printed, it needs to be
    // deferenced, and that takes ownership,
    //
    // this is not a problem for i32, because
    // it has Copy and Clone implemented
    title_printer(s1);
    title_printer(c1);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn area_printer<T: HasArea>(obj: &T) {
    println!("Area of obj: {}", obj.area());
}

struct Square {
    side: f64,
    title: String,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

trait HasTitle {
    fn title(self) -> String;
}

fn title_printer<T: HasTitle>(obj: T) {
    println!("Title: {}", obj.title());
}

impl HasTitle for Square {
    fn title(self) -> String {
        self.title
    }
}

impl HasTitle for Circle {
    fn title(self) -> String {
        "Circle".to_string()
    }
}
