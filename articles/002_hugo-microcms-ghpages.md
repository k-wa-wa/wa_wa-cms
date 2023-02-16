---
title: "Hugo(+Robust) × MicroCMS × Github Pagesでサクッと静的Webサイト構築！"
emoji: "💻"
type: "tech"
topics: ["hugo", "robust", "microcms", "github pages"]
published: true
published_at: 2023-02-17 01:32
---

# はじめに

最近は寒くなったり暖かくなったり忙しいですね。

今回はやっと重い腰を上げ、ポートフォリオサイトを作ろうかなと思ったのですが。。

わざわざ色々なページを作り込まなくても、ブログ記事だけサクッと手っ取り早く公開できないかなぁと思いたち、Markdownをレンダリングできる静的サイトジェネレーターを探してホスティングまでしてみました。

## 使用技術

### Hugo

https://gohugo.io/

Goで記述された、オープンソースの静的サイトジェネレーターです。

Markdownを使用して簡単にページを作成したり、JavaScriptやCSSを使用して自由にカスタマイズしたりと、柔軟にWebサイトを構築することができます。

fastとmodernをウリにしているようです。

### Robust

https://github.com/dim0627/hugo_theme_robust

Hugoのテーマの一つです。

HugoにはThemeという機能があり、自分で1からスタイルやフォーマットを作成せずとも、他の人が作成したThemeを利用して「いい感じに」Webサイトを作成することができます。

公式ドキュメントにも色々なテーマが載っていますね。

![](/images/002_01.png)

今回使用したRobustは日本人の方が作成されたテーマのようです。

### MicroCMS

https://microcms.io/

MicroCMSはバックエンドを自作せずともブログやECサイトなどのWebサイトを構築できる、Headless CMS (Contents Management System)です。

こちらも日本製です。

他にも、 [Contentful](https://www.contentful.com/) や [Hygraph](https://hygraph.com/) (旧GraphCMS)などといったHeadless CMSがありますが、以前Contentfulを使用したことがあったのと、日本製ということで今回はMicroCMSを採用しています。

無料枠が充実しており、制限はあるものの、無料でもかなり使うことができます。( [料金プラン](https://microcms.io/pricing) )

### Github Pages

https://docs.github.com/ja/pages

こちらは言わずもがな、みんな大好きGithubで静的サイトをホスティングできるサービスです。

今回はHugoのビルドのために、Github Actionsも使用しています。

## アーキテクチャ図

後ほど追加

# 開発！

## Hugoのインストールとセットアップ

環境に合わせてHugoのインストールを行なってください。( [ガイド](https://gohugo.io/installation/) )

Macではbrewで一発でした。

```
brew install hugo
```

[クイックスタート](https://gohugo.io/getting-started/quick-start/) にあるように進めていきます。

```
hubo new site sample
cd sample
```

テーマの追加を行います。公式にあるようにsubmoduleを使用しておきましょう。

```
# サブモジュールを使用するために、gitの初期化が必要
git init
# themes/以下に配置
git submodule add https://github.com/dim0627/hugo_theme_robust themes/robust
```

テーマの追加ができたので、Hugoにどのテーマを使用するかを教えてあげるため、configファイルに追記しましょう。(コマンドではなく、ファイルを開いて追記しても大丈夫です)

```
echo "theme = 'robust'" >> config.toml
```

config.tomlファイルを開くと、以下のようになっているはずです。

```
baseURL = 'http://example.org/'
languageCode = 'en-us'
title = 'My New Hugo Site'
theme = 'robust'
```

では、実際にサーバーを立ち上げて確認してみましょう。

```
hugo server
```

http://localhost:1313 にアクセスして、以下のような画面が見られれば成功です！

![](/images/002_02.png)

## 記事の追加

これだけでは面白くないので、実際に記事を追加してみましょう。最初の `hugo new site sample` コマンドによってcontentsディレクトリが作成されていると思います。

このディレクトリ下にあるファイルが記事とみなされるので、実際に記事を作成してみましょう。

```
# 中身の追加
echo "最初のページ！" >> content/content1.md
```

再度 http://localhost:1313 にアクセスしましょう。(hot reloadされるので、サーバーを上げなおす必要はないです。更新されない際はブラウザのreloadなど試してみてください。)

以下のように一覧の中にコンテンツが追加されていると思います。

記事をクリックすると個別ページに飛べるので試してみてください！

![](/images/002_03.png)

ここまでできたらHugoのセットアップは完了です。

## MicroCMSでのコンテンツ作成

APIテンプレートといって、ブログなどのコンテンツのためのAPIをワンクリックで作成できる便利な機能が追加されています。サクッと作って検証してから、ニーズに合わせてカスタマイズ可能です。

https://blog.microcms.io/api-template/

## MicroCMSとの連携

Hugoはビルド時にcontentディレクトリにあるMarkdownファイルを参照して、ページを生成します。

そのため、リクエストに応じてCMSに問い合わせする、といったことはできません。

今回は、Hugoのビルド前にMicroCMSから全コンテンツを取得し、contentディレクトリにMarkdownとして配置してから、Hugoのビルドを行うこととしました。

こちらは何を使用して実装しても良いのですが、Goを使用してみました。

実装は [こちら](https://github.com/k-wa-wa/wa_wa-techblog/blob/master/microcms/main.go) になります。※microcmsのコンテンツをカスタマイズしているため、構造体は参考にならないかもしれません。

SDKを使用しても良かったのですが、httpモジュールを使用してAPIを叩いてもあまり手間はかからなそうだったため、SDKは使用せずに実装しました。

ポイントは3つの環境変数です。

- `MICROCMS_BASE_URL`
  - APIエンドポイントを指定します。APIプレビュー画面で確認できます。
  - https://{service_name}.microcms.io/api/v1/blog などになると思います。
- `X_MICROCMS_API_KEY`
  - APIキーを指定します。同じくAPIプレビュー画面で確認できます。
- `MD_OUTPUT_DIR`
  - markdownファイルをアウトプットするディレクトリを指定します。今回の場合は ../content を指定しました。

```
cd microcms
go run .
```

で実行すると、プロジェクトルートのcontentディレクトリにMarkdownファイルが作成されます。(MicroCMSではリッチテキストはHTMLで提供されるため、HTMLをMarkdownに変換するライブラリを噛ませています。)

## Hugoで記事を表示する

再度、 `hugo server` を実行すると、MicroCMSにある記事が http://localhost:1313 から確認できると思います。

## Github Pagesにホストする

そろそろ体力が切れてきました。。

最後は [こちら](https://github.com/ko-he-e/ko-techblog/blob/master/.github/workflows/gh-pages.yml) のようにGithub Actionsをセットアップして終わりです。(雑

ポイントは以下です。

- HugoのRobustテーマはsubmodeulで使用していたため、Github Actionsでチェックアウトする際にはsubmoduleも含めてチェックアウトする必要があります。

```
uses: actions/checkout@v3
with:
  submodules: true
```

- Github Pagesへのデプロイは [こちら](https://github.com/peaceiris/actions-hugo) を使用しています。Github Pagesの設定では、gh-pagesブランチを指定しましょう。
- HugoのconfigファイルのbaseUrlを修正する必要があります。( [参照](https://github.com/ko-he-e/ko-techblog/blob/master/config.toml) )こちらについてはまた記事を作成予定です！

最後にWebhookの設定もお好みで！

## 完成！

実際に作成したサイトは [こちら](https://ko-he-e.github.io/ko-techblog/) になります。当記事もホストされています！

# まとめ

最後まで読んでくださり、ありがとうございます。

今回はHugo(+Robust) × MicroCMS × Github Pagesを使用して、静的Webサイトを作成してみました。

比較的簡単な手順で作成することができたのではないでしょうか。(途中少し端折っているところがありすみません。。

引き続き、メインのポートフォリオサイトの作成を進めたいと思います。。

ではまた！
