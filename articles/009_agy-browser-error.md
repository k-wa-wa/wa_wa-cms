---
title: "Tailscale環境下でAntigravityのブラウザツールが使えなかった"
emoji: "👀"
type: "tech"
topics: ["antigravity"]
published: true
published_at: 2026-07-06 19:01
---

## 1. 発生した問題

ブラウザツール（`browser_subagent`）の起動および動作確認時に、以下のエラーが発生し、ブラウザの制御およびページ遷移ができない状態となった。

- **名前解決エラー**:
  ```
  failed to create browser context: failed to resolve CDP URLs: get CDP version info: could not resolve IP for 127.0.0.1
  ```
- **プロトコルエラー（Tailscaleオン/オフ切り替え後に発生）**:
  ```
  playwright: Protocol error (Browser.setDownloadBehavior): Browser context management is not supported.
  ```

---

## 2. 原因

### ① `127.0.0.1` の名前解決失敗
- **不要なDNS問い合わせ**:
  ブラウザの制御ツール（Playwright等）が起動する際、制御用プロトコル（CDP）接続のために `127.0.0.1` への接続を試みる。この際、内部ライブラリが `127.0.0.1` というIPアドレス（またはその逆引き）に対してDNS名前解決を要求する。
- **Tailscale DNSの応答拒否**:
  Tailscale接続中は、すべてのDNSクエリが優先DNS（`100.100.100.100`）へ転送される。Tailscaleのデフォルト状態（Global Nameservers未設定）では、プライベートドメイン（`*.ts.net`）以外の名前解決要求に対してエラー（`recursion not available` / 再帰不可）を返す。
- **リゾルバの仕様とハング**:
  Go言語などの組み込みDNSリゾルバ（Pure Go リゾルバなど）は、このエラー応答を受け取ると、次のDNSサーバーへフォールバックを行わずに処理を打ち切る。その結果、`127.0.0.1` 自体の解決に失敗する。
- **普段の利用で問題がない理由**:
  一般的なアプリケーション（`curl` など）やWebブラウザはOS標準の名前解決API（`getaddrinfo` など）を利用しており、エラー時も次のDNSサーバーへ正常にフォールバックするため、問題が表面化しない。

### ② プロトコルエラー
- Tailscaleの切断・再接続を行った際、ポート `9222`（CDP用ポート）でリスニングしていた古いブラウザプロセスが終了せず残存した。
- これにより、新しく接続しようとした制御ツールと古いブラウザセッションとの間で整合性が取れなくなり、プロトコルエラー（`Browser context management is not supported`）が発生した。

---

## 3. 解消方法

### DNSエラーの解消
1. Tailscaleの管理画面（Admin Console -> DNS）にアクセスする。
2. **Nameservers** セクションの「Add nameserver」から `Custom` を選択する。
3. パブリックDNS（Google Public DNS `8.8.8.8` / `8.8.4.4` や Cloudflare `1.1.1.1`）を追加する。
4. これにより、`100.100.100.100` が再帰問い合わせをパブリックDNSに委任して正常に応答を返すようになるため、名前解決エラーが解消する。

### プロトコルエラーの解消
1. 開いているブラウザウィンドウ（Antigravity Browser Control）をすべて閉じる。
2. ターミナルで以下のコマンドを実行し、ポート `9222` を使用している残存プロセスを確認する。
   ```bash
   lsof -i :9222
   ```
3. 対象のプロセスID（PID）を確認し、強制終了する。
   ```bash
   kill <PID>
   ```
4. ブラウザプロセスが完全に終了した状態で、再度ツールを実行する。
