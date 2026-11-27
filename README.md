<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Librel%20DeepSeek%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Wallet%20Sign-In%20%7C%20API%20Key%20Creation%20%7C%20Batch%20Enroll&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Solana+Wallet+Sign-In+%7C+No+Account+Needed;Wallet+Signature+Challenge+%7C+Session+Cookie;API+Key+Creation+%7C+apikey.txt;Batch+Enroll+%7C+Probe+%7C+Proxy+Support"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Lebrel%20AI-111111"/>
  <img alt="build" src="https://github.com/Yuurisan-N1/Librel-Deepseek/actions/workflows/build.yml/badge.svg"/>
  <img alt="proxy" src="https://img.shields.io/badge/Proxy-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Librel DeepSeek Bot</b> is an account automation client for Lebrel AI.<br/>
  It generates a fresh Solana keypair for every account, signs the challenge message the server issues, obtains a session cookie, and creates an API key that is appended to <code>apikey.txt</code>. No existing account or seed phrase is needed. Use create mode to register any number of accounts in one run, and probe mode to verify the host and every proxy before enrolling.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Bot](#running-the-bot)
- [Run Modes](#run-modes)
- [Features](#features)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Git
- A supported platform: Linux x86_64, Windows x86_64, or macOS Apple Silicon

No compiler or runtime is needed. The bot ships as a prebuilt binary.

---

## Installation

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Librel-Deepseek.git
cd Librel-Deepseek
```

**Download the binary for your platform from the latest release:**

https://github.com/Yuurisan-N1/Librel-Deepseek/releases/latest

| File | Platform | Link |
|---|---|---|
| `lebrel-bot-linux-x86_64` | Linux x86_64 | [Download](https://github.com/Yuurisan-N1/Librel-Deepseek/releases/latest/download/lebrel-bot-linux-x86_64) |
| `lebrel-bot-macos-aarch64` | macOS Apple Silicon | [Download](https://github.com/Yuurisan-N1/Librel-Deepseek/releases/latest/download/lebrel-bot-macos-aarch64) |
| `lebrel-bot-windows-x86_64.exe` | Windows x86_64 | [Download](https://github.com/Yuurisan-N1/Librel-Deepseek/releases/latest/download/lebrel-bot-windows-x86_64.exe) |

Place the downloaded binary in the same folder as `proxy.txt` and `config.json`.

**Linux / macOS — mark executable:**

```bash
chmod +x lebrel-bot-linux-x86_64
```

---

## Configuration

### 1. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to connect directly):

```
http://user:pass@host:port
socks5://user:pass@host:port
```

Proxies are assigned to accounts by index in round-robin order. Credentials are masked before logging.

### 2. Bot Settings (config.json)

`sleep_seconds` controls how many seconds the bot waits between cycles in run mode. The value is clamped between 60 and 86400 seconds. The default is 3600 (one hour). If `config.json` is missing it is created automatically.

---

## Running the Bot

Make `run.sh` executable first (Linux / macOS):

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh create <n>     # generate n wallets, sign in each, and append every key to apikey.txt
./run.sh run            # run one cycle in foreground
./run.sh bg             # run in background, logs to lebrel-bot.log
./run.sh bgprobe        # run probe in background
./run.sh screen         # run in a detached screen session
./run.sh stop           # stop the background or screen session
./run.sh probe          # check the host and every proxy without writing anything
./run.sh selftest       # offline checks, no network
```

For background mode, tail the log anytime:

```bash
tail -f lebrel-bot.log
```

For screen mode, attach anytime:

```bash
screen -r lebrel-bot
```

Press `Ctrl+C` once to stop the bot cleanly when running in foreground.

### Manual run

```bash
# Linux / macOS
./lebrel-bot-linux-x86_64 create 5

# Windows
.\lebrel-bot-windows-x86_64.exe create 5
```

---

## Run Modes

| Mode | Command | Description |
|---|---|---|
| create | `./run.sh create <n>` | Generate n wallets, sign in each, append every key to apikey.txt |
| run | `./run.sh run` | Run one cycle with the keys already in apikey.txt |
| bg | `./run.sh bg` | Same as run but in background with log file |
| bgprobe | `./run.sh bgprobe` | Probe in background |
| screen | `./run.sh screen` | Same as run in a detached screen session |
| probe | `./run.sh probe` | Check the host and every proxy without writing anything |
| selftest | `./run.sh selftest` | Offline checks, no network |
| stop | `./run.sh stop` | Stop background or screen session |

---

## Features

### Solana Wallet Sign-In
The bot generates a fresh Ed25519 Solana keypair for every account. No existing wallet, seed phrase, or account is needed. The public key is used as the account address in the sign-in flow.

### Wallet Signature Challenge
The site issues a nonce and a sign-in message. The bot signs the message locally with the generated private key and submits the signature to verify ownership. The session cookie returned by the server is used for all subsequent requests.

### API Key Creation
After sign-in the bot creates an API key named `lebrel-bot-key` under the session and appends the full key secret to `apikey.txt`. The key prefix is logged in masked form. The wallet address, private key, account id, and key details are saved to `session.json` for reference.

### Batch Enroll
Running `create <n>` generates exactly n wallets and signs each one into a fresh account, writing every key to `apikey.txt` as it completes. Each account uses its own proxy slot from `proxy.txt`.

### Probe Mode
Probe checks the Lebrel AI host and reports the exit country of every proxy in `proxy.txt` without signing up or modifying any files.

### Proxy Support
Proxies are loaded from `proxy.txt` and assigned to accounts by position in round-robin order. Running without proxies is fully supported.

---

## File Structure

```text
Librel-Deepseek/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 3-platform matrix build and test
├── assets/
│   ├── icon.ico                         # Windows binary icon
│   └── icon.png                         # App icon
├── src/
│   ├── cli/
│   │   ├── create.rs                    # Create mode: batch wallet signup (distributed separately)
│   │   ├── mod.rs                       # CLI mode dispatch
│   │   ├── probe.rs                     # Probe mode: host and proxy check
│   │   └── selftest.rs                  # Offline self-test checks
│   ├── job/
│   │   ├── mod.rs
│   │   ├── onboard.rs                   # Per-account wallet sign-in and key creation
│   │   └── run_loop.rs                  # Cycle runner
│   ├── net/
│   │   ├── client.rs                    # HTTP client and cookie jar
│   │   ├── egress.rs                    # Proxy loader and exit country check
│   │   └── mod.rs
│   ├── step/
│   │   ├── apikey.rs                    # API key creation request
│   │   ├── mod.rs
│   │   ├── signin.rs                    # Challenge fetch and signature verify (distributed separately)
│   │   └── wallet.rs                    # Solana keypair generation and Ed25519 signing (distributed separately)
│   ├── support/
│   │   ├── banner.rs                    # ASCII banner and terminal title
│   │   ├── cfg/
│   │   │   ├── loader.rs                # Config and proxy loader
│   │   │   ├── mod.rs
│   │   │   └── schema.rs                # Config schema and sleep setting
│   │   ├── consts/
│   │   │   ├── app.rs                   # File name constants and key name
│   │   │   ├── mod.rs
│   │   │   └── net.rs                   # Base URLs and API path constants (distributed separately)
│   │   ├── device.rs                    # Browser device profile builder
│   │   ├── error.rs                     # Error types and result alias
│   │   ├── log/
│   │   │   ├── format.rs                # Colored log helpers and countdown
│   │   │   ├── init.rs                  # Signal handler setup
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── wire/
│   │   ├── endpoint.rs                  # API endpoint path constants
│   │   ├── mod.rs
│   │   └── shape.rs                     # JSON response parsers
│   └── main.rs                          # Entry point, Ctrl+C handler, mode dispatch
├── .gitignore
├── build.rs                             # Build script: embeds icon into Windows binary
├── Cargo.lock
├── Cargo.toml                           # Project manifest and dependencies
├── config.json                          # Sleep duration setting
├── HOWTO.txt                            # Quick start guide
├── LICENSE                              # GNU AGPL v3.0
├── Makefile                             # make targets: build, release, run, start, clean, size
├── proxy.txt                            # Proxy list (optional)
├── README.md                            # This file
├── run.sh                               # Run helper: create, run, bg, bgprobe, screen, probe, selftest, stop
├── apikey.txt                           # Collected API keys, one per line (auto-generated)
└── session.json                         # Wallet address, key, and account details (auto-generated)
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>