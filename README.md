# leapgate-cardo

[English](#english) | [中文](#中文)

## English

The shared library of [Leapgate](https://github.com/Ogannesson/leapgate), a remote desktop built on RustDesk. This repository is a fork of [rustdesk/hbb_common](https://github.com/rustdesk/hbb_common), the configuration, protocol and networking code shared by the RustDesk client and server. The name comes from the Latin *cardo*, the pivot a door turns on: one side holds the door, the other the frame, much as this library connects the client and the server.

Leapgate has not been released. This repository is not meant to be used on its own.

### Branches

| Branch | Used by |
|---|---|
| `main` | Upstream base point; only this README added |
| `leapgate/server` | [leapgate-server](https://github.com/Ogannesson/leapgate-server) |
| `leapgate/client` | [leapgate-client](https://github.com/Ogannesson/leapgate-client) |

Each Leapgate branch is checked out as a submodule at `libs/hbb_common` in the repository that uses it. Clone those repositories, or the [leapgate](https://github.com/Ogannesson/leapgate) umbrella repository, with `--recurse-submodules` instead of cloning this one directly.

### What is changed

- The crate is still named `hbb_common`, so upstream code that uses it compiles without changes.
- Protocol fields added by Leapgate in `rendezvous.proto` are numbered from 1000, away from numbers upstream may use later. Protocol changes are committed to both branches with identical content.
- On the client branch, the default rendezvous server and its public key are injected at build time (`LEAPGATE_RENDEZVOUS_SERVER`, `LEAPGATE_RS_PUB_KEY`) instead of pointing to RustDesk's public servers.
- On the client branch, `APP_NAME` is changed to `Leapgate`, which gives the client its own configuration directory, and the "Powered by RustDesk" line is hidden by default.

The design behind these changes is described, in Chinese, in `docs/architecture.md` of the umbrella repository.

### License

Upstream hbb_common has no license file of its own; it is a submodule of RustDesk, which is licensed under AGPL-3.0. Leapgate's changes here are released under AGPL-3.0. Leapgate is not affiliated with or endorsed by the RustDesk project; the RustDesk name belongs to its owners.

## 中文

[Leapgate（跃门）](https://github.com/Ogannesson/leapgate)的共用库。Leapgate 是基于 RustDesk 改进的远程桌面。本仓库是 [rustdesk/hbb_common](https://github.com/rustdesk/hbb_common) 的 fork，hbb_common 是 RustDesk 客户端和服务端共用的配置、协议和网络代码。名字取自拉丁语 cardo，意思是门枢：门靠它转动，一头连着门、一头连着门框，正像这个库连着客户端和服务端。

Leapgate 还没有发布。本仓库不单独使用。

### 分支

| 分支 | 使用方 |
|---|---|
| `main` | 上游基点，只加了本 README |
| `leapgate/server` | [leapgate-server](https://github.com/Ogannesson/leapgate-server) |
| `leapgate/client` | [leapgate-client](https://github.com/Ogannesson/leapgate-client) |

两条 Leapgate 分支各自以子模块的形式挂在使用方仓库的 `libs/hbb_common` 下。请用 `--recurse-submodules` 克隆使用方仓库或总仓库 [leapgate](https://github.com/Ogannesson/leapgate)，不要直接克隆本仓库。

### 改了什么

- crate 名仍是 `hbb_common`，上游引用它的代码不用改就能编译。
- Leapgate 在 `rendezvous.proto` 里新加的字段从 1000 开始编号，避开上游以后可能用到的号段。协议改动在两条分支上各提交一份，内容一致。
- 客户端分支的默认会合服务器和公钥改为构建时注入（`LEAPGATE_RENDEZVOUS_SERVER`、`LEAPGATE_RS_PUB_KEY`），不再指向 RustDesk 的公共服务器。
- 客户端分支把 `APP_NAME` 改为 `Leapgate`，客户端因此有自己的配置目录；并默认隐藏 "Powered by RustDesk"。

这些改动的设计依据写在总仓库的 `docs/architecture.md` 里，只有中文。

### 许可证

上游 hbb_common 自身没有许可证文件，它是 RustDesk 的子模块，RustDesk 采用 AGPL-3.0。Leapgate 在这里的改动以 AGPL-3.0 发布。Leapgate 与 RustDesk 项目没有关联，也未获其认可；RustDesk 这个名字归其所有者。
