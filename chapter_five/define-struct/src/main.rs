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
//
