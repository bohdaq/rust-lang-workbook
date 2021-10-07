// Rust’s central feature is ownership. 
// All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; 
// in other languages, the programmer must explicitly allocate and free the memory.
// Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
// None of the ownership features slow down your program while it’s running.




// The Stack and the Heap
// In many programming languages, you don’t have to think about the stack and the heap very often. 
// But in a systems programming language like Rust, whether a value is on the stack or the heap has more of an effect on how the language behaves and why you have to make certain decisions.
// Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways. 
// The stack stores values in the order it gets them and removes the values in the opposite order.
// This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top.
// Adding data is called pushing onto the stack, and removing data is called popping off the stack.

// All data stored on the stack must have a known, fixed size.
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
// The heap is less organized: when you put data on the heap, you request a certain amount of space.
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
// This process is called allocating on the heap and is sometimes abbreviated as just allocating.
// Pushing values onto the stack is not considered allocating. Because the pointer is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.


// Ownership Rules
// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
//  - Each value in Rust has a variable that’s called its owner.
//  - There can only be one owner at a time.
//  - When the owner goes out of scope, the value will be dropped.


// Variable Scope
// As a first example of ownership, we’ll look at the scope of some variables. A scope is the range within a program for which an item is valid. Let’s say we have a variable that looks like this:
fn main() {
                        // s is not valid here, it’s not yet declared
    let s = "hello";    // s is valid from this point forward
    // do stuff with s
}                       // this scope is now over, and s is no longer valid

//In other words, there are two important points in time here:
//    When s comes into scope, it is valid.
//    It remains valid until it goes out of scope.

//At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we’ll build on top of this understanding by introducing the String type.


//The String Type
//To illustrate the rules of ownership, we need a data type that is more complex than the ones we covered in the “Data Types” section of Chapter 3. The types covered previously are all stored on the stack and popped off the stack when their scope is over, but we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data.





