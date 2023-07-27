fn main() {
    //////////////////////////////////////////
    // 5.1 -- Structs

    // Defining a struct:
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    struct PlaneTicket {
        seat: String,
        boarding_group: u8,
        is_precheck: bool,
        name: String,
    }

    // Making an instance of a struct:
    let mut user1 = User {
        active: true,
        username: String::from("babybean"),
        email: String::from("babyboy@gmail.com"),
        sign_in_count: 5,
    };

    let bob_ticket = PlaneTicket {
        seat: String::from("5b"),
        boarding_group: 3,
        is_precheck: false,
        name: String::from("Bob McGee"),
    };

    // Accessing a struct's values
    user1.email = String::from("newemail@gmail.com");

    let bobs_seat = bob_ticket.seat;

    // A struct is either entirely mutable or entirely immutable.
    // You can not mark only certain fields as mutable

    // Field Init Shorthand
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        } // Note there is not semicolon, so this struct gets returned implicitly.
          // Also,
          //    username: username,
          //    email: email,
          // can be shortened to
          //    username,
          //    email,
    }

    // Struct Update syntax
    let user2 = User {
        email: String::from("myemail@email.net"),
        ..user1
    };
    // note! since user1 had a string, and that string has now been moved to user2, user1 can not be used as a whole anymore

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs
    // A struct with no fields
    // Dont understand what these are for yet
    // "useful when you want to implement a trait on some type, but don't have any data that you want to store in the type itself"
    struct SomethingOrAnother;
    let this_or_that = SomethingOrAnother;

    /////////////////////////////////////////////////////////
    // 5.3 -- Method Syntax

    // methods are like functions (defined w/ fn, have parameters, return values), but are defined within the context of a struct/enum/trait object
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Everything within this impl block is associated with the Rectangle type
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, rect: &Rectangle) -> bool {
            self.width > rect.width && self.height > rect.height
        }

        // This is an "associated function" because it is defined in the impl block, but doesn't have &self as the first parameter
        // It is basically a function that acts as a "constructor" for a square (aka a Rectangle with width == height)
        // "Self" anywhere within an impl block refers to the type that comes after the impl keyword, so Rectangle here
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", rec1.area());

    // To call associated functions, use the :: syntax
    let my_square = Rectangle::square(5);
}
