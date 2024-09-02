struct User { // Struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // Tuple Structs
struct Point(i32, i32, i32);

struct AlwaysEqual; // Unit Struct

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    /*
    let user2 = User { // User 3 is a more efficient example
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // must come last to specify that any remaining fields should get their values from the corresponding fields in user1
    };

    // we can no longer use user1 as a whole after creating user3 because the String in the username field of user1 was moved into user3.

    user1.email = String::from("anotheremail@example.com");

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // this would set the username and email to the args provided in the functions parameters because the fields and arguments have the same names
        email,
        sign_in_count: 1,
    }
}
