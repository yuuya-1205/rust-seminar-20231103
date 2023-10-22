struct UserId(String);
struct UserName(String);

struct UserView {
    id: String,
    name: String,
    age: u32,
}

enum MyError {
    DatabaseError,
    ConvertError,
}

struct Payload {
    users: UserPayload,
}

struct UserPayload {
    id: String,
    name: Option<String>,
    age: Option<String>,
}
// TODO: collect::<Result<Vec<T>, E>>の紹介する
// cf.   collect::<Vec<Result<T, E>>>
fn get_users() -> Result<Vec<UserView>, MyError> {
    // 取得して
    // コマンド組み立てる
    // .map_err()?.into_iter().filter().map().collect::<Result<Vec<_>,_>>()?
    todo!()
}

mod Repository {
    pub enum DbError {}
    pub struct User {}

    pub fn fetch_users() -> Result<Vec<User>, DbError> {
        todo!()
    }

    pub fn persist_users() -> Result<(), DbError> {
        todo!()
    }
}

fn main() {}
