# Rust勉強会ネタだし

# TOC

自己紹介が要るのでは？

- 所属
- こんぶくんとの関係
- Rustとの出会い
    - 安全なアプリケーションであると確信できるようになるには
    - 状態爆発との闘い
    - そのアンサーとしての形式手法。AWSも採用

# なぜRustか from [https://www.rust-lang.org/ja](https://www.rust-lang.org/ja)

## パフォーマンス

Rustは非常に高速でメモリ効率が高くランタイムやガベージコレクタがないため、パフォーマンス重視のサービスを実装できますし、組込み機器上で実行したり他の言語との調和も簡単にできます。

## 信頼性

Rustの豊かな型システムと所有権モデルによりメモリ安全性とスレッド安全性が保証されます。さらに様々な種類のバグをコンパイル時に排除することが可能です。

## 生産性

Rustには優れたドキュメント、有用なエラーメッセージを備えた使いやすいコンパイラ、および統合されたパッケージマネージャとビルドツール、多数のエディタに対応するスマートな自動補完と型検査機能、自動フォーマッタといった一流のツール群が数多く揃っています。

# 深堀り

- 実行速度
    - ガベージコレクションがない
        - かといってCやC++のようにプログラマが自分でメモリ管理するわけではない
        - 所有権という制約を課すことでコンパイル時にリソース解放のタイミングを正確に把握
    - ゼロコスト抽象化
        - ある条件を満たした列挙型をnullableポインタ最適化
        - スタック領域でメモリ確保するように最適化
        - 静的ディスパッチ
        - 参考：[https://blog.rust-jp.rs/tatsuya6502/posts/2019-12-zero-cost-abstraction/](https://blog.rust-jp.rs/tatsuya6502/posts/2019-12-zero-cost-abstraction/)
- 信頼性
    - コンパイル時に危険な動作がないかチェックしてくれる
        - nullポインタのデリファレンス
        - 未初期化メモリの参照
        - マルチスレッド特有の見つけづらいバグが起こらないことを担保
            - 所有権よって同一のメモリを複数箇所から同時に書き換えることができないないのでデータ競合が発生しない
            - dartのサンプルコード書く？
- 生産性
    - ツールが一通り揃っていて、戦争がない。
        - パッケージマネージャ・ビルドツールetc. : cargo
        - フォーマッタ: rustfmt
        - リンタ: clippy
        - LSP: rust-analyzer

# 良いところばかり紹介するのはフェアじゃないので辛みも紹介する

## コンパイルに時間がかかる

- 前述の信頼性・パフォーマンスを実現するためにコンパイル時の処理が重い。プロジェクトが大きくなるとコンパイルに時間かかる。
    - クレート分割である程度対応はできる。
    - CIはキャッシュの有効活用がキモ

## 学習コストが高い

- 2024年に向けたロードマップに掲げたテーマの一つに「学習しやすくする」を挙げている
- [https://lang-team.rust-lang.org/roadmaps/roadmap-2024.html](https://lang-team.rust-lang.org/roadmaps/roadmap-2024.html)

## 長く開発してるとローカルストレージを圧迫することがある

- `$HOME/.cargo` にクローンしたリポジトリがキャッシュされて貯まっていく
- プロジェクトの`target/debug` 配下にデバッグシンボルファイルやライブラリファイルが溜まっていく
    - `rustup self uninstall` でRustをアンインストールできます。この勉強会以降は絶対に絶対にRustは使わない人は消しておくといいかも。

# Rustを採用した企業・プロジェクト

- Discord
- AWS Lambda

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

1. ターミナルで`cargo init hello-world` とするとカレントディレクトリにhello-worldディレクトリが作成されます。これがプロジェクトの雛形です。このプロジェクトをVSCodeで開いてください。
2. `src/main.rs` が起点となるファイルです。このファイルのmain()関数が実行の起点です。
3. VSCode内のターミナルから`cargo run` を実行してみてください。標準出力にビルド時のログと「Hello, world!」が表示されると思います。
4. `target/debug/hello-world` がいまビルドされて作成された実行ファイルです。ターミナルで`./target/debug/hello-world` を実行して「Hello, world!」が出力されるのを確認しましょう。

※実行ファイルの説明もいる？

リリースビルドの話する

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

イラスト付きで説明する

- スタックに積まれる。サイズが確定している必要あり。
- ヒープ領域で確保される。アクセス遅い。

# ガベージコレクション

Wikipediaより
[https://ja.wikipedia.org/wiki/ガベージコレクション](https://ja.wikipedia.org/wiki/%E3%82%AC%E3%83%99%E3%83%BC%E3%82%B8%E3%82%B3%E3%83%AC%E3%82%AF%E3%82%B7%E3%83%A7%E3%83%B3)

> コンピュータプログラムが動的に確保したメモリ領域のうち、不要になった領域を自動的に解放する機能である
> 

> ガベージコレクションを使用する場合、メモリを確保するコードはプログラマが明示的に記述するが、メモリの解放については明示的に記述する必要がなく、ガベージコレクタが不要と判断した時に、自動的にメモリを解放する。確保したメモリが不要かどうかは、プログラムが今後そのメモリにアクセスするかどうかで決まり、スタックや変数テーブルなどから参照をたどってメモリに到達可能かどうかによって判断される。
> 
- Stop The World
    - Full GC(ガベージコレクション)が実行されたときに、すべてのアプリケーションスレッドが停止する事象。ユーザから見るとアプリケーションが停止しているように見える。

# 基本的な型

- i8, i16, i32, i64, u8…
- usize, isize
- boolean

# 文字・文字列

- &str
- String
- char

# 配列・スライス・ベクタ

サンプルコードも用意して説明する

# 構造体（struct）

サンプルコードも用意して説明する

# 列挙体（enum）

サンプルコードも用意して説明する

# 参照

[参照](https://www.notion.so/d2d638dcbc81423e8fb0d82949b0b873?pvs=21)

# ライフタイム

[ライフタイム](https://www.notion.so/1bcafcd4ee0941beae08bb107776597c?pvs=21)

# トレイト

まだ完成してないサンプルコード

- [ ]  functional_programing
- [ ]  ownership
- [ ]  lifetime
    - 所有権・ライフタイム・借用
        - 『Rust The Book』
        - 『プログラミングRust』pp 98,99
        - コンパイルエラーメッセージの読み方
        
        ```rust
        let s = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let t = s;
        let u = s;
        ```
        
        - ‘a （tick aと発音する）
        - 再帰リストとBox<T>
        - 『プログラミングRust』pp 102,103
        - 
- [ ]  trait
    - トレイト
        - `型が実装すべきメソッドを列挙して宣言したもの`
        - オブジェクト指向言語で用いられるクラスによる抽象化との比較
            - トレイトオブジェクト
        - コンパイルエラーに従って実装していく
        - 『コンセプトから理解するRust』
- [ ]  closure
    - 説明軽くで十分。型はなくて、３つのトレイトってことだけ伝える。あとmove

# マクロ

println!はマクロ。Do you know メタプログラミング？

RustのマクロはCの`#define`ような単なる文字列の置換では無い。安全に使える。

- 宣言的（declarative）マクロ
- ３種類の手続き的 (procedural) マクロ
    - deriveマクロ
    - 属性マクロ
    - 関数マクロ

## よく使われるマクロ

- println!
- format!
- assert_eq!
- vec!
- derive
- panic!
- todo!
- unimplemented!
- tokio::main
    - 自作もできるが凝ったものを作ると黒魔術
    - RustのAST（抽象構文木: Abstract Syntax Tree）の知識が必要になる場合がある

# 今日説明しなかったこと

- feature フラグ（クレート紹介のところで説明しそう）
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
    
    non-lexical scopeはRust 1.31.0から導入されたライフタイム推論アルゴリズム（※"推論アルゴリズム”はニュアンスが違う気がする
    
    [Non-Lexical Lifetimes - Qiita](https://qiita.com/_EnumHack/items/8b6ecdeb52e69a4ff384)
    

# Rust初心者に送る言葉

## まずはコンパイルが通るコードを書こう

- コード書く → Check On Save → エラーメッセージに従ってコード修正 → Check On Save → …
のサイクルを回す
- プログラムは動いてなんぼ。実行できなきゃ意味がない。最初から”完璧”を目指そうとしない。O
- 困ったら躊躇わずにcloneを使おう
    - 慣れてきたら参照を使ってメモリ効率の良いコードを目指そう。
- 本番運用じゃなければ、Result, Optionから中身を取り出すときはunwrap() or expect()してしまおう
    - 慣れてきたらエラーハンドリングしよう
- （ある程度は）clippyがよりRustっぽい書き方を提案してくれる

# いまどきのエラーハンドリング

- 近年のデファクトスタンダード化しつつあるanyhowとthiserrorクレートを組み合わせたエラーハンドリングを教える。それを使えば簡潔に書ける。

- ファイル分割・モジュール分割
- テスト
- スレッド
    - データ競合を防ぐ仕組み
    - Arc<Mutex<T>>
- プロジェクトにクレートを追加する方法
- 非同期
    - Future, async, 非同期ランタイム
- クレートのドキュメントの読み方
- 有名なクレートの紹介
    - serde（シリアライズ・デシリアライズ。読み方は統一されておらず、セルデとかサーデとかシリデとか）
    - tokio（デファクトスタンダードな非同期ランタイム）
    - chrono（日付・時刻・タイムスタンプ）
    - axum（tokioと同じチームが作ったWebフレームワーク）
- 簡単なAPI Server（GETのみ）
- DBとつなぐ（docker-compose.ymlをこちらで用意する）
- sqlx追加
- tracing追加

簡易curlコマンドを作る

- オプションなしで実行したらusageを表示
- `コマンド <URL>` ならURLにGETでリクエストし、ステータスコード200が返ってきたらレスポンスを標準出力に表示
- `コマンド -X POST <URL> -d <DATA>` でURLにPOSTでリクエストする。ステータスコード200が返ってきたらレスポンスを標準出力に表示
- 自分でAPIのdocumentを読んだりサンプルコードを見ながら実装する
- reqwest, clap クレートを使う
- 余裕があれば`-H “KEY:VALUE”` でヘッダを追加したり、クエリパラメータに対応したり、エラーハンドリングを追加してみる。
- `-v`や`-vvv`相当のことがしたいなら、reqwestより低レイヤーのhyperを使わないできないと思われる。
- axumでアプリ作る
    - ヘッダ・ペイロード・クエリパラメータ・メソッド・パスを標準出力
    - GETはhello worldを返すだけ
    - POSTはリクエストのフォーマットを判定してからオウム返し
        - application/json ?

# 勉強会のフォーマット

資料はGitHub上で公開

スライドはあまり枚数多くないと思う

マークダウンで教科書を書く

Zero To Productionを参考に

Windowsどうする