---
name: cpp-project-setup
description: C++でのLeetCode問題セットアップ。C++プロジェクトを新規作成する時、C++でLeetCodeに取り組む時に使用。CMakeLists.txt、C++ソースファイル、GoogleTestの設定を行う。
---

# C++ LeetCode プロジェクトセットアップ

## 開発コマンド

### ビルド・実行
- ビルド: `cmake -B build && cmake --build build`
- テスト実行: `./build/test_{問題名}`
- 直接コンパイル: `g++ -std=c++17 -o solution {問題名}.cpp -lgtest -lgtest_main -pthread`

### デバッグ
- 警告付きコンパイル: `g++ -std=c++17 -Wall -Wextra -o solution {問題名}.cpp -lgtest -lgtest_main -pthread`
- valgrind: `valgrind ./solution`
- gdb: `gdb ./solution`

## よく使うライブラリ
- `<vector>` - 動的配列
- `<string>` - 文字列操作
- `<unordered_map>` - ハッシュテーブル
- `<unordered_set>` - ハッシュセット
- `<queue>` - BFS、優先度キュー
- `<stack>` - DFS、パース処理
- `<algorithm>` - ソート、検索ユーティリティ

## ディレクトリ・ファイル作成

ディレクトリ名: `{問題番号}_{問題名(アンダースコア区切り)}`
例: `15_3Sum`, `14_Longest_Common_Prefix`

作成するファイル:
- `CMakeLists.txt`
- `{問題名}.cpp`

### CMakeLists.txt テンプレート

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

### {問題名}.cpp テンプレート

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
