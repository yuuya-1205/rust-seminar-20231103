# 自己紹介
[自己紹介](./self_introduce.md)

# Rustことはじめ
[rust](./rust.md)

# 環境構築

- install rustup, vscode, rust-analyzer
- Rustをローカルにインストールする
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`　をターミナルで実行するだけ
    - [https://www.rust-lang.org/ja/learn/get-started](https://www.rust-lang.org/ja/learn/get-started)
- エディタはVSCodeを使ってください
    - もちろん自分の使い慣れてるエディタを使ってもらっても構いませんが、その場合は私がサポートできないのでトラシューは自己責任でお願いします。
- 拡張機能のrust-analyzerをインストール
    - [https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# Hello, world

1. ターミナルで`cargo new hello-world` とするとカレントディレクトリにhello-worldディレクトリが作成されます。これがプロジェクトの雛形です。このプロジェクトをVSCodeで開いてください。
2. `src/main.rs` が起点となるファイルです。このファイルのmain()関数が実行の起点です。
3. VSCode内のターミナルから`cargo run` を実行してみてください。標準出力にビルド時のログと「Hello, world!」が表示されると思います。
4. `target/debug/hello-world` がビルドされて作成された実行ファイルです。ターミナルで`./target/debug/hello-world` を実行して「Hello, world!」が出力されるのを確認しましょう。

`cargo run --release`とするとリリースビルドになり、最適化されて実行速度が上がります。その代わりビルド時間は伸びます。`--release`オプションをつけないとデバッグビルドになる。

# cargoの使い方

- cargo check
    - 実行ファイルを生成せず、コンパイル時のチェックのみを行う。実行ファイルを生成しないためcargo buildより早い。
- cargo build
    - 実行ファイルを生成する
- cargo run
    - cargo build + 実行ファイルの実行
- cargo test
    - テストの実行（後述）

# 文法紹介・・・のその前に

Rustに限らないプログラミング言語一般のお話

1. “式”と”文”を区別してますか？ 
2. ２つのメモリ領域（スタックとヒープ）とガベージコレクション

# 式（expressions）と文（statements）

- 文とは、なんらかの動作をして値を返さない命令です。 式は結果値に評価されます。
    - ex. `if`は言語により `if”文”` だったり`if”式”`だったりする
        - Dart, C, Python は `if”文”`
        - Rustは`if“式”`
- 式の終端にセミコロンを付けたら、文になる。
- ブロックの最後ではセミコロンを省略でき、その式を戻り値として扱う

```rust
fn is_even(input: i32) -> bool {
    // if式なので、評価値を変数束縛できる
    // if式の条件式の丸括弧は不要
    let result = if input % 2 == 0 {
        return true;
    } else {
        // ブロックの最後の行のセミコロンを外すと
        // その式の評価値がそのブロックの戻り値になる
        false
				// false;　とするとコンパイルエラー
    };
    return result;
}
```

[関数 - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html#関数本体は文と式を含む)

# スタック（stack）とヒープ（heap）

## スタック領域（Stack）

- 高速で効率的なメモリ領域
- ローカル変数や関数呼び出しの情報を格納
- スタック上のデータはスコープを抜けると自動的に解放される
- 固定サイズで制約があり、小規模なデータに適している

## ヒープ領域（Heap）

- 動的なメモリ割り当てに使用
- メモリ確保・解放が手動で管理（ex. malloc/free）
- 大容量のデータや動的データ構造に適している
- 柔軟で容量の制約が少ない


# ガベージコレクション

Wikipediaより
[https://ja.wikipedia.org/wiki/ガベージコレクション](https://ja.wikipedia.org/wiki/%E3%82%AC%E3%83%99%E3%83%BC%E3%82%B8%E3%82%B3%E3%83%AC%E3%82%AF%E3%82%B7%E3%83%A7%E3%83%B3)

> コンピュータプログラムが動的に確保したメモリ領域のうち、不要になった領域を自動的に解放する機能である
> 

> ガベージコレクションを使用する場合、メモリを確保するコードはプログラマが明示的に記述するが、メモリの解放については明示的に記述する必要がなく、ガベージコレクタが不要と判断した時に、自動的にメモリを解放する。確保したメモリが不要かどうかは、プログラムが今後そのメモリにアクセスするかどうかで決まり、スタックや変数テーブルなどから参照をたどってメモリに到達可能かどうかによって判断される。
> 
- Stop The World
    - Full GC(ガベージコレクション)が実行されたときに、すべてのアプリケーションスレッドが停止する事象。ユーザから見るとアプリケーションが停止しているように見える。

--- 


# 変数
- 変数はデフォルトで不変（immutable）

```rust
let a = 4;

// 可変にするには mut キーワードを使う
let mut b = 10;

// これはシャドーイングなので mut は要らない
a = 7;
```

# 関数

Rustの関数と変数の命名規則はスネークケースです。関数定義は`fn`キーワードで始まり、関数名の後に丸括弧の組が続きます。
定義した関数は、名前に丸かっこの組を続けることで呼び出すことができます。 

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");  // 別の関数
}
```

次は引数ありのパターンです。
another_functionの宣言には、`x`という名前の仮引数があります。`x`の型はi32です。

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}
```

複数の引数を持たせたい場合は、カンマで区切ります。

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

最後に戻り値のあるパターンです。

戻り値は矢印(->)の後に型を描きます。

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

# 真偽値・数値

[サンプルコード](../examples/01_primitive.rs)

# タプル（tuple）

[サンプルコード](../examples/02_tuple.rs)

# 構造体(struct)

[サンプルコード](../examples/03_struct.rs)

# 列挙体（enum）

[サンプルコード](../examples/04_enum.rs)

# 文字・文字列

[サンプルコード](../examples/07_char_ref_str_string.rs)


# 配列・スライス・ベクタ

[サンプルコード](../examples/06_array_slice_vector.rs)

# 所有権

[所有権](./ownership.md)

# 参照と借用

[参照と借用](./reference_and_borrow.md)

# ライフタイム

[ライフタイム](./lifetime.md)

# トレイト
他の言語ではインタフェースなどと呼ばれる機能。

[サンプルコード１](../examples/trait_1.rs)
[サンプルコード２](../examples/trait_2.rs)


関数を定義するとき、トレイトは型ではないのでそのままでは使えない。
トレイトを型化する方法が２種類ある。

- 静的ディスパッチ（static dispatch）
- 動的ディスパッチ（dynamic dispatch）


静的ディスパッチは`impl`キーワードを使ってこのように書きます。

```rust
fn notify(item: &impl std::fmt::Display) {
    println!("Breaking news! {}", item);
}
```

この構文は↓のシンタックスシュガーになっている。
```rust
fn notify<T: std::fmt::Display>(item: &T) {
    println!("Breaking news! {}", item);
}
```

バリエーション

```rust
// Tのトレイト境界が複数のパターン
fn notify<T: std::fmt::Display + std::fmt::Debug>(a: T, b: T) {
    // skip
}

// ↑をwhereで書いたパターン
fn notify<T>(a: T, b: T)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    // skip
}

// 型引数が複数 & ライフタイムパラメータを含むパターン
fn notify3<'a, T, U>(a: T, b: &'a U)
where
    T: IntoIterator,
    U: std::fmt::Display,
{
}
```

静的ディスパッチと動的ディスパッチの利点・欠点は以下の通りです。

> ## 静的ディスパッチと動的ディスパッチの利点・欠点
> 静的ディスパッチについては下記のようにまとめられるでしょう。
> 
> - 利点
>   - 速度が速い。
> - 欠点
>   - いわゆる単相化により具象実装分のコードが個別に生成されることになるので、一般にはバイナリサイズの肥大化に繋がる。
>   - 型引数を構造体に毎回書く必要が出てくるため、構造体に注入するオブジェクトの数が増えれば増えるほど、型引数の管理をする必要が出てくる。
>
> 動的ディスパッチについては下記のようにまとめられるでしょう。
> 
> - 利点
>   - 実行時に紐付けが解決されるので、静的ディスパッチのケースとは異なりバイナリサイズが具象実装分だけ肥大化することはない。
>   - コンパイル時にサイズが決定できないオブジェクトも管理できる。
>   - 静的ディスパッチのケースと比較すると型引数が不要になるので、とくに実装変更時の手間が少ない。
> - 欠点
>   - 静的ディスパッチの場合行われるようなコンパイラが自動でかけるいくつかの最適化が無効化されるケースがある。
>   - vtable 内の探索など Rust の言語処理系の実装の都合で速度が静的ディスパッチに比べるとかなり遅くなる可能性がある。

[Rust の DI を考える –– Part 2: Rust における DI の手法の整理 - paild tech blog](https://techblog.paild.co.jp/entry/2023/06/12/170637) より引用
 

# クロージャ
[サンプルコード](../examples/closure.rs)

# マクロ
[マクロ](./macro.md)

## テスト
[サンプルプロジェクト](../../projects/test-sample)

## クレート
[クレート](./crate.md)

## エラーハンドリング

Rustでは`Option<T>`や`Result<T, E>`といった型が頻出する。Rustには`?`演算子というものが存在し、これを使ってOption::NoneやResult::Errを早期リターンするとコードの見通しが良くなります。


- [?なしパターン](../examples/error_handling_1.rs)
- [?ありパターン](../examples/error_handling_2.rs)

thiserrorクレートとanyhowクレートを組み合わせたエラーハンドリングが近年デファクトスタンダード化しつつあります。

thiserrorクレートを使うとDisplay, Error, Fromトレイトをシンプルに実装できます。

anyhowクレートを使うのは以下のような理由があります。

- 他のエラー型から`anyhow::Error`への変換が容易
- エラーの階層化
- `Backtrace``
- `anyhow!`, `ensure!`, `bail!`, `ensure!`等の便利マクロ

引用: [Rust/AnyhowのTips](https://zenn.dev/yukinarit/articles/b39cd42820f29e#%E3%81%AA%E3%81%9Canyhow%E3%82%92%E4%BD%BF%E3%81%86%E3%81%AE%E3%81%8B)

thiserrorとanyhowを使って書き換えたコードがこちらです。

[thiserror + anyhow](../../projects/modern-error-handling/)

```rust
#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("ここにエラーメッセージを書ける")]
    IOError(#[from] std::io::Error),
    #[error("'='が見つかりませんでした")]
    NotFound,
}

// 戻り値がanyhowのResultになった
fn get_first_cell(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path).map_err(|e| MyError::IOError(e))?;
    let cell = content.split_once(',').ok_or_else(|| MyError::NotFound)?;

    return Ok(String::from(cell.0));
}

fn main() {
    let result = get_first_cell("/path/hoge.csv");
    match result {
        Ok(cell) => {
            println!("first cell: {}", cell)
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
```

これを実行します。指定したパスにファイルが存在しないので`std::fs::read_to_string()`でエラーになります。

```
Error: ここにエラーメッセージを書ける

Caused by:
    No such file or directory (os error 2)
```

## ファイル分割・モジュール分割
[サンプルコード](../projects/module-sample/src/main.rs)

mod.rs, `extern crate`キーワードを使う方法は古い方式です。いまからRustを始める人は使わないようにしましょう。
- [パスとモジュールシステムへの変更 > さようなら、extern crate - エディションガイド](https://doc.rust-jp.rs/edition-guide/rust-2018/path-changes.html#%E3%81%95%E3%82%88%E3%81%86%E3%81%AA%E3%82%89extern-crate)
- [パスとモジュールシステムへの変更 > さようなら、mod.rs - エディションガイド](https://doc.rust-jp.rs/edition-guide/rust-2018/path-changes.html#%E3%81%95%E3%82%88%E3%81%86%E3%81%AA%E3%82%89modrs)

## スレッド
[スレッド](./threads.md)

## 非同期処理
[非同期処理](./async.md)

# Rust初心者に送る言葉
## まずはコンパイルが通るコードを書こう

- コード書く → Check On Save → エラーメッセージに従ってコード修正 → Check On Save → …
のサイクルを回す
- プログラムは動いてなんぼ。実行できなきゃ意味がない。最初から”完璧”を目指そうとしない。
- 困ったら躊躇わずにcloneを使おう
    - 慣れてきたら参照を使ってメモリ効率の良いコードを目指そう。
- 本番運用じゃなければ、Result, Optionから中身を取り出すときはunwrap() or expect()してしまおう
    - 慣れてきたらエラーハンドリングしよう
- （ある程度は）clippyがよりRustっぽい書き方を提案してくれる


# 今日説明しなかったこと

- バイナリクレートとライブラリクレート
- unsafe
- エディション
- async trait
- 他言語との連携（FFI）
- デバッガ（LLDB）
- マーカトレイト・Phantom Type
- Web Assembly
- const / static
- プロファイル（`cargo benchmark`）
- ドキュメント（`cargo doc`）
- クロスコンパイル
- CPU boundな場合のマルチスレッド
- OSスレッドとグリーンスレッド
- Nightly
- orphan rule
- lexical scope と non-lexical scope    
    -  [Non-Lexical Lifetimes - Qiita](https://qiita.com/_EnumHack/items/8b6ecdeb52e69a4ff384)

# 演習


## lsコマンドを作る

ターミナルで`ls`コマンドを実行したことはありますか？リポジトリルートで`ls`を実行すると、以下のような出力が得られます。

```
$ ls
Cargo.lock      Cargo.toml      src             target
```

これと同じ出力をする`myls`コマンドを作ってみましょう。`cargo new myls`でmylsプロジェクトを作りましょう。

```
$ ./target/debug/myls 
Cargo.lock   Cargo.toml   src   target   
```

`myls`が出力するファイル・ディレクトリの順番・表示は`ls`と一致させなくて大丈夫です。引数なしで実行するとカレントディレクトリのファイル・ディレクトリを標準出力に出力するようにしましょう。
<details>
<summary>
ヒント💡
</summary>
[std::fs](https://doc.rust-lang.org/std/fs/index.html)を使います。
</details>


それが出来たら、パスを引数で受け取り、そのパスのファイル・ディレクトリを標準出力に出力するようにしましょう。

<details>
<summary>
ヒント💡
</summary>
[コマンドライン引数を受け付ける - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch12-01-accepting-command-line-arguments.html#%E5%BC%95%E6%95%B0%E3%81%AE%E5%80%A4%E3%82%92%E5%A4%89%E6%95%B0%E3%81%AB%E4%BF%9D%E5%AD%98%E3%81%99%E3%82%8B)
</details>


## curlコマンドを作る

- `コマンド <URL>` ならURLにGETでリクエストし、ステータスコード200が返ってきたらレスポンスを標準出力に表示
- `コマンド -X POST <URL> -d <DATA>` でURLにPOSTでリクエストする。ステータスコード200が返ってきたらレスポンスを標準出力に表示
- [reqwest](https://docs.rs/reqwest/latest/reqwest/), [clap](https://docs.rs/clap/latest/clap/) クレートを使う
- 余裕があれば`-H “KEY:VALUE”` でヘッダを追加したり、クエリパラメータに対応したり、エラーハンドリングを追加してみる。
- オプションなしで実行したらusageを表示


`cargo run -p print-server`を実行するとデバッグ用のアプリがローカルで起動します。

```
$ curl localhost:3000
Hello from `GET /`
$ curl -X POST localhost:3000
Hello from `POST /`    
``````