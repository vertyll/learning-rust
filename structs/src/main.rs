// struktura

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struktura krotkowa

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// struktura jednostkowa

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("jakiś username"),
        email: String::from("test@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("email@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let email = String::from("test");
    let username = String::from("test");

    build_user(email, username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
