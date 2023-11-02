# なぜRustか 
[日本語版Rust公式サイト](https://www.rust-lang.org/ja) には以下の３つが挙げられている

## パフォーマンス

> Rustは非常に高速でメモリ効率が高くランタイムやガベージコレクタがないため、パフォーマンス重視のサービスを実装できますし、組込み機器上で実行したり他の言語との調和も簡単にできます。

## 信頼性

> Rustの豊かな型システムと所有権モデルによりメモリ安全性とスレッド安全性が保証されます。さらに様々な種類のバグをコンパイル時に排除することが可能です。

## 生産性

> Rustには優れたドキュメント、有用なエラーメッセージを備えた使いやすいコンパイラ、および統合されたパッケージマネージャとビルドツール、多数のエディタに対応するスマートな自動補完と型検査機能、自動フォーマッタといった一流のツール群が数多く揃っています。

# もっとくだけた表現で説明してみる

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
- Linux
    - [Linuxカーネルのバージョン6.1が公開、カーネル記述にRust言語を一部採用した最初のバージョン|CodeZine（コードジン）](https://codezine.jp/article/detail/17038)
- Dropbox
    - [DropboxがコアサービスをRustで書き換えた背景とは | Think IT（シンクイット）](https://thinkit.co.jp/article/17513)
- Android
    - [「ベアメタル」環境でもRustを採用 Googleが「Android 14」での取り組みを解説 - 窓の杜](https://forest.watch.impress.co.jp/docs/news/1538800.html)
- Cloudflare
    - [Cloudflare、NGINXに代えて自社開発のRust製HTTPプロキシ「Pingora」をグローバルCDNに採用。性能向上しつつCPUとメモリ消費を3分の1に － Publickey](https://www.publickey1.jp/blog/22/cloudflarenginxrusthttppingoracdncpu31.html)
- Discord
    - [なぜDiscordはGoからRustへ移行するのか - MISONLN41's Blog](https://misonln41.hateblo.jp/entry/2020/02/12/232853)
- AWS Lambda
    - [Firecracker – サーバーレスコンピューティングのための軽量な仮想化機能 | Amazon Web Services ブログ](https://aws.amazon.com/jp/blogs/news/firecracker-lightweight-virtualization-for-serverless-computing/)

- 日本でRustを利用している会社はこのリポジトリにまとめられている
  - [fnwiya/japanese-rust-companies: 日本で Rust を利用している会社一覧](https://github.com/fnwiya/japanese-rust-companies) 
    - クックパット
    - サイバーエージェント
    - ピクシブ
    - etc

