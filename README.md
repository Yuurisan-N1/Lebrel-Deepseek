<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Lebrel%20DeepSeek%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Wallet%20Sign-In%20%7C%20API%20Key%20Creation%20%7C%20Batch%20Enroll&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Solana+Wallet+Sign-In+%7C+No+Account+Needed;Wallet+Signature+Challenge+%7C+Session+Cookie;API+Key+Creation+%7C+apikey.txt;Batch+Enroll+%7C+Probe+%7C+Proxy+Support"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Lebrel%20AI-111111"/>
  <img alt="build" src="https://github.com/Yuurisan-N1/Lebrel-Deepseek/actions/workflows/build.yml/badge.svg"/>
  <img alt="proxy" src="https://img.shields.io/badge/Proxy-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Lebrel DeepSeek Bot</b> は、Lebrel AI 向けのアカウント自動化クライアントやで。<br/>
  アカウントごとに新しい Solana キーペアを生成して、サーバーが発行するチャレンジメッセージに署名して、セッションクッキーをもらって、API キーを作って <code>apikey.txt</code> に追記するんや。既存のアカウントもシードフレーズも要らへん。create モードで何個でもアカウントを一発で登録できるし、probe モードで登録前にホストとプロキシを全部確認できるで。<br/>
  作ったんは <b>Yuurisandesu</b> やで。
</p>

</div>

---

## 目次

- [必要なもの](#requirements)
- [インストール](#installation)
- [設定](#configuration)
- [ボットの起動](#running-the-bot)
- [実行モード](#run-modes)
- [機能](#features)
- [ファイル構成](#file-structure)
- [免責事項](#disclaimer)

---

## 必要なもの

- Git
- 対応プラットフォーム：Linux x86_64、Windows x86_64、macOS Apple Silicon

コンパイラもランタイムも要らへん。ボットはビルド済みのバイナリで配布してるで。

---

## インストール

**リポジトリをクローンする：**

```bash
git clone https://github.com/Yuurisan-N1/Lebrel-Deepseek.git
cd Lebrel-Deepseek
```

**最新リリースからプラットフォームに合ったバイナリをダウンロードしてな：**

https://github.com/Yuurisan-N1/Lebrel-Deepseek/releases/latest

| ファイル | プラットフォーム | リンク |
|---|---|---|
| `lebrel-bot-linux-x86_64` | Linux x86_64 | [ダウンロード](https://github.com/Yuurisan-N1/Lebrel-Deepseek/releases/latest/download/lebrel-bot-linux-x86_64) |
| `lebrel-bot-macos-aarch64` | macOS Apple Silicon | [ダウンロード](https://github.com/Yuurisan-N1/Lebrel-Deepseek/releases/latest/download/lebrel-bot-macos-aarch64) |
| `lebrel-bot-windows-x86_64.exe` | Windows x86_64 | [ダウンロード](https://github.com/Yuurisan-N1/Lebrel-Deepseek/releases/latest/download/lebrel-bot-windows-x86_64.exe) |

ダウンロードしたバイナリは `proxy.txt` と `config.json` と同じフォルダに置いてな。

**Linux / macOS — 実行権限をつける：**

```bash
chmod +x lebrel-bot-linux-x86_64
```

---

## 設定

### 1. プロキシ（proxy.txt）

`proxy.txt` にプロキシを1行ずつ書いてな（任意、空のままでも直接つながるで）：

```
http://user:pass@host:port
socks5://user:pass@host:port
```

プロキシはアカウントのインデックス順でラウンドロビンに割り当てられるで。ログには認証情報はマスクして出るで。

### 2. ボット設定（config.json）

`sleep_seconds` は run モードでのサイクル間の待機秒数を指定するんや。値は 60〜86400 秒の間にクランプされるで。デフォルトは 3600（1時間）やで。`config.json` がなかったら自動で作られるで。

---

## ボットの起動

まず Linux / macOS では `run.sh` に実行権限をつけてな：

```bash
chmod +x run.sh
```

あとはモードを選んで実行するだけや：

```bash
./run.sh create <n>     # n 個のウォレットを生成して、それぞれサインインして、全キーを apikey.txt に追記するで
./run.sh run            # フォアグラウンドで1サイクル実行するで
./run.sh bg             # バックグラウンドで実行、ログは lebrel-bot.log に出るで
./run.sh bgprobe        # バックグラウンドで probe を実行するで
./run.sh screen         # デタッチした screen セッションで実行するで
./run.sh stop           # バックグラウンドや screen セッションを止めるで
./run.sh probe          # 何も書き込まずにホストと全プロキシを確認するで
./run.sh selftest       # ネットワークなしのオフラインチェックやで
```

バックグラウンドモードのログは好きなときに確認できるで：

```bash
tail -f lebrel-bot.log
```

screen モードはいつでもアタッチできるで：

```bash
screen -r lebrel-bot
```

フォアグラウンドで動いてるときは `Ctrl+C` 1回できれいに止まるで。

### 手動実行

```bash
# Linux / macOS
./lebrel-bot-linux-x86_64 create 5

# Windows
.\lebrel-bot-windows-x86_64.exe create 5
```

---

## 実行モード

| モード | コマンド | 説明 |
|---|---|---|
| create | `./run.sh create <n>` | n 個のウォレットを生成して、それぞれサインインして、全キーを apikey.txt に追記するで |
| run | `./run.sh run` | apikey.txt にあるキーで1サイクル実行するで |
| bg | `./run.sh bg` | run と同じやけどバックグラウンドでログファイル付きで動くで |
| bgprobe | `./run.sh bgprobe` | バックグラウンドで probe するで |
| screen | `./run.sh screen` | デタッチした screen セッションで run するで |
| probe | `./run.sh probe` | 何も書き込まずにホストと全プロキシを確認するで |
| selftest | `./run.sh selftest` | ネットワークなしのオフラインチェックやで |
| stop | `./run.sh stop` | バックグラウンドや screen セッションを止めるで |

---

## 機能

### Solana ウォレットサインイン
アカウントごとに新しい Ed25519 Solana キーペアを生成するで。既存のウォレットもシードフレーズもアカウントも要らへん。公開鍵がサインインフローのアカウントアドレスとして使われるんや。

### ウォレット署名チャレンジ
サイトがノンスとサインインメッセージを発行するで。ボットは生成した秘密鍵でローカルにメッセージへ署名して、署名を送信して所有権を証明するんや。サーバーが返すセッションクッキーはその後の全リクエストで使うで。

### API キー作成
サインイン後、ボットはセッション下で `lebrel-bot-key` という名前の API キーを作って、フルキーシークレットを `apikey.txt` に追記するで。キープレフィックスはマスクしてログに出るで。ウォレットアドレス、秘密鍵、アカウント ID、キーの詳細は参照用に `session.json` に保存されるで。

### バッチ登録
`create <n>` を実行すると n 個のウォレットを生成して、それぞれ新しいアカウントにサインインして、完了するたびに全キーを `apikey.txt` に書き込むで。各アカウントは `proxy.txt` から自分のプロキシスロットを使うで。

### Probe モード
probe はサインアップやファイル変更なしで、Lebrel AI ホストを確認して `proxy.txt` の全プロキシの出口国をレポートするで。

### プロキシ対応
プロキシは `proxy.txt` から読み込まれて、位置順でアカウントにラウンドロビン割り当てされるで。プロキシなしでも全然動くで。

---

## ファイル構成

```text
Lebrel-Deepseek/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 3プラットフォームのマトリックスビルドとテスト
├── assets/
│   ├── icon.ico                         # Windows バイナリアイコン
│   └── icon.png                         # アプリアイコン
├── src/
│   ├── cli/
│   │   ├── create.rs                    # create モード：バッチウォレットサインアップ（別途配布）
│   │   ├── mod.rs                       # CLI モードディスパッチ
│   │   ├── probe.rs                     # probe モード：ホストとプロキシチェック
│   │   └── selftest.rs                  # オフラインセルフテストチェック
│   ├── job/
│   │   ├── mod.rs
│   │   ├── onboard.rs                   # アカウントごとのウォレットサインインとキー作成
│   │   └── run_loop.rs                  # サイクルランナー
│   ├── net/
│   │   ├── client.rs                    # HTTP クライアントとクッキージャー
│   │   ├── egress.rs                    # プロキシローダーと出口国チェック
│   │   └── mod.rs
│   ├── step/
│   │   ├── apikey.rs                    # API キー作成リクエスト
│   │   ├── mod.rs
│   │   ├── signin.rs                    # チャレンジ取得と署名検証（別途配布）
│   │   └── wallet.rs                    # Solana キーペア生成と Ed25519 署名（別途配布）
│   ├── support/
│   │   ├── banner.rs                    # ASCII バナーとターミナルタイトル
│   │   ├── cfg/
│   │   │   ├── loader.rs                # 設定とプロキシローダー
│   │   │   ├── mod.rs
│   │   │   └── schema.rs                # 設定スキーマとスリープ設定
│   │   ├── consts/
│   │   │   ├── app.rs                   # ファイル名定数とキー名
│   │   │   ├── mod.rs
│   │   │   └── net.rs                   # ベース URL と API パス定数（別途配布）
│   │   ├── device.rs                    # ブラウザデバイスプロファイルビルダー
│   │   ├── error.rs                     # エラータイプと結果エイリアス
│   │   ├── log/
│   │   │   ├── format.rs                # 色付きログヘルパーとカウントダウン
│   │   │   ├── init.rs                  # シグナルハンドラセットアップ
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── wire/
│   │   ├── endpoint.rs                  # API エンドポイントパス定数
│   │   ├── mod.rs
│   │   └── shape.rs                     # JSON レスポンスパーサー
│   └── main.rs                          # エントリーポイント、Ctrl+C ハンドラ、モードディスパッチ
├── .gitignore
├── build.rs                             # ビルドスクリプト：Windows バイナリにアイコンを埋め込む
├── Cargo.lock
├── Cargo.toml                           # プロジェクトマニフェストと依存関係
├── config.json                          # スリープ時間設定
├── HOWTO.txt                            # クイックスタートガイド
├── LICENSE                              # GNU AGPL v3.0
├── Makefile                             # make ターゲット：build、release、run、start、clean、size
├── proxy.txt                            # プロキシリスト（任意）
├── README.md                            # このファイルや
├── run.sh                               # 実行ヘルパー：create、run、bg、bgprobe、screen、probe、selftest、stop
├── apikey.txt                           # 収集した API キー、1行1キー（自動生成）
└── session.json                         # ウォレットアドレス、キー、アカウント詳細（自動生成）
```

---

## 免責事項

このツールは教育目的と技術探求のために作ったもんやで。賢く使って、自己責任でよろしくな。

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>