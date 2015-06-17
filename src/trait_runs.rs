pub fn examples() {
    println!("I need challenges to and practice and hone my trait skills");
    animals();
}

fn animals() {
    let fido = Dog { name: "Fido" };
    let boots = Cat { name: "Boots" };
    animal_whisperer(&fido);
    animal_whisperer(&boots);

    let basel = Snake { name: "Basel" };
    trick_challenge(&basel);
}

fn trick_challenge<T>(animal: &T)
    where T: HasSpeak + HasName + HasSpecialTalent {
    animal.speak();
    let name = animal.name();
    println!("The Animals name is: {}", name);
    animal.special_talent();
}

fn animal_whisperer<T: HasSpeak>(animal: &T) {
    animal.speak();
}

trait HasName {
    fn name(&self) -> String;
}

trait HasSpecialTalent {
    fn special_talent(&self);
}

trait HasSpeak {
    fn speak(&self);
}

struct Snake {
    name: &'static str,
}

impl HasSpeak for Snake {
    fn speak(&self) {
        println!("Hiss Hiss");
    }
}

impl HasName for Snake {
    fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl HasSpecialTalent for Snake {
    fn special_talent(&self) {
        println!("**...feeds on your fear...**");
    }
}

struct Cat {
    name: &'static str,
}

impl HasSpeak for Cat {
    fn speak(&self) {
        println!("Meow Meow");
    }
}

struct Dog {
    name: &'static str,
}

impl HasSpeak for Dog {
    fn speak(&self) {
        println!("Woaf Woaf");
    }
}

// The Shape is the first example in the Trait Book
//
// HasArea is the trait
//
// I need more examples like this
//
// So here are some cheesy examples

// Pets, 2 traits Speak and Play, then maybe Tricks



// So there are Structs, Traits and Methods that take Structs with Traits
//
// where does the construction start
//
// well we could just build structs and figure out what to do with them,
// or we could find a problem and solve it.
//
// Now I have no problems, but this does inform me the order I think
// I should go
//
// Function
//
// then Traits
//
// then Structs that implement that trait



// #1 ====================================================================
// fn animal_whisperer<T>(animal: &T) {
//     animal.speak();
// }

// So here I start with function I want,
// and want I want to call on the Generic Passed in
//
//
// And I get this compiler message
// no method named `speak` found for type `&T` in the current scope
//
// ...leading me naturally to the trait!
// =======================================================================

// #2 ====================================================================
// fn animal_whisperer<T: HasSpeak>(animal: &T) {
//     animal.speak();
// }
//
// trait HasSpeak {
//     fn speak(&self);
// }

// this gives us a happy compiler, but no one is using our function yet
// =======================================================================

// #3 ====================================================================
