# my-warp

`my-warp` 是基于官方 [Warp](https://github.com/warpdotdev/warp) 开源仓库维护的个人化 fork，用户可见产品名为 **My Warp**。

这个 fork 的第一阶段目标是中文本地化：让中文系统用户可以在启动引导、设置、高频菜单、命令面板、全局搜索、资源中心和常见交互里看到一致、可维护、可切换的中文界面。后续会继续跟踪上游 Warp 更新，并在这个 fork 中逐步完善隐私默认值、AI UI 弱化、打包分发和更多平台构建。

> [!IMPORTANT]
> `my-warp` 不是官方 Warp，不代表 Warp 官方发布、支持或兼容承诺。它也不是 `Warp Pro`、`Warp Plus` 或任何形式的破解版；本项目不会绕过官方账号、订阅、服务端、同步或 AI 能力限制。

## 当前版本

最新版本：**My Warp OSS v0.1.0**

- Release：[`my-warp-oss-v0.1.0`](https://github.com/slicenferqin/my-warp/releases/tag/my-warp-oss-v0.1.0)
- 构建渠道：`oss`
- macOS 产物：universal DMG
- 包内应用名：`WarpOss.app`
- 签名状态：self-signed / ad-hoc signed，未 notarized

v0.1.0 是首个可下载的中文本地化版本。当前包内仍沿用 `WarpOss.app`、`warp-oss` binary 和 `dev.warp.WarpOss` bundle id；完整改名为 `My Warp.app`、独立 bundle id、URL scheme 和图标属于后续发布工程任务。

## 下载与安装

从 [GitHub Releases](https://github.com/slicenferqin/my-warp/releases) 下载最新 DMG。

macOS 首次打开时可能会被 Gatekeeper 拦截，这是因为首版构建没有使用 Apple Developer ID 做 notarization。可以在系统设置中手动允许打开，或通过 Finder 右键打开完成确认。

校验 DMG：

```bash
shasum -a 256 My-Warp-my-warp-oss-v0.1.0-macos-universal-unnotarized.dmg
```

v0.1.0 DMG 的 SHA256：

```text
3add564aebe02fe1e1a6378035b9a4bf185fe89cd38de265bc8ab2805a35e388
```

## 汉化范围

当前版本重点覆盖高频客户端 UI：

- 首次启动与登录引导。
- Settings 设置页、语言切换与常见设置项。
- 应用菜单、上下文菜单和 macOS 顶部菜单中的高频项。
- 通用按钮、确认弹窗、toast、空状态和基础错误提示。
- 命令面板、命令搜索、欢迎面板和全局搜索。
- 资源中心、标签页配置、代码审查入口和部分 AI 入口 UI。

本地翻译层只负责客户端 UI 字符串。以下内容不会被本地翻译层改写：

- 终端命令输出、shell 提示和 PTY 内容。
- AI 模型回复、Agent 运行结果和工具调用输出。
- 服务端返回内容、账号服务文案、公告、计费或同步对象。
- 用户项目文件、命令名、路径、协议名和可复制代码。

## 本地构建

准备环境：

```bash
./script/bootstrap
```

运行开发版：

```bash
./script/run
```

构建 OSS release DMG：

```bash
./script/bundle --channel oss --release-tag my-warp-oss-v0.1.0 --selfsign
```

i18n 校验：

```bash
cargo xtask check-i18n --check-parity
cargo xtask check-i18n --mode hard
```

发布流程见 [docs/my-warp-release.md](docs/my-warp-release.md)，汉化与 fork 维护约束见 [docs/my-warp-i18n.md](docs/my-warp-i18n.md)。

## 维护策略

`my-warp` 会长期跟踪官方 Warp 上游：

1. 先保持 fork 能跟随上游 `master` 更新。
2. 再维护 `warp_i18n` 资源、key parity 和裸字符串检查。
3. 每次大范围上游同步后，优先修复构建、启动、Settings、onboarding 和高频命令路径。
4. 发布前通过 i18n 校验、release 编译检查和本机首次引导验证。

如果参考或复用 `Heartcoolman/warp-cn` 的翻译、代码或实现结构，相关提交或 PR 需要明确记录来源 commit。仅作为调研参考时，也应在变更说明中写清参考范围。

## 与官方 Warp 的关系

本项目保留官方 Warp 的上游来源、版权、许可证和必要署名。官方项目仍位于 [warpdotdev/warp](https://github.com/warpdotdev/warp)。

需要官方文档、账号支持、正式发布版或官方 Preview 版时，请使用 Warp 官方渠道。

## 许可证

本仓库继承上游 Warp 的许可证边界：

- `warpui_core` 和 `warpui` crates 使用 [MIT license](LICENSE-MIT)。
- 其余代码使用 [AGPL v3](LICENSE-AGPL)。

fork 发布、源码分发和二进制分发都需要继续遵守这些许可证要求，并保留第三方依赖许可证说明。
