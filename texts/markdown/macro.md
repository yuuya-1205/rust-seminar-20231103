# マクロ
println!はマクロ。Do you know メタプログラミング？

RustのマクロはCの`#define`ような単なる文字列の置換では無い。安全に使える。

- 宣言的（declarative）マクロ
- ３種類の手続き的 (procedural) マクロ
    - deriveマクロ
    - 属性マクロ
    - 関数マクロ
- 凝ったものを作ると黒魔術しがち
    - RustのAST（抽象構文木: Abstract Syntax Tree）の知識が必要になる場合がある

## よく使うマクロ

- println!(): 引数を標準出力に出力
- format!(): 文字列のフォーマット
- assert_eq!(): テストで使う。第１引数と第２引数が不一致だとpanicする。
- vec![]: Vec型の値を生成する。
- #[derive()]: 型にトレイトの標準的な実装をする。`#[derive(Debug)]`など
- panic!(): パニックする
- todo!(): これを書いておくと、そのブロックの型解析をパスできる。
- unimplemented!(): 挙動は`todo!`と同じ。
- #[tokio::main]: プロジェクトを非同期処理に対応させる。