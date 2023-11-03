# 非同期処理

Rustには非同期ランタイムが含まれていない。
公式からはFutureトレイトが提供されるので、各々が好きな非同期ランタイムを選ぶ。自作の非同期ランタイムも使える。

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

Futureは、終了したかどうかを確認できる操作を表している。poll関数は操作の終了を待たず常に即座にリターンする。もし操作が終了していたら`Poll::Ready(output)`を返す。終了していなければ`Poll::Pending`を返します。

`tokio`がデファクトスタンダードな非同期ランタイム。とりあえずこれを使っとけばOK。

プロジェクトを非同期処理に対応するには、

- Cargo.tomlのdependencyにtokioを追加 
- main関数に`#[tokio::main]`マクロを付与
- `fn`を`async fn`に変更

すれば`await`式が使えるようになる。


[simple-tcp-echo-server](../../projects/simple-tcp-echo-server)

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                let out = String::from_utf8(buf[0..n].to_vec()).unwrap();
                let out = out.trim();

                if let Err(e) = socket
                    .write_all(&format!("{}{}\n", out, out).as_bytes())
                    .await
                {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

私は`tokio`しか使ったことないけど、他には`async-std`や`io_uring`という非同期ランタイムがあるっぽい。

参考: [Rustの非同期ランタイムが多すぎる？io_uringなやつを使おう！ - nttlabs - Medium](https://medium.com/nttlabs/rust-async-runtime-comparison-7a79a4477fed)