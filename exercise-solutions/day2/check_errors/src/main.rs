#[allow(dead_code)]
struct User {
    username: String,
    first: String,
    last: String,
    uid: u32,
    gid: u32,
}

fn create_user(first: String, last: String) -> User {
    let username = first[..1].to_string() + &last;
    User {
        first,
        last,
        username,
        uid: 0,
        gid: 0
    }
}

fn main() {
    let mut user1 = create_user(String::from("foo"), String::from("bar"));
    user1.uid = 1000;
    println!("user1 = {}", user1.username);

    let mut user2 = user1;
    user2.username = String::from("baz");
    println!("user2 = {}", user2.username);
}
