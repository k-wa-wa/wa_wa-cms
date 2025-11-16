---
title: "Proxmox向けCluster API採用を断念した話"
emoji: "💡"
type: "tech"
topics: ["kubernetes", "proxmox", "cluster-api"]
published: true
published_at: 2025-09-01
---

> [!WARNING]
> この記事はほぼ自分用のメモであり、AI を使用して書いている。

自宅の Kubernetes クラスタは、Proxmox VE 上に Ansible で構築・破棄を繰り返す運用をしていた。これをより宣言的に管理し、ノード追加などを容易にしたいと考え、Kubernetes の Cluster API を Proxmox 環境に導入できないかと検討を始めた。

Proxmox VE 対応の Cluster API プロバイダをいくつか試したが、残念ながら複数の課題に直面し、最終的に導入を断念することになった。本記事では、その経緯と断念理由を共有する。

## ionos-cloud/cluster-api-provider-proxmox (capmox) を試した

まず最初に、ionos-cloud が提供している`cluster-api-provider-proxmox (capmox)`を試した。[公式ドキュメントの手順通りに](https://github.com/ionos-cloud/cluster-api-provider-proxmox/blob/0acdcd63ccddb54f9d99a90a4c6c32999bc9c036/docs/Usage.md#quick-start)進めたが、どうも期待通りには動作しなかった。

### 直面した課題

1.  **Secret の自動生成失敗**:
    Proxmox VE への認証情報を含む Kubernetes Secret が自動で作成されず、手動で作成する必要があった。これは手間であった。

2.  **Proxmox API 呼び出しの不調**:
    Kubernetes ノード作成時、`capmox`が Proxmox VE の API を正しく呼び出せていない、または認証情報を適切に読み込めていないようであった。ソースコードを追跡したが、ログ情報が不十分で、結局原因を特定するには至らなかった。

これらの問題にぶつかり、`capmox`でのクラスタ構築は断念せざるを得なかった。

## k8s-proxmox/cluster-api-provider-proxmox (cappx) も試した

次に、`k8s-proxmox`が提供している`cluster-api-provider-proxmox (cappx)`を試した。こちらはクラスタ構築自体は成功したが、いくつか運用上の懸念点が見つかり、採用を見送ることになった。

### 運用上の課題

1.  **Proxmox ノードの全 Active 状態が必須**:
    `cappx`は、クラスタ起動時にすべての Proxmox ノードが Active 状態であることを要求するようであった。筆者の環境では、運用上 Proxmox ノードを 1 つ停止させていることがあったため、これではクラスタ構築ができなかった。この仕様はソースコードを読んで判明したが、起動時以外のノード状態がどう影響するのかは未検証で、懸念点の一つであった。

2.  **DHCP による IP アドレス割り当て**:
    `cappx`で作成される Kubernetes ノードの IP アドレスは DHCP で割り当てられる仕様であった。ノードの自動生成を考えると避けられないのかもしれないが、現在の Ansible による固定 IP 管理と比較すると、どちらが安全で運用しやすいかという点で迷いが生じた。

3.  **ProxmoxClusterTemplate の欠如**:
    `ionos-cloud/capmox`には存在した`ProxmoxClusterTemplate`のような機能が、`k8s-proxmox/cappx`にはなかった。テンプレート化できないとなると、クラスタ構成の汎用的な再利用が難しく、宣言的な管理という当初の目標達成には不十分であると判断した。

これらの理由から、`k8s-proxmox/cappx`も導入を断念することになった。

## まとめと考察

今回の Proxmox 向け Cluster API プロバイダ採用検討から、いくつかの学びがあった。

- **プロダクトの成熟度確認は重要である**:
  特定の環境に特化した Cluster API プロバイダは、まだまだ発展途上のものが多い印象である。ドキュメント不足やデバッグの難しさ、運用上の制約があることも指摘できる。導入を検討する際には、プロダクトの成熟度やコミュニティの活発さを事前に確認することが重要であると認識した。

- **既存運用との比較検討も忘れずに実施すべきである**:
  新しいツールを導入する際は、既存の運用との統合性や、導入によるメリット・デメリットを詳細に比較検討すべきである。特に IP アドレス管理のような基盤部分は、慎重に検討する必要がある。

- **テンプレート化の重要性**:
  宣言的な管理を目指す上で、クラスタ構成のテンプレート化は不可欠であると改めて感じた。`ClusterTemplate`のような機能の有無は、導入判断の大きなポイントとなる。

現状では、引き続き Ansible を用いた運用を継続していくことになりそうだが、将来的には Terraform と Ansible の組み合わせなど、別のツールチェーンでの宣言的アプローチも視野に入れている。

今回の経験が、筆者と同じように Proxmox VE 上での Kubernetes クラスタ管理を検討している方の参考になれば幸いである。
