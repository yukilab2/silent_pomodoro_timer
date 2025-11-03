# Docker環境での開発（Linux用）

注意: Docker内（Linux環境）でGUIアプリを実行するには、X11またはWaylandが必要です。

macOSでXQuartzを使わずに開発したい場合は、Dockerを使わずにmacOS上で直接開発することを推奨します（メインのREADME.mdを参照）。

## 前提条件

- Docker と Docker Compose
- X11サーバー（macOSではXQuartz、Linuxでは通常インストール済み）

## macOSでのXQuartzセットアップ

1. XQuartzをインストール:
   ```bash
   brew install --cask xquartz
   ```

2. XQuartzを起動（アプリケーション > ユーティリティ > XQuartz）

3. XQuartzの設定で「Allow connections from network clients」を有効化

4. XQuartzを再起動

5. ターミナルでXQuartzへの接続を許可:
   ```bash
   xhost +localhost
   ```

## Dockerコンテナの起動

```bash
# 開発環境を起動
docker-compose up -d

# コンテナ内に入る
docker-compose exec pomodoro-dev bash

# コンテナ内でアプリを実行
DISPLAY=:0 cargo run
```

