pub fn back_in_the_habit() {
    println!("Time to remember trait objects");
    println!("==============================");

    println!("{}", 8u8.method());
    println!("{}", "nice".to_string().method());

    // do_something(11u8);

    let x = 5u8;
    // Type ensure!!! whaaa
    do_something2(&x as &Foo);
}


fn do_something2(x: &Foo) {
  x.method();
}

fn do_something<T: Foo>(x: T) {
    x.method();
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        println!("method called!");
        "U8 in da house".to_string()
    }
}

impl Foo for String {
    fn method(&self) -> String {
        println!("method called!");
        "String in da house".to_string()
    }
}
