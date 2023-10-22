// 列挙型（enum）は取りうる値をすべて列挙したもの
// このときのV4、V6は列挙子と呼ぶ
enum IpAddrKind {
    V4,
    V6,
}

// 列挙子にはデータを紐づけることもできる
// ```
// let v4 = IpAddr::V4(192, 168, 1, 1);
// ```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// このenumには、異なる型の列挙子が4つ
//  1. Quitには紐付けられたデータは全くなし。
//  2. Moveは、中に匿名構造体を含む。
//  3. Writeは、単独のStringオブジェクトを含む。
//  4. ChangeColorは、3つのi32値を含む。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// もちろん構造体を紐づけることも可能
enum Shape {
    Rect(Rect),
    Circle(Circle),
}

struct Rect {
    width: u32,
    height: u32,
}

struct Circle {
    radius: u32,
}

fn main() {
    let v4 = IpAddr::V4(192, 168, 1, 1);
}
