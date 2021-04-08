#[allow(dead_code)]
struct User {
    username: String,
    first: String
    last: String,
    uid: u32,
    gid: u32,
}

fn create_user(first: str, last: String) -> User {
    let username = String::from(first[..1] + last);
    User {
        first,
        last,
        username,
        uid: 0,
        gid: 0
    };
}


fn main() {
    let user1 = create_user("foo", "bar");
    user1.uid = 1000;
    println!("user1 = {}", user1.username);

    let user2 = user1;
    user2.username = "baz";
    println!("user2 = {}", user2.username);
}
