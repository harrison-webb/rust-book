fn main() {
    ///////////////////////////////////////
    // Chapter 7 -- Managing growing projects with packages, crates, and modules

    /*
    - As projects grow, it is a good idea to organize different parts into different
       modules and then different files
    - Rust has a number of features that allow you to manage code organization, collectively
        called the "module system"
        - packages -> cargo feature that lets you build, test, and share crates
        - crates -> tree of modules that produces a library or executable
        - modules and use -> let you control the organization, scope, and privacy of paths
        - paths -> way of naming an item, such as a struct function, or module
     */

    ///////////////////////////////////////
    // 7.1 -- Packages and Crates
    /*
    A crate is the smallest amount of code that the rust compiler considers at a time.
    A crate can be a binary or a library.
        - binary crate -> programs that compile to an executable, such as a command line program or server
            - must have a main function
        - library crate -> define functionality intended to be shared with other projects
            - no main, no executable. Ex: rand crate

    A package is a bundle of one or more crates that provides a set of functionality
        - contains a cargo.toml file that describes how to build those crates
        - a package can contain multiple binary crates, but only 0 or 1 library crate */
}
