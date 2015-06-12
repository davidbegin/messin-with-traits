pub fn rep_one() {
    println!("Warm up 1!");

    c1(1);
    c1("woah");
    c1("fancy beat".to_string());
    c1(232.44f64);
    c1(b'a');

    // I want to do vec, but it might be too complicatd for now!
    //
    // I need to get back to mess with vecs more.
    //
    // I am also out of practice
    // let vec: Vec<i32> = vec![1];
    c2(2, 1);
}

trait HasDoIt {
    fn do_it(&self);
}

trait HasYeahYeah {
    fn yeah_yeah(&self) -> String;
}

impl HasDoIt for i32 {
    fn do_it(&self) {
        println!("I'm i32 and I'm doing it!");
    }
}

impl HasYeahYeah for i32 {
    fn yeah_yeah(&self) -> String {
        "Yeah Yeah i32!".to_owned()
    }
}

impl HasDoIt for &'static str {
    fn do_it(&self) {
        println!("I'm &str and I'm doing it!");
    }
}

impl HasDoIt for String {
    fn do_it(&self) {
        println!("I'm a big ole heap string and I'm doing it!");
    }
}

impl HasDoIt for f64 {
    fn do_it(&self) {
        println!("I'm a f64 and I'm doing it!");
    }
}

impl HasDoIt for u8 {
    fn do_it(&self) {
        println!("I'm an u8 that I made on accident and I'm doing it!");
    }
}

// √ create a method that takes a generic

// √ create a method that takes a genric bound by a trait

// √ create 2 structs or types that fill that role
// ...so structs can have traits, and types, structs are types...and...I am obviously
//    missing osme crucial concepts.

fn c1<T: HasDoIt>(thang: T) {
  thang.do_it();
  // what should I do with t
}

// create method that takes an utilizes two generics, 1 with a 1 trait, and the other with 2

// essentially fn foo<T: Thang, R: Thang + OtherThang>();

// create a trait has a method that returns a value
// ....wait or are traits only queries on internal state of the obj
// the trait is implemented for?


// wow its nice not having to think about a real life example.
//
// and I like this whole has thing, makes traits easier to comprehed
// ...but is it an actula convention? I just generally want to see more
// complex ways traits are used in real world examples.
fn c2<T: HasDoIt, Z: HasDoIt + HasYeahYeah>(thang1: T, thang2: Z) {
    thang1.do_it();
    thang2.do_it();
    println!("yeah yeah: {}", thang2.yeah_yeah());
}
