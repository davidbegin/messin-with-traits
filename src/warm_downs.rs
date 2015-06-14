pub fn where_have_i_lost_my_mind() {
    // println!("Converted: {}", normal(&5));
    // println!("Converted: {}", inverse());

    // this doesn't compile, because i32 only has i64 implement for a conversion
    // let bad_inverse: i32 = inverse();

    // could I implement convert for i32 and this work?
    // let bad_inverse: i32 = inverse();
    // use_a_trait_with_defaults();
    use_a_trait_with_inheritance();
}

// I am defining a trait that takes a Generic
trait ConvertTo<Output> {

    // this is a method function because of the self
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}

// since this trait needs a type,
// can we not use it without declaring its type?
//
// that makes sense to me.
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

// so by using Where, I can add trait constraits to a specific
// type and not just a Generic?

// This inverse example is not making much sense to me
//
fn inverse<T>() -> T
        where i32: ConvertTo<T> {
   1i32.convert()
}

// these two comments are in the Rust Book,
// and they are confusing to me.
// can be called with T == i64
// this is using ConvertTo as if it were "ConvertFrom<i32>"

// I think the T == i64,
// means this can only be called, if i32 which is used
// inside the function implements ConvertTo

// So if this function is supposed to convert to i64 it will work
// but if we put it in a function where it doesn't
// ...then it will break?


// well this created another problem!
//
// when I only had conversions to 1 type,
// the compiler didn't complain using inverse inline
// because it knew it could be one type
//
// now my previous code won't compile until
// explictly set the type of variables being bound from
// the result of inverse
// impl ConvertTo<i32> for i32 {
//     fn convert(&self) -> i32 {
//         *self as i32
//     }
// }

// default methods

trait WithDefaults {
    fn you_need_to_implement_this(&self);

    fn default_for_free(&self) {
        println!("You get this method for free!");
    }
}

impl WithDefaults for i32 {
    fn you_need_to_implement_this(&self) {
        println!("this was implementing custom for i32");
    }
}

fn use_a_trait_with_defaults() {
    1.default_for_free();
    10i64.default_for_free();
}

impl WithDefaults for i64 {
    fn you_need_to_implement_this(&self) {
        println!("i63 needed to implement this");
    }

    fn default_for_free(&self) {
        println!("i64 overoad this even though it didn't need to");
    }
}

// inheritance in traits

fn use_a_trait_with_inheritance() {
    println!("Inheritance examples");
    let person1 = Person { name: "Steve".to_owned() };
    name_printer(person1);
    // let person2 = Person2 { name: "Bill" };
    // name_printer2(person2)
    // fancy_name_printer(person2);
}


// lets try making a trait of  HasName
// and then a trait that requires that trait call FancyHasName

struct Person {
    name: String
}

impl HasName for Person {
    fn name(self) -> String {
        self.name
    }
}

trait HasName {
    fn name(self) -> String;
}

fn name_printer<T: HasName>(x: T) {
    println!("Name: {}", x.name());
}

// fn name_printer2<T: HasName2>(x: T) {
//     println!("Name: {}", x.name());
// }

fn fancy_name_printer<T: FancyName>(x: T) {
    println!("Fancy Name: {:?}", x.fancy_name());
}

trait FancyName : HasName {
    fn fancy_name(self) -> String;
}

// struct Person2 {
//     name: str,
// }
//
// trait HasName2 {
//     fn name(&self) -> str;
// }

// So I can't have a function return a str
// unless I know exactly what the size will be at compile time
//
// how is this possible for a method
// impl HasName2 for Person2 {
//     fn name(&self) -> str {
//         self.name
//     }
// }

// impl FancyName for Person2 {
//     fn fancy_name(self) -> str {
//         // let mut title = "Mr.".to_owned();
//         // let name = "Bill".to_owned();
//         // title.push_str(name)
//
//         // "Mr. ".to_owned() + self.name
//         // "Mr. " + self.name
//         // self.name
//     }
// }

// I need to learn more about these two behaviors
// the trait `core::marker::Sized` is not implemented for the type `str` [E0277]
// note: `str` does not have a constant size known at compile-time
















