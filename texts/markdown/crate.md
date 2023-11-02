いわゆるライブラリとかパッケージのことをRustではクレート（crate）と呼ぶ。
プロジェクトにクレートを追加するにはCargo.tomlの`[dependencies]`にクレート名とバージョンを書く。

```toml
[dependencies]
axum = "0.6.16"
```

featureフラグは、コンパイル時にコードの一部を有効または無効にするためのメカニズム。
コードの条件付きコンパイルを可能にし、プロジェクト内で異なる機能セットを持つバージョンをビルドするのに役立つ。
featureフラグは主に次の目的で使用さる：

1. クロスプラットフォーム対応: 異なるプラットフォーム（例: Windows、Linux、macOS）でコードをビルドする場合、プラットフォーム固有の機能やライブラリの使用を切り替える
1. コードのカスタマイズ: コードベース内で異なる機能セットを持つカスタムバージョンをビルドする際に、特定の機能をオンまたはオフにする
1. テストとデバッグ: デバッグやテストのために特定の機能を有効または無効にする

featureフラグを指定するときの記法は↓

```toml
sqlx = { version = "0.7.1", features = ["sqlite", "runtime-tokio-native-tls", "chrono"] }
```

そのクレートにどんなfeatureフラグがあるかは、ドキュメントかCargo.tomlを読めば書いてある。

## 有名なクレートの紹介
### serde
シリアライズ・デシリアライズ。読み方は統一されておらず、セルデとかサーデとかシリデとか

[serde - Rust](https://docs.rs/serde/latest/serde/)

[Overview · Serde](https://serde.rs/)

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
```

### chrono
日付・時刻・タイムスタンプ

[chrono - Rust](https://docs.rs/chrono/latest/chrono/)

```rust
use chrono::prelude::*;
use chrono::offset::LocalResult;


let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_opt(9, 10, 11)?.and_local_timezone(Utc).unwrap());

// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, NaiveDate::from_yo_opt(2014, 189)?.and_hms_opt(9, 10, 11)?.and_utc());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, NaiveDate::from_isoywd_opt(2014, 28, Weekday::Tue)?.and_hms_opt(9, 10, 11)?.and_utc());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_milli_opt(9, 10, 11, 12)?.and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_micro_opt(9, 10, 11, 12_000)?.and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_nano_opt(9, 10, 11, 12_000_000)?.and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33),
           LocalResult::Single(NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_opt(21, 15, 33)?.and_utc()));
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 8, 80, 15, 33), LocalResult::None);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 38, 21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);
```

### reqwest

Httpクライアント

[reqwest - Rust](https://docs.rs/reqwest/latest/reqwest/)

```rust
let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

println!("body = {:?}", body);
```

### tokio
デファクトスタンダードな非同期ランタイム

[tokio - Rust](https://docs.rs/tokio/latest/tokio/)

[Tutorial | Tokio - An asynchronous Rust runtime](https://tokio.rs/tokio/tutorial)
```rust
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
```

### axum
tokioと同じチームが作ったWebフレームワーク

[axum - Rust](https://docs.rs/axum/latest/axum/)

```rust
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
```

## sqlx
ORM（厳密にはORMではない）。SQLを直接書く。コンパイル時にコード内のSQLの構文チェック（テーブルの有無や型のチェックなど）をしてくれる。

[sqlx - Rust](https://docs.rs/sqlx/latest/sqlx/)


```rust
#[derive(sqlx::FromRow)]
struct User { name: String, id: i64 }

let mut stream = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? OR name = ?")
    .bind(user_email)
    .bind(user_name)
    .fetch(&mut conn);
```

```rust
let countries = sqlx::query!(
        "
SELECT country, COUNT(*) as count
FROM users
GROUP BY country
WHERE organization = ?
        ",
        organization
    )
    .fetch_all(&pool) // -> Vec<{ country: String, count: i64 }>
    .await?;

// countries[0].country
// countries[0].count
```

## diesel

ORM

[diesel - Rust](https://docs.rs/diesel/latest/diesel/)

diesel CLIツールがマイグレーションファイルを読み取って以下のようなマクロが自動生成される

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

すると、このような感じでDBにアクセスするコードが書ける。

```rust
use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    // 「SELECT * FROM posts where polished = true LIMIT 5;」相当
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
```

## クレートのドキュメントの読み方
ほぼ全てのクレートのドキュメントが以下の形式で提供されます。

[reqwest - Rust](https://docs.rs/reqwest/latest/reqwest/)

![image](../medias/docrs.png)

- ソースコードのリポジトリへのリンク
- バージョンの切り替え
- モジュール・構造体・トレイト・関数
- 検索
