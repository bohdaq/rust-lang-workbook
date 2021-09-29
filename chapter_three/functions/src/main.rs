// Function bodies are made up of a series of statements optionally ending in an expression. 
// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. 
// Creating a variable and assigning a value to it with the let keyword is a statement. In Listing 3-1, let y = 6; is a statement.
// Function definitions are also statements; 
// Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.


fn main() {
    println!("Hello, world!");

    const x:i32 = 5;
    let y = 6;
    another_function(x, y);

    let y = five();
    println!("The value of x is {}", y);
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("Another function argument values are {} and {}", x, y);
}


