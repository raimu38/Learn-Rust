# Rustの公式イメージを使用
FROM rust:latest

# 作業ディレクトリを設定
WORKDIR /app

# Cargo.toml と Cargo.lock をコピーして依存関係を解決
COPY Cargo.toml  Cargo.lock ./
RUN cargo fetch

# ソースコードをコピー
COPY src ./src

# プロジェクトをビルド
RUN cargo build --release

# デフォルトのコマンド
CMD ["cargo", "run"]
