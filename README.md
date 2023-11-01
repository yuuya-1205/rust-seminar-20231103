# Rust勉強会資料

## テキスト
使用するテキストは`/texts/markdown`配下に入っています。主に[texts/markdown/index.md](texts/markdown/index.md)を使います。


## サンプルコード
`/projects`ディレクトリ配下には複数の解説用のプロジェクトが入っています。
実行するには、リポジトリルートで

```
cargo run -p <project名>
```

と実行してください。例えば`static-text-hard-coded`なら

```
cargo run -p static-text-hard-coded
```

です。




`/texts/examples/`ディレクトリ配下には小規模なサンプルコードが入っています。サンプルコードを実行するには、リポジトリルートで

```
cargo run --example <ファイル名（拡張子なし）>
```

と実行してください。例えば`thread_1.rs`なら

```
cargo run --example thread_1
```

です。



