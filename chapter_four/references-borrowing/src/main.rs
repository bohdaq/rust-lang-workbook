// References and Borrowing
}    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

//The issue with the tuple code is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length.
//
//Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String.
//
//These ampersands are references, and they allow you to refer to some value without taking ownership of it.
//Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.
//
//Let’s take a closer look at the function call here:

let s1 = String::from("hello");
let len = calculate_length(&s1);

//The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used.
//Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference. Let’s add some explanatory annotations:

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

//The scope in which the variable s is valid is the same as any function parameter’s scope, but we don’t drop what the reference points to when s stops being used because we don’t have ownership. When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

//We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

//So what happens if we try to modify something we’re borrowing? Spoiler alert: it doesn’t work!

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

//cannot borrow `*some_string` as mutable, as it is behind a `&` reference --> src/main.rs:8:5
//Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.


//Mutable References
//We can fix the error in the code with just a few small tweaks:
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//First, we had to change s to be mut. Then we had to create a mutable reference with &mut s where we call the change function, and update the function signature to accept a mutable reference with some_string: &mut String. This makes it very clear that the change function will mutate the value it borrows.
//But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. This code will fail:
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

//error[E0499]: cannot borrow `s` as mutable more than once at a time --> src/main.rs:5:14

//This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.
//The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like.
//The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
//   Two or more pointers access the same data at the same time.
//   At least one of the pointers is being used to write to the data.
//   There’s no mechanism being used to synchronize access to the data.

//Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!
//As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

//A similar rule exists for combining mutable and immutable references. This code results in an error:
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

//error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
//Whew! We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them! However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

//Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}



