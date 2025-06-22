# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Rustで実装されたシンプルな電卓アプリケーションです。テスト駆動開発（TDD）を採用し、テストは日本語で記述されています。

## 開発コマンド

```bash
# ビルド
cargo build

# テスト実行
cargo test

# 特定のテストのみ実行（例：計算機関連のテスト）
cargo test 計算機

# コード品質チェック
cargo clippy

# コードフォーマット
cargo fmt

# 実行
cargo run
```

## アーキテクチャ構造

### メインプロジェクト構成
- `src/lib.rs`: `Calculator`構造体と計算ロジックの実装
- `src/main.rs`: アプリケーションのエントリーポイント
- 現在は`add`メソッドのみ実装済み

### 開発方針
1. **テスト駆動開発**: 新機能追加時は必ずテストを先に作成
2. **日本語テスト**: テスト名とコメントは日本語で記述
3. **段階的実装**: 各演算を個別に実装し、段階的に機能を追加

### 注意事項
- `calculator/`サブプロジェクトは現在未使用（将来的な統合または削除を検討）
- VS Code設定（`.vscode/`）は詳細な開発環境設定を含む
- Rust Analyzerとclippyによる静的解析を活用