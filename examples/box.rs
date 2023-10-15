// リストを定義してみる

// List型のサイズがコンパイル時に確定しないのでコンパイルエラー
// リストの長さが１かもしれないし100かもしれない。メモリをどれだけ確保したらいいのか決められない。
// ex. List::Child(1, List::Child(2, List::Child(3, ...)))
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// これはコンパイル時にA型のサイズがわかるのでOK
// A::Child(3, B::Nil) が最大サイズ
enum A {
    Child(i32, B),
    Nil,
}

enum B {
    Nil,
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // 列挙体Aはたかだか長さ（深さ？）は２
    let x = A::Child(3, B::Nil);
    let y = A::Nil;

    // Box<List>型
    // データの実体はヒープに設置される
    // スタックにはそのヒープ領域へのポイントとメタデータが設置されるので、コンパイル時にサイズが確定する
    // TODO: スライド用意する
    let l1 = Box::new(List::Nil);

    // これはList型
    let l2 = List::Cons(3, l1);

    // 4 -> 10 -> 8
    let l3 = List::Cons(
        4,
        Box::new(List::Cons(10, Box::new(List::Cons(8, Box::new(List::Nil))))),
    );

    // Listを辿る
    let mut next;
    match l3 {
        List::Cons(n, l) => {
            println!("{:?}", n);
            next = l
        }
        List::Nil => return,
    };
    loop {
        match *next {
            List::Cons(n, l) => {
                println!("{:?}", n);
                next = l
            }
            List::Nil => break,
        }
    }
}
