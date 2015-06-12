pub fn rep_one() {
    println!("Warm up 1!");

    c1(1);
    // c1("woah");
    // c1("fancy beat".to_string());
    // c1(232.44f64);
    // c1(b'a');
}

trait DoIt {
    fn do_it(&self);
}

impl HasDoIt for i32 {
    fn do_it(&self) {
        println!("I'm i32 and I'm doing it!");
    }
}

// create a method that takes a generic

// create a method that takes a genric bound by a trait

// create 2 structs or types that fill that role
// ...so structs can have traits, and types, structs are types...and...I am obviously
//    missing osme crucial concepts.

fn c1<T: HasDoIt>(thang: T) {
  thang.do_it();
  // what should I do with t
}

// what is something that I can do for all types
// well lets implement something for all these types

//...maybe start with 1
