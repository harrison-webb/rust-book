fn main() {
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
}
