fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }



    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //When this program executes, it checks each if expression in turn and executes the first body for which the condition holds true. Note that even though 6 is divisible by 2, we don’t see the output number is divisible by 2, nor do we see the number is not divisible by 4, 3, or 2 text from the else block. That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.


    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    // Because if is an expression, we can use it on the right side of a let statement
    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
    // This means the values that have the potential to be results from each arm of the if must be the same type;


}
