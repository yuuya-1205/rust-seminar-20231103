// 構造体を定義する
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // このようにメソッドを定義するとRect::new()で呼び出せる
    fn new(width: u32, height: u32) -> Self {
        Rect { width, height }
    }
}

// フィールドのない構造体も定義できる
struct NoField;

// タプル構造体
struct Point(u32, u32);

// これはNewTypeパターンと呼ばれるテクニック
struct UserName(String);

// 後述する列挙体
enum MyError {
    InvalidName,
}

// バリデーションに成功した場合のみ構造体のインスタンスをつくることができる
impl UserName {
    fn try_new(name: String) -> Result<Self, MyError> {
        if &name == "prn" {
            Err(MyError::InvalidName)
        } else {
            Ok(UserName(name))
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 5,
    };

    // Rect::newの方がシンプルに書ける
    let rect2 = Rect::new(10, 5);
}
