struct User {
    id: String,
    name: String,
}

fn main() {
    let maybe_exist = Some(User {
        id: String::from("000"),
        name: String::from("Ferris"),
    });

    // match 式を使ったパターンマッチング
    match maybe_exist {
        Some(ref user) => {
            println!("Found User!!");
            println!("User name: {}", user.name);
        }
        None => println!("Not Found"),
    };

    // if let Some()
    if let Some(ref user) = &maybe_exist {
        println!("Found User!!");
        println!("User name: {}", user.name);
    } else {
        println!("Not Found")
    };

    let id = "001";
    // Option::map()でSome()のときのみ中身の値を操作する
    // ex. user_name: Stringから User インスタンスに変換する
    let user = Db::get_user_name("001").map(|user_name| User {
        id: id.to_string(),
        name: user_name,
    });
}

struct Db;

impl Db {
    fn get_user_name(id: &str) -> Option<String> {
        if id == "001" {
            Some(String::from("Alice"))
        } else {
            None
        }
    }
}
