fn main() {
    let x = 5;
    let y = x;

    println!("X: {x}\nY: {y}");

    /*
    let s1 = String::from("hello"); // this is called a move V
    let s2 = s1;

    //println!("{s1}, world!"); // This does not work because rust declares s1 invalid now that it's moved to s2 to provent double free error
    */

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("Hello");

    takes_ownership(s); //Strings do not have a copy trait so the string is dead after this

    let x = 5;

    makes_copy(x); //x is an i32 so it has the Copy trait so its safe to use afterwards

    let v1 = gives_ownership();         // gives_ownership moves its return value into s1

    let v2 = String::from("hello");     // s2 comes into scope

    let v3 = takes_and_gives_back(v2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function 
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

/* 

        ** COPY TRAIT TYPES **

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

*/

/*

    ** STACK **

    The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Adding data is called pushing onto the stack, and removing data is called popping off the stack. 
    All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
    Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
    Types such as integers that have a known size at compile time are stored entirely on the stack

    ** HEAP **

    The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. 
    This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). 
    Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
    Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.

    ** OWNERSHIP RULES **

    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.

*/