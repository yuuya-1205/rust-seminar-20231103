# Rustベースの公式イメージを使用
FROM rust:latest

# 作業ディレクトリを設定
WORKDIR /app

# ソースコードをコンテナにコピー
COPY ./src /app/src
COPY Cargo.toml /app

# アプリのビルド
RUN cargo build --features "tracer"

# アプリを起動
CMD ["./target/debug/print-server"]
