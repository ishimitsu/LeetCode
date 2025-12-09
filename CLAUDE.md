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

## 新規LeetCode問題のセットアップ手順

新しいLeetCode問題に取り組む際は、以下の手順でセットアップを行う：

### 1. ブランチ作成
```bash
git checkout -b feature/{問題番号}-{問題名(小文字、ハイフン区切り)} main
git push -u origin feature/{問題番号}-{問題名}
```
例: `feature/15-3sum`, `feature/14-longest-common-prefix`

### 2. ディレクトリ・ファイル作成
ディレクトリ名: `{問題番号}_{問題名(アンダースコア区切り)}`
例: `15_3Sum`, `14_Longest_Common_Prefix`

作成するファイル:
- `CMakeLists.txt`
- `{問題名}.cpp`

#### CMakeLists.txt テンプレート
```cmake
cmake_minimum_required(VERSION 3.14)
project({問題名})

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_{問題名} {問題名}.cpp)
target_link_libraries(test_{問題名} GTest::gtest GTest::gtest_main)
```

#### {問題名}.cpp テンプレート
```cpp
#include <iostream>
#include <vector>
// 必要に応じて他のヘッダを追加

using std::vector;
// 必要に応じて他のusingを追加

class Solution {
public:
    // LeetCodeの関数シグネチャをここに記述
    // TODO: implement
};

#include <gtest/gtest.h>

class {問題名}Test : public ::testing::Test {
protected:
    Solution solution;
};

// LeetCodeのExample1, 2, 3...をテストケースとして追加
TEST_F({問題名}Test, Example1) {
    // TODO: implement test
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
```

### 3. コミット・プッシュ
```bash
git add {ディレクトリ名}/
git commit -m "add initial files for {問題番号}. {問題名}"
git push
```

### 4. PR作成
```bash
gh pr create --title "Feature/{問題番号} {問題名(小文字)}" --body ""
```
例: `Feature/15 3sum`, `Feature/14 longest common prefix`