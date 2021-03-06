pub fn journey_into_the_unknown() {
    println!("Journey into the Unknown!");

    let z = 1u8;
    let j = "Dolpeedurf".to_string();

    // This example
    do_something(z);
    do_something(j);

    println!("
         \n\n<=====================================>
         Hold back I'm about to go dynamic!
         <=====================================>\n\n
    ");

    let k = 32u8;
    dynamic_dispatch_in_yo_face(&k as &Foo);

    let s = "Remy".to_string();
    dynamic_dispatch_in_yo_face(&s as &Foo);

    // Won't compile, because this is not implementing
    // traits dynamically
    //
    // ...value is something else
    //
    // ...but what
    // let u = 9.4f64;
    // dynamic_dispatch_in_yo_face(&u as &Foo);
}

// straight from the book

trait Foo {
    fn method(&self) -> String;
}

// Static Dispatch
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("String: {}", *self)
    }
}

// Woah so this compiles to two methods!
fn do_something<T: Foo>(x: T) {
    println!("the result of method: {}", x.method());
}

// Type Erasure!?!?
//
// dang this static dispatch section is making my brain dizzy

// I need to hang my hat on something
// So can I say if I see this pattern: &Foo or Box<Foo>
//
// then that means dynamic dispatch is involved


// COPYPASTE BECAUSE I NEED TO KEEP READING IT
// A trait object can be obtained from a pointer to a concrete type
// that implements the trait by casting it (e.g. &x as &Foo)
// or coercing it (e.g. using &x as an argument to a function that takes &Foo).

// For some reason is this really hard to reason about,
//
// I think maybe an example of each could help
// and there they are!
//
// So I think maybe just this order could be fixed to just show
// the example after each example.

// I think that is easier for me, but is it easier for everyone.

// So I am saying this variable x passed, needs to be something
// that can be coerced into into the the type Foo,
// which implements our Trait?!?!?!
//
// I have no clue!

// So this compiles so easily!
//
// because we are back in my world of dynamicness!
//
fn dynamic_dispatch_in_yo_face(x: &Foo) {
    println!("dynamic dispatch in action: {}", x.method());
}

// Another Copy Pasta I need to read over and over again
//
// A function that takes a trait object is not specialized to each of the types that implements
// Foo: only one copy is generated, often (but not always) resulting in less code bloat. However,
// this comes at the cost of requiring slower virtual function calls, and effectively inhibiting
// any chance of inlining and related optimizations from occurring.
