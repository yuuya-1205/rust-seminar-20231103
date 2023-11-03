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
    // 列挙体Aの高さはたかだか２
    let x = A::Child(3, B::Nil);
    let y = A::Nil;

    // Box<List>型
    //
    // データの実体はヒープに設置される
    // スタックにはそのヒープ領域へのポイントとメタデータが設置されるので、コンパイル時にサイズが確定する
    let l1 = Box::new(List::Nil);

    // 注: これはList型
    let l2 = List::Cons(3, l1);

    // 4 -> 10 -> 8 のリスト
    let l3 = List::Cons(
        4,
        Box::new(List::Cons(10, Box::new(List::Cons(8, Box::new(List::Nil))))),
    );
    // コードが長くて読みづらいのでヘルパ関数を定義したり、マクロを書いたりする。
    // マクロを自分で定義すれば、vec![4, 10, 8]のように list![4, 10, 8]と書ける。

    // おまけ
    //
    // Listをラップする構造体を定義して、その構造体にIteratorトレイトを実装してやれば自作ListをIterateできる
    let list_iter = ListIter(&l3);
    for (i, value) in list_iter.into_iter().enumerate() {
        println!("{}番目の要素は{}", i + 1, value);
    }
}

struct ListIter<'a>(&'a List);
impl<'a> Iterator for ListIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            List::Cons(inner, next) => {
                self.0 = next;
                Some(inner)
            }
            List::Nil => None,
        }
    }
}

fn notify<T: std::fmt::Display + std::fmt::Debug>(item: T) {}

fn notify2<T>(item: T)
where
    T: std::fmt::Display + std::fmt::Debug,
{
}

fn notify3<'a, T, U>(a: T, b: &'a U)
where
    T: IntoIterator,
    U: std::fmt::Display,
{
}
