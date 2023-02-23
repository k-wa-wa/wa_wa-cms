---
title: "JavaScriptのsplitメソッドで詰まった話"
emoji: "💡"
type: "tech"
topics: ["javascript", "typescript", "python"]
published: true
published_at: 2023-02-23 12:55
---

今回は短めですが、js を使っている中で、困った点があったので、書き留めておきます。

## 結論

Python でいう、

```py
str_1, str_2, str_3 = "a,b,c,d,e,...".split(",", 2)
```

は、

```js
×
const [str_1, str_2, str_3] = "a,b,c,d,e,...".split(",", 2);

○
const [str_1, str_2, ..._str_3] = "a,b,c,d,e,...".split(",");
const str_3 = _str_3.join(",");
```

とする！

以下でもう少し詳細に説明します。

## 詰まりポイント

csv を扱っていた際に、`"a,b,c,d,e,..."`という文字列があり、`"a", "b", "c,d,e,..."`のように分割したいと思っていました。

Python だと以下のように書けます。

```py
>>> str_1, str_2, str_3 = "a,b,c,d,e,...".split(",")
>>> print(str_1)
a
>>> print(str_2)
b
>>> print(str_3)
c,d,e,...
```

split メソッドのドキュメントを見てみると、`str.split(sep=None, maxsplit=-1)`となっており、`maxsplit`で分割する回数を指定できます。

## js でも書いてみよう！

Python での知識をもとに、同じノリで思考停止で書きました。
js にも`String.prototype.split()`というメソッドが用意されています。

```js
> const [str_1, str_2, str_3] = "a,b,c,d,e,...".split(",", 2);
> console.log(str_1);
'a'
> console.log(str_2);
'b'
> console.log(str_3);
undefined
```

おやおや？？？

`limit`(Python でいう`maxsplit`)の数え方が違うのでしょうか。

```js
> const [str_1, str_2, str_3] = "a,b,c,d,e,...".split(",", 3);
> console.log(str_3);
'c'
```

あれれ〜

この時点でドキュメントを見に行けば良かったのですが、血迷った末、他の箇所の原因などを探りながら 1 時間近くを無駄にしました。。。

## じゃあどうすればいいの？

`String.prototype.split()`の[ドキュメント](https://developer.mozilla.org/ja/docs/Web/JavaScript/Reference/Global_Objects/String/split)には、このように書いていました。

> 非負の整数で、分割する数を制限します。指定された場合、文字列は separator が現れるたびに分割されますが、 limit の数の項目が配列に配置されると停止します。残りのテキストは配列に入りません。

めちゃくちゃちゃんと書いてある。。。**「残りのテキストは配列に入りません。」** とのことでした。

一旦全て分割してから、再度繋げるというのが良さそうです。

```js
const [str_1, str_2, ..._str_3] = "a,b,c,d,e,...".split(","); // _str_3 = ['c', 'd', 'e', '...']
const str_3 = _str_3.join(",");
```

`...`をつけることによって、残りの要素を配列として取れるようにしています。(`...`を忘れると、`c`しか撮れないので注意)

これで思い通りに文字をとってくることができました！

```js
> console.log(str_3);
'c,d,e,...'
```

## まとめ

**ドキュメントはちゃんと読みましょう。**
他言語の知識で脳死で書いていると痛い目を見ます。
