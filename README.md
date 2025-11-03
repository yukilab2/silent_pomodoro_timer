# ポモドーロタイマー

視覚的なフィードバックで通知するポモドーロタイマーアプリケーション（音声なし）。

## 機能

- 25分の作業タイマーと5分の休憩タイマー
- タイマー終了時にウィンドウをフラッシュして通知
- 設定可能なフラッシュ色（白/黒/赤/青/緑）
- タイマーの一時停止・再開機能
- 設定の永続化（`~/.config/pomodoro/settings.ini`）

## 開発環境のセットアップ

### 前提条件

- Rust (最新のstable版推奨)
- macOSでは追加のセットアップ不要（X11不要）

### ローカル開発（推奨 - macOSではX11不要）

macOS上で直接開発する場合、XQuartzは不要です。egui/eframeはmacOSのネイティブAPIを使用します。

```bash
# アプリを実行
cargo run

# リリースビルド
cargo build --release
```

### Docker内での開発（Linux環境の場合）

Docker内で開発する場合、Linux環境のためX11またはWaylandが必要です。

```bash
# 開発環境を起動
docker-compose up -d

# コンテナ内に入る
docker-compose exec pomodoro-dev bash

# コンテナ内でアプリを実行（X11転送が必要）
DISPLAY=:0 cargo run
```

### ビルドと実行

```bash
# コンテナ内で実行
cargo build
cargo run

# リリースビルド
cargo build --release
```

## クロスコンパイル（Windows/Linux用ビルド）

eframe/eguiはクロスプラットフォーム対応のため、WindowsとLinux用にビルドできます。

### Windows用ビルド（macOS/Linuxから）

```bash
# Windows用ツールチェーンの追加
rustup target add x86_64-pc-windows-gnu

# Windows用ビルド（GNUツールチェーン使用）
cargo build --release --target x86_64-pc-windows-gnu

# またはMSVCツールチェーン使用（Windows上でのみ可能）
cargo build --release --target x86_64-pc-windows-msvc
```

**注意**: Windows用ビルドには、追加のツールチェーンが必要な場合があります。

- GNUツールチェーン: `mingw-w64`が必要
  - macOS: `brew install mingw-w64`
  - Linux: `sudo apt-get install gcc-mingw-w64-x86-64`
- MSVCツールチェーン: Windows上でのみ使用可能

### Linux用ビルド（macOS/Windowsから）

```bash
# Linux用ツールチェーンの追加
rustup target add x86_64-unknown-linux-gnu

# Linux用ビルド
cargo build --release --target x86_64-unknown-linux-gnu
```

**注意**: Linux用クロスコンパイルには、追加のツールチェーンが必要です。

- macOS: `brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu`
- またはDockerコンテナ内で直接ビルドする方法もあります

### 各プラットフォームでのネイティブビルド

#### macOS

```bash
# リリースビルド（バイナリのみ）
cargo build --release

# macOS .appバンドルの作成（ターミナルを開かないGUIアプリ）
./build-macos-app.sh

# 作成された.appバンドルをApplicationsフォルダにコピー
cp -r target/PomodoroTimer.app /Applications/
```

#### Linux

```bash
cargo build --release
```

#### Windows

```bash
# MSVCツールチェーン
cargo build --release

# またはGNUツールチェーン
cargo build --release --target x86_64-pc-windows-gnu
```

## 使用方法

1. アプリを起動すると、25分のタイマーが開始待ち状態になります
2. 中央のボタンをクリックするとタイマーが開始します
3. タイマー実行中にボタンをクリックすると一時停止します
4. タイマー終了時、ウィンドウがフラッシュします
5. フラッシュ中にボタンをクリックすると次のタイマーが開始します
6. 右上の⚙ボタンで設定を変更できます

## プロジェクト構造

```text
.
├── src/
│   ├── main.rs      # エントリーポイント
│   ├── app.rs       # メインアプリケーションロジック
│   └── settings.rs  # 設定管理
├── Dockerfile       # 本番用ビルド
├── Dockerfile.dev   # 開発用環境
├── docker-compose.yml
└── Cargo.toml
```

## 技術スタック

- Rust
- egui (GUIフレームワーク)
- eframe (アプリケーションフレームワーク)

## ライセンス

（必要に応じて追加）
