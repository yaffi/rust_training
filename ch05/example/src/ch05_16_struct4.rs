struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

fn main() {
    let id = Id(400);
    let now = Timestamp(4567890123);
    // 引数の順番がおかしいのでエラー
    // let bad_user = new_user(UserName(String::from("yaffi"), now, id);
    // error[E0308]: mismatched types
    // expected type `id`, found type `Timestamp`
}