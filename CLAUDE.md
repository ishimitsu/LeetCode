# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリでコードを操作する際のガイダンスを提供します。

## リポジトリ概要

このリポジトリはC++とRustでのLeetCode練習用です。主な用途は：
- 問題で行き詰まった時のアドバイス
- 実装したソリューションのコードレビュー

## コードレビューの重点項目
- 時間・空間計算量の分析
- エッジケースの処理
- コードの明瞭性と保守性
- 各言語のベストプラクティスとイディオム
- メモリ管理の考慮事項

## 新規LeetCode問題のセットアップ手順

新しいLeetCode問題に取り組む際は、以下の共通手順に加え、言語別のSkillを参照する。

### 1. ブランチ作成
```bash
git checkout main
git pull origin main
git checkout -b feature/{問題番号}-{問題名(小文字、ハイフン区切り)}
git push -u origin feature/{問題番号}-{問題名}
```
例: `feature/15-3sum`, `feature/14-longest-common-prefix`

### 2. ディレクトリ・ファイル作成

**言語別のSkillを参照：**
- C++: `cpp-project-setup` Skill
- Rust: `rust-project-setup` Skill

### 3. コミット・プッシュ
```bash
git add {ディレクトリ名}/
git commit -m "add initial files for {問題番号}. {問題名}"
git push
```

### 4. PR作成
```bash
gh pr create --title "Feature/{問題番号} {問題名(小文字)}" --body "https://leetcode.com/problems/{問題名(小文字、ハイフン区切り)}"
```
例:
- タイトル: `Feature/15 3sum`, `Feature/14 longest common prefix`
- URL: `https://leetcode.com/problems/3sum`, `https://leetcode.com/problems/longest-common-prefix`

## ローカルブランチのクリーンアップ

PRがマージされリモートブランチが削除された後、対応するローカルブランチを削除する。

```bash
git fetch --prune && \
git branch -vv | grep ': gone]' | awk '{print $1}' | while read branch; do
  state=$(gh pr list --head "$branch" --state merged --json number --jq 'length')
  if [ "$state" -gt 0 ]; then
    git branch -D "$branch"
    echo "Deleted (merged PR): $branch"
  else
    echo "Skipped (not merged?): $branch"
  fi
done
```

- `git fetch --prune`: リモートで削除されたブランチのトラッキング情報を更新
- `gone` と表示されるブランチ（リモート削除済み）を対象に、PRのマージ状態を `gh` で確認
- マージ済みであれば強制削除（`-D`）、未マージは保留してスキップ
- squash merge / rebase merge もPR状態で正しく判定できる
