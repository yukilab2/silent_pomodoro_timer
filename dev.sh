#!/bin/bash

# macOSでのXQuartzセットアップ確認
if [[ "$OSTYPE" == "darwin"* ]]; then
    if ! command -v xhost &> /dev/null; then
        echo "Error: xhost コマンドが見つかりません。XQuartzをインストールしてください:"
        echo "  brew install --cask xquartz"
        exit 1
    fi
    
    # XQuartzへの接続を許可（既に許可されている場合は無視される）
    xhost +localhost 2>/dev/null || true
    
    export DISPLAY=:0
fi

# Docker Composeでコンテナを起動
docker-compose up -d

echo ""
echo "開発環境が起動しました。以下のコマンドでコンテナに入ってください:"
echo "  docker-compose exec pomodoro-dev bash"
echo ""
echo "コンテナ内でアプリを実行:"
echo "  cargo run"
echo ""

