FROM rust:1.75-slim as builder

# ビルドに必要な依存関係をインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 依存関係を先にコピーしてキャッシュを効かせる
COPY Cargo.toml ./
COPY Cargo.lock* ./

# ソースコードをコピー
COPY src ./src

# リリースビルド
RUN cargo build --release

# 実行用のイメージ
FROM debian:bookworm-slim

# 実行時に必要なライブラリをインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libxcb1 \
    libxcb-render0 \
    libxcb-shape0 \
    libxcb-xfixes0 \
    libxkbcommon0 \
    libxkbcommon-x11-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/pomodoro_timer .

# 設定ディレクトリ用のマウントポイント
RUN mkdir -p /root/.config/pomodoro

CMD ["./pomodoro_timer"]

