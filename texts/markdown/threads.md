# スレッド

## spawnで新規スレッドを作成する
新規スレッドを生成するには、`std::thread::spawn`関数を呼び出し、 新規スレッドで走らせたいコードを含むクロージャを渡します。 

下記のコードはメインスレッドと新規スレッドからテキストを出力するサンプルコードです。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

同じコードが[examples/thread_1.rs](../examples/thread_1.rs)にありますので、
リポジトリルートで`cargo run --example thread_1`を実行するとサンプルコードを実行できます。

これを実行すると以下のような出力が得られます。結果は安定しないのでログが前後することがあります。

```rust
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

このサンプルコードでは、新しいスレッドは実行が終わったかどうかにかかわらず、メインスレッドが終了したら停止することに注意してください。

## joinハンドルで全スレッドの終了を待つ
spawn関数の戻り値（型はJoinHandle）に対してjoin()を呼び出すとスレッドが終了するまで待つことができます。

[examples/thread_2.rs](../examples/thread_2.rs)

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

実行すると以下のような出力が得られます。スレッドのログが9まで出力されています。

```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

## 

下記のコードはメインスレッドでベクタを生成し、 立ち上げたスレッドで使用しようとしています。しかし、このコードはコンパイルエラーになります。

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

```
live the current function, but it borrows `v`, which is owned by the current function
 --> texts/examples/thread_3.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> texts/examples/thread_3.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println!("Here's a vector: {:?}", v);
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++

For more information about this error, try `rustc --explain E0373`.
```

Rustは`v`のキャプチャ方法を推論し、`println!`は`v`への参照のみを必要とするので、クロージャは`v`を借用しようとします。ですが、問題があります。コンパイラには立ち上げたスレッドがどのくらいの期間走るのかわからないので、 `v`への参照が常に有効であるか把握できないのです。

この問題が浮き彫りになるコードがこちらです。
[examples/thread_3.rs](../examples/thread_3.rs)

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

もしこのコードを実行できてしまうなら、立ち上げたスレッドはまったく実行されることなく即座にバックグラウンドに置かれる可能性があります。 立ち上げたスレッドは内部に`v`への参照を保持していますが、メインスレッドはdrop関数を使用して、 即座にvをドロップしています。そして、立ち上げたスレッドが実行を開始する時には、`v`はもう有効ではなく、 参照も不正になるのです。

コンパイルエラーを修正するには、helpに書いてあるとおり、クロージャの前に`move`キーワードを追加することで、クロージャに使用している値の所有権を強制的に奪わせます。この変更を加えたコードはコンパイルでき、意図通りに動きます。


コンパイル出来て、動くコード。
[examples/thread_4.rs](../examples/thread_4.rs)

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

## スレッド間での共有データの参照

スレッド間でデータを共有したい場合は`std::sync::Arc`を使います。
Arc（Atomic Reference Counted）はスレッドセーフなリファレンスカウンタポインタで状態を共有できます。

読み込みしかしないのであれば、Arcで包まずに単純にcloneしても動くのだが、ヒープ領域のデータのコピーが発生するので速度は遅くなるし、メモリ使用量も増える。

[examples/thread_5.rs](../examples/thread_5.rs)


```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let v = Arc::new(vec![1, 1, 2, 3, 5]);

    let mut handles = vec![];
    for i in 0..v.len() {
        let v = v.clone();
        let handle = thread::spawn(move || {
            println!("Thread ID: {}, v[{}] = {}", i, i, v[i]);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

## スレッド間での共有データの書き込み


