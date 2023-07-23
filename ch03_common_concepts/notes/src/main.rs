fn main() {
    //////////////////////////////////////////
    // VARIABLES AND MUTABILITY
    //
    /*
    variables are immutable by default, use "mut" keyword to make them mutable
     */
    //ex:
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6; //-> we would like to simply reassign x, but can not because x has not been declared mutable
    // println!("The value of x is: {x}");

    //fixed:
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // this works becuase we have declared x to be mutable

    /*
    CONSTANTS
    constants are declared with the *const* keyword and are ALWAYS immutable
    the type also must be annotated
    naming convention is to use ALL_CAPS_WITH_UNDERSCORES */
    const EXAMPLE_CONSTANT: u32 = 24 * 365;
    println!("EXAMPLE_CONSTANT: {EXAMPLE_CONSTANT}");

    /*
    SHADOWING
    shadowing is declaring a new variable with the same name as a previous variable
     */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Value of shadowed spaces variable: {spaces}");

    //////////////////////////////////////////
    // DATA TYPES
    //
    /*
    every value in rust has a specific data dype
    rust is statically typed, but the compiler can very often infer the type of a variable*/

    /*
    SCALAR TYPES
    scalar type variables represent a single value
    four primary scalar types: integers, floating-point numbers, booleans, and characters

    Integers:
    signed integers can be positive or negative
    unsigned integers are always positive

    signed  unsigned
    i8      u8
    i16     u16
    i32     u32
    i64     u64
    i128    u128
    isize   usize

    isize/usize depend on the architecture of the computer the program is running on, 64-bit or 32-bit

    a good default is i32 or u32


    Floating point:
    all floats are signed
    two options: f32 and f64
    roughly same speed, f64 has more precision
    f32 is signe-precision float, f64 is double-precision


    Booleans:
    one byte in size
    true and false


    Chars:
    most primitive alphabetic type
    declared with single quotes
    four bytes in size, represents Unicode Scalar Value (many more values than ASCII)


    /*
    COMPOUND TYPES
    compound types can group multiple values into one type
    two primitive compound types: tuples and arrays

    Tuples:
    fixed length once declared
    elements can be accessed with tupleName.<index>
    a tuple with no values is called a unit. written (). expressions implicitly return a unit if they don't return anything else
    */
    */
    let my_tup = (1, 2, 3);
    let (x, y, z) = my_tup;
    println!("The second element of my_tup is: {y}");
    println!("We can also access it with my_tup.1: {}", my_tup.1);

    /*
    Arrays:
    unlike tuples, every element of an array needs to have the same type
    arrays have fixed length
    "If you're unsure whether to use an array or a vector, chances are you should use a vector."
    allocates on the stack
     */
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; //equivalent to [3, 3, 3, 3, 3]

    //////////////////////////////////////////
    // FUNCTIONS
    //
    /*
    declared with fn keyword
    function names are snake_case
    type annotations of arguments are required
    */
    fn function_with_parameter(x: i32) {
        println!("Your argument is {x}");
    }

    /*
    Expressions and statements:
    statement -> does not return anything, ends with semicolon. Example: "let x = 5;"
    expression -> returns a value, does not end with a semicolon
    Expression examples:
    6
    5+5
    *calling a function*
    *calling a macro* */

    /*
    Functions with return values:
    functions can return values to the code that calls them
    return values are not named, but their type is indicated
    you can return early from a function with the return keyword, otherwise the function will return the last expression implicitly
     */
    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    println!("plus_one(6): {}", plus_one(6));

    //////////////////////////////////////////
    // CONTROL FLOW
    //
    /*
    if expressions:
    condition must evaluate to a bool
    rust will not automatically convert non-bool types to a bool */
    fn if_demo(num: i32) {
        if num < 100 {
            println!("The number is less than 100");
        } else if num > 100 {
            println!("The number is greater than 100");
        } else {
            println!("The number is 100");
        }
    }

    // if is an expression, therefore you can use it on the right side of a let statement to assign the outcome to a variable
    fn if_assignment(num: i32) -> &'static str {
        let result = if num % 2 == 0 { "even" } else { "odd" };
        result
    }
    println!("if_assignment(5) returns: {}", if_assignment(5));

    /*
    Loops:
    loop -> tells block of code to repeat until explicitly told to stop
        use break to exit loop, or continue to skip over remaining code for "this" loop and start the next loop
        to return a value from loop, add it after "break" */
    fn loop_demo() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result of loop_demo is {result}");
    }
    loop_demo();

    /*
    For loops:
     */
    fn for_demo() {
        let a = [1, 2, 3, 4, 5];

        for item in a {
            println!("for_demo(): the value is: {item}");
        }
    }
    for_demo();

    fn countdown() {
        for number in (1..=5).rev() {
            println!("{number}");
        }
        println!("Liftoff!");
    }
    countdown();
}
