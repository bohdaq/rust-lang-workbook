//Using Structs to Structure Related Data
//A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes. In this chapter, we’ll compare and contrast tuples with structs. We’ll demonstrate how to define and instantiate structs. We’ll discuss how to define associated functions, especially the kind of associated functions called methods, to specify behavior associated with a struct type. Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile time type checking.

//Defining and Instantiating Structs
//Structs are similar to tuples, which were discussed in “The Tuple Type” section. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
//To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. For example, Listing 5-1 shows a struct that stores information about a user account.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. We don’t have to specify the fields in the same order in which we declared them in the struct. In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type. For example, we can declare a particular user as shown in Listing 5-2.

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
//Listing 5-2: Creating an instance of the User struct

//To get a specific value from a struct, we can use dot notation. If we wanted just this user’s email address, we could use user1.email wherever we wanted to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Listing 5-3 shows how to change the value in the email field of a mutable User instance.

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
//Listing 5-3: Changing the value in the email field of a User instance

//Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

//Listing 5-4 shows a build_user function that returns a User instance with the given email and username. The active field gets the value of true, and the sign_in_count gets a value of 1.

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
//Listing 5-4: A build_user function that takes an email and username and returns a User instance

//It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the email and username field names and variables is a bit tedious. If the struct had more fields, repeating each name would get even more annoying. Luckily, there’s a convenient shorthand!

//Using the Field Init Shorthand when Variables and Fields Have the Same Name
//Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of email and username, as shown in Listing 5-5.

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
//Listing 5-5: A build_user function that uses field init shorthand because the email and username parameters have the same name as struct fields

//Here, we’re creating a new instance of the User struct, which has a field named email. We want to set the email field’s value to the value in the email parameter of the build_user function. Because the email field and the email parameter have the same name, we only need to write email rather than email: email.

//Creating Instances From Other Instances With Struct Update Syntax
//It’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some. You can do this using struct update syntax.
//First, Listing 5-6 shows how we create a new User instance in user2 without the update syntax. We set a new value for email but otherwise use the same values from user1 that we created in Listing 5-2.

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
//Listing 5-6: Creating a new User instance using one of the values from user1

//Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
//Listing 5-7: Using struct update syntax to set a new email value for a User instance but use the rest of the values from user1

//The code in Listing 5-7 also creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1. The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

//Note that the struct update syntax is like assignment with = because it moves the data, just as we saw in the “Ways Variables and Data Interact: Move” section. In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. The types of active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.


//Using Tuple Structs without Named Fields to Create Different Types
//You can also define structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.
//To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple. For example, here are definitions and usages of two tuple structs named Color and Point:

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

//Note that the black and origin values are different types, because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces, you can use a . followed by the index to access an individual value, and so on.

//Unit-Like Structs Without Any Fields
//You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself. We’ll discuss traits in Chapter 10. Here’s an example of declaring and instantiating a unit struct named AlwaysEqual:

    struct AlwaysEqual;

    let subject = AlwaysEqual;

//To define AlwaysEqual, we use the struct keyword, the name we want, then a semicolon. No need for curly brackets or parentheses! Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine we’ll be implementing behavior for this type that every instance is always equal to every instance of every other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.

//Ownership of Struct Data
//In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
//It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:

struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

//error[E0106]: missing lifetime specifier


//In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs, but for now, we’ll fix errors like these using owned types like String instead of references like &str.
