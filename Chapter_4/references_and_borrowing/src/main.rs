fn main() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    change(&mut s);

    let mut v = String::from("Hello");

    /* 
    let r1 = &mut v;
    let r2 = &mut v;        This doesn't work because you cannot have 2 mutable references to the same variable at once to prevent data races
    */

    let r1 = &v; // np ez
    let r2 = &v; // np ez
    //let r3 = &mut v; // PROBLEM!!!! You cannot have both a mutable and immutable reference at the same time

    println!("{r1} & {r2}"); // r1 and r2 are dead after this point

    let reference_to_nothing = dangle();

}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn calculate_len(s: &String) -> usize { // this is called borrowing, you cannot modify a variable whilst it is being borrowed unless if the reference is mutable
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s // error: returns a reference to data owned by the current function
} // Here, s goes out of scope, and is dropped. Its memory goes away. That's not very good.
