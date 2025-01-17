# Rustの公式イメージを使用
FROM rust:latest

# 作業ディレクトリを設定
WORKDIR /app

# デフォルトのコマンド（コンテナ内で操作できるようにする）
CMD ["bash"]
