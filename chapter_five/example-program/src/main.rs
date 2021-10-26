//An Example Program Using Structs
//To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. We’ll start with single variables, and then refactor the program until we’re using structs instead.

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// => The area of the rectangle is 1500 square pixels.

//Even though Listing 5-8 works and figures out the area of the rectangle by calling the area function with each dimension, we can do better. The width and the height are related to each other because together they describe one rectangle.

//The issue with this code is evident in the signature of area:

fn area(width: u32, height: u32) -> u32 {}

//The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters. The parameters are related, but that’s not expressed anywhere in our program. It would be more readable and more manageable to group width and height together. We’ve already discussed one way we might do that in “The Tuple Type” section of Chapter 3: by using tuples.

//Refactoring with Tuples
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: tuples don’t name their elements, so our calculation has become more confusing because we have to index into the parts of the tuple.

//It doesn’t matter if we mix up width and height for the area calculation, but if we want to draw the rectangle on the screen, it would matter! We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. If someone else worked on this code, they would have to figure this out and keep it in mind as well. It would be easy to forget or mix up these values and cause errors, because we haven’t conveyed the meaning of our data in our code.

//Refactoring with Structs: Adding More Meaning
//We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a data type with a name for the whole as well as names for the parts.

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//Here we’ve defined a struct and named it Rectangle. Inside the curly brackets, we defined the fields as width and height, both of which have type u32. Then in main, we created a particular instance of Rectangle that has a width of 30 and a height of 50.
//Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
//The area function accesses the width and height fields of the Rectangle instance. Our function signature for area now says exactly what we mean: calculate the area of Rectangle, using its width and height fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity.

//Adding Useful Functionality with Derived Traits
//
