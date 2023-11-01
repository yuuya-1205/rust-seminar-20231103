# 参照と借用

## TL;DR
- `&`で不変参照を、`& mut`で可変参照を作れる（ex. `&String`, `&mut String`）
- 複数読み出し or 単一書き込みの徹底
    - 不変参照は同時にいくつも作れる。
    - 有効な可変参照があるときには不変参照も可変参照も作ることができない。所有者も値の変更を出来ない。
- 参照はポインタLikeな概念だが、決してnullにはならない

## 解説はTRPLに

Rust公式ドキュメントの参照と借用の章が簡潔でわかりやすいのでそちらを使う

[参照と借用 - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch04-02-references-and-borrowing.html)
