use std::io;

fn main() {
   const MAX_POINTS: u32 = 100_000;

   let mut x = 5;
   println!("The value of x is {}", x);
   x = 6;
   println!("The value of x is {}", x);

   //shadowing
   let x = 5;
   let x = x + 1;
   let x = x + 2;

   println!("The value of x is: {}", x);


   // shadowing allow this
   let spaces = "    ";
   let spaces = spaces.len();

   // compile time error
   // let mut spaces = "    ";
   // spaces = spaces.len();

   // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 

   let x = 2.0; // f64

   let y: f32 = 3.0; // f32

   // addition
   let sum = 5 + 10;

   // subtraction
   let difference = 95.5 - 4.3;

   // multiplication
   let product = 4 * 30;

   // division
   let quotient = 56.7 / 32.2;

   // remainder
   let remainder = 43 % 5;

   let t = true;
   let f: bool = false;

   // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';

   // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

   // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

   let tup: (i32, f64, u8) = (500, 6.4, 1);
   let tup = (500, 6.4, 1);
   let (x, y, z) = tup;
   println!("The value of y is: {}", y);

   let x: (i32, f64, u8) = (500, 6.4, 1);
   let five_hundred = x.0;
   let six_point_four = x.1;
   let one = x.2;


   //Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.
   let a = [1, 2, 3, 4, 5];

   // Arrays are useful when you want your data allocated on the stack rather than the heap
   // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you should probably use a vector.
   let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
   // Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
   let a: [i32; 5] = [1, 2, 3, 4, 5];

   // if you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon
   let a = [3; 5];

}

fn displayValueAtIndex() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
