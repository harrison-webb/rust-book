fn main() {
    //////////////////////////////
    // 6.1 -- Enums

    // Enums give you a way of saying a value is one of a possible set of values
    enum IpAddrKind {
        V4,
        V6,
    }
    // IpAddrKind is now a cutom *data type*!
    // To create an instance of an enum:
    let four = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {
        // stuff
    }

    route(four); // This function can be called with either variant of IpAddrKind
    route(six);

    // This IpAddrKind enum tells what kind an IP address is, but not any data about it
    // One option is to use a mix of enums and structs to associate data with an IP address:
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let ip_address_1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // But we can also accomplish this more concisely using just an enum
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    let ip_adddr_2 = IpAddr2::V4(String::from("155.97.3.10")); // This contains a enum variant as well as address data

    // Also, putting everything in an enum lets us vary the associated data for each variant:
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // another enum example
    enum Message {
        Quit,                       // no data associated with Quit variant
        Move { x: i32, y: i32 },    // named fields like a struct
        Write(String),              // Write variant has a single string as data
        ChangeColor(i32, i32, i32), // ChangeColor has three i32 values
    }

    impl Message {
        fn call(&self) {
            // stuff goes here
        }
    }

    let m = Message::Write(String::from("hello!"));
    m.call();

    // Option Enum
    // Encodes the scenario where a value could be something or it could be nothing
    // Defined in the standard library like this (it is actually just Option<T>):
    enum FakeOption<T> {
        None,
        Some(T),
    }

    let some_number = Some(4);
    let some_char = Some('b');
    let empty_number: Option<i32> = None;

    // With "regular" languages that have null, any type has the possibility of being null at any time (i think)
    // in Rust, values of type Option<T> are incompatible with values of type T, so this forces you to be extremely
    //   specific on the type-level about when a value could be null or not.
    // For example, a value of type i32, or char, or bool, will ALWAYS have a valid value, and only values with
    //   type Option<i32> or Option<char> or Option<bool> even have the possibility of containing an invalid value
    // "This helps catch one of the most common issues with null: assuming that something isn't null when it actually is"
}
