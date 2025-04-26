FROM rust:latest

WORKDIR /app

# Python3最小セットだけインストールして、pipだけ使えるようにする
RUN apt-get update && apt-get install -y --no-install-recommends python3 python3-pip && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# maturinインストール
RUN pip3 install --break-system-packages --no-cache-dir maturin

CMD ["bash"]
