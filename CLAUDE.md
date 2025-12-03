# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリでコードを操作する際のガイダンスを提供します。

## リポジトリ概要

このリポジトリはC++でのLeetCode練習用です。主な用途は：
- 問題で行き詰まった時のアドバイス
- 実装したソリューションのコードレビュー

## 開発コマンド

### C++開発
- コンパイル: `g++ -std=c++17 -o solution solution.cpp`
- デバッグ用コンパイル: `g++ -std=c++17 -g -o solution solution.cpp`
- 実行: `./solution`

### テスト・デバッグ
- 警告を含むコンパイル: `g++ -std=c++17 -Wall -Wextra -o solution solution.cpp`
- valgrindで実行（利用可能な場合）: `valgrind ./solution`
- gdbでデバッグ: `gdb ./solution`

## LeetCode用の一般的なC++ライブラリ
- `<vector>` - 動的配列
- `<string>` - 文字列操作
- `<unordered_map>` - ハッシュテーブル
- `<unordered_set>` - ハッシュセット
- `<queue>` - BFS、優先度キュー
- `<stack>` - DFS、パース処理
- `<algorithm>` - ソート、検索ユーティリティ

## コードレビューの重点項目
- 時間・空間計算量の分析
- エッジケースの処理
- コードの明瞭性と保守性
- C++のベストプラクティスとイディオム
- メモリ管理の考慮事項