type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

fn main() {
    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("yaffi"), id, now);
}