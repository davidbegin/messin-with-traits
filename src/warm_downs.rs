pub fn where_have_i_lost_my_mind() {
    println!("Converted: {}", normal(&5));
    println!("Converted: {}", inverse());

    // this doesn't compile, because i32 only has i64 implement for a conversion
    // let bad_inverse: i32 = inverse();

    // could I implement convert for i32 and this work?
    // let bad_inverse: i32 = inverse();
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
