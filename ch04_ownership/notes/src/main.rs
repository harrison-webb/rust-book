fn main() {
    /*
    OWNERSHIP
    set of rules that govern how a rust program manages memory

        STACK AND HEAP
        parts of memory available to code to use at runtime
            stack:
                - lifo
                - data stored on stack must have known, fixed size
                - faster than heap
            heap:
                - "blob", not organized contiguously like the stack
                - stores data with unknown size @ compile time, or data with size that can change
                - when putting data on heap, memory allocater finds empty memory slot big enough & returns a pointer to the address
                - this pointer gets stored on the stack, but must follow pointer to retrieve data on heap

        THE RULES
            - each value in rust has an owner
            - there can only be one owner at a time
            - when the owner goes out of scope, the value will be dropped

        THE 'String' TYPE
            - unlike string literals, which are fixed and stored on stack, String type is stored on heap (so it's more flexible)
            - String can be mutated, where string literals can not.
            EX:*/
    let mut s = String::from("hello");
    s.push_str(" world"); //appends world to s

    /*
    when a variable that owns memory goes out of scope, the memory is automatically deallocated
    to ensure memory safety, when we have something like:
        let s1 = String::from("hello");
        let s2 = s1;
    rust considers s1 as no longer valid. this is because s1 and s2 both point at the same memory on the heap,
      so when s1 and s2 go out of scope, there would be a double free error from both strings trying to deallocate the same memory.

    this is similar to a shallow copy in other languages, but becuase the first variable is invalidated, it is called a *move*

    to perform a DEEP COPY, the clone method can be used. EX: let s2 = s.clone(); */

    /*
    REFERENCES AND BORROWING
    to give an object to some new scope (like a function) without having to return it from the funtion in order to use it again,
      we can give the function a reference (using &) to the object instead. this is called borrowing
    however, you can not modify something you are borrowing!
      unless you use a mutable reference. Ex: "&mut String"
    if you have a mutable reference to a value, you can not have any other references to that value

    note: while you can only reference a value once if using a mutable reference, this does not apply for immutable references


    SLICES
    slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
    a slice is a kind of reference, so it does not have ownership*/
}
