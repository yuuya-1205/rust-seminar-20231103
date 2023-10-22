fn main() {
    // タプルは、丸かっこの中にカンマ区切りの値リストを書くことで生成する
    // タプルの位置ごとに型があり、 タプル内の値はそれぞれ全てが同じ型である必要はない
    let point = (4, 5);

    // アクセスしたい値の番号をピリオド(.)に続けて書くことで、 タプルの要素に直接アクセスできる
    assert_eq!(point.0, 4);
    assert_eq!(point.1, 5);

    // パターンマッチングで分配することもできる。
    let (x, y) = point;
    assert_eq!(x, 4);
    assert_eq!(y, 5);

    // タブル内の要素の型がバラバラのパターン
    let t1 = (1, String::from("a"), false);

    assert_eq!(t1.0, 1);
    assert_eq!(t1.1, String::from("a"));
    assert_eq!(t1.2, false);
}
