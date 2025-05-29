# saba

Wasabi OS上で動作するRustアプリケーションのサンプルプロジェクトです。

## 概要
『［作って学ぶ］ブラウザのしくみ──HTTP、HTML、CSS、JavaScriptの裏側』で解説されているソースコード
https://github.com/d0iasm/sababook

sabaは、[Wasabi OS](https://github.com/hikalium/wasabi)上で実行されるno_stdなRustアプリケーションです。noliライブラリを使用してシステムコールを実行し、基本的な入出力操作を行います。

## 特徴

- **no_std環境**: 標準ライブラリを使用せずに動作
- **Wasabi OS対応**: 専用のOSカーネル上で実行
- **noli統合**: Wasabi OS用のシステムコールライブラリを使用
- **クロスコンパイル**: x86_64-unknown-noneターゲット向けにビルド

## 必要な環境

- Rust nightly (2024-01-01)
- Git
- Make
- jq
- wget
- readelf, hexdump (デバッグ用)

## セットアップ

### 1. リポジトリのクローン

```bash
git clone <このリポジトリのURL>
cd saba
```

### 2. Rustツールチェーンの設定

プロジェクトには`rust-toolchain.toml`が含まれているため、適切なRustバージョンが自動的に使用されます。

### 3. ターゲットの追加

```bash
rustup target add x86_64-unknown-none
```

## ビルドと実行

### ローカルビルド

```bash
make build
```

### Wasabi OS上での実行

Wasabi OSをセットアップして実行する場合：

```bash
MORE_QEMU_FLAGS="-display cocoa" ./run_on_wasabi.sh
```

このスクリプトは以下の処理を自動的に行います：
- `build/`ディレクトリの作成
- Wasabi OSの`for_saba`ブランチのクローン/更新
- アプリケーションのビルド
- Wasabi OS上でのアプリケーション実行

### 手動実行

```bash
make run
```

## 開発

### テスト

```bash
make test
```

### コード品質チェック

```bash
make clippy
```

### デバッグ用コマンド

```bash
# オブジェクトファイルの逆アセンブル
make objdump

# シンボルテーブルの表示
make nm

# ELFヘッダーの表示
make readelf

# バイナリのhexdump
make hexdump
```

## プロジェクト構造

```
saba/
├── Cargo.toml              # プロジェクト設定
├── rust-toolchain.toml     # Rustツールチェーン設定
├── Makefile                # ビルドスクリプト
├── run_on_wasabi.sh        # Wasabi OS実行スクリプト
├── src/
│   └── main.rs             # メインアプリケーション
├── build/                  # ビルド成果物（自動生成）
└── target/                 # Cargoビルド成果物
```

## 依存関係

- **noli**: Wasabi OS用のシステムコールライブラリ
  - リポジトリ: https://github.com/hikalium/wasabi.git
  - ブランチ: for_saba

## ライセンス

このプロジェクトのライセンスについては、プロジェクトの管理者にお問い合わせください。

## 貢献

バグ報告や機能提案は、GitHubのIssueまたはPull Requestでお願いします。

## 関連リンク

- [Wasabi OS](https://github.com/hikalium/wasabi)
- [Rust Programming Language](https://www.rust-lang.org/)
