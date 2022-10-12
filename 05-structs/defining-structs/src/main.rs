struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("user1@mail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    let name1 = user1.username;
    user1.username = String::from("new_user1");

    // with the function constructor
    let user2 = build_user(String::from("user2@mail.com"), String::from("user2"));

    // reusing instance data
    let user3 = User {
        email: String::from("user3@mail.com"),
        username: String::from("user3"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 1,
    }
}
