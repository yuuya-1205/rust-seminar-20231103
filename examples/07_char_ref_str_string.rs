fn main() {
    // これはchar型。シングルクォーテーションマークで囲んだ１文字が文字リテラルになる。
    let a = 'a';

    // Rustのchar型は1バイトとは限らない
    let emoji = '🦀';

    // シングルクォーテーションマークで２文字以上の文字列を囲むとコンパイルエラー
    // let c = 'yeah';

    // 文字列リテラル（&str）を表現するにはダブルクォーテーションマークで囲む。
    // "&" が付いてるのでこれは参照。省略されているが、このときのライフタイムは'static。
    // すなわち、プログラム実行開始から終了までずっと生存する。もちろん'staticではないライフタイムの&strも存在する
    //
    // ELFやMach-Oなどのオブジェクトファイル形式について知ってる人向けに補足すると、
    // この場合の参照先は"静的領域"に配置されます。
    let s1 = "red";

    // "使いやすい"のはString型。Vecと同じで実体はヒープに置かれる。
    let s2 = String::from("blue");

    // &strから`to_string()`を使ってString型に変換できる。
    let mut s3 = "green".to_string();

    // 所有権を持ってるので変更できる
    s3.push_str(" and yellow");
    assert_eq!(&s3, "green and yellow");

    // 実際は可変にするよりformat!マクロを使うケースが多そう
    let g = String::from("green");
    let p = String::from("purple");
    let s4 = format!("{} and {}", g, p);
    assert_eq!(&s4, "green and purple");
}
