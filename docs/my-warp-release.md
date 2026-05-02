# My Warp 发布流程

本文档约束 `my-warp` fork 的首版构建与 GitHub Release 发布流程。当前目标是先稳定产出 macOS OSS DMG，让中文本地化版本可以被下载、验证和手动安装；官方多平台发布、自动更新、Developer ID 签名和 notarization 不进入首版范围。

## 发布目标

首版 release 只发布 macOS OSS 桌面应用：

- Release channel：`oss`
- 构建脚本：`./script/bundle --channel oss --release-tag <tag> --selfsign`
- 产物：`My-Warp-<tag>-macos-universal-unnotarized.dmg`
- 校验文件：`SHA256SUMS.txt`
- 发布方式：GitHub Actions 手动触发，默认创建 draft + prerelease

当前 OSS 打包脚本仍会产出 `WarpOss.app`、`warp-oss` binary 和 `dev.warp.WarpOss` bundle id。`My Warp` 是 fork 的用户可见发布名称；app bundle 元数据改名可以作为后续独立任务处理，避免首版 release 同时承担品牌重命名风险。

## 为什么不用官方 Release Workflow

仓库自带的 `.github/workflows/create_release.yml` 是官方 Warp 发布链路，包含 GitHub Release、Sentry、Google Cloud Storage、Developer ID 签名、notarization、官方 release channel 配置和内部 secret。fork 不应复用这条链路做首版分发，因为它依赖官方私有基础设施，也容易在发布说明和渠道语义上混淆 `my-warp` 与官方 Warp。

`my-warp` 使用独立 workflow：

```text
.github/workflows/my_warp_release.yml
```

这条 workflow 只做 fork 需要的事情：

- 校验 tag 命名，避免误用官方 release 名称。
- 校验 i18n parity 与 hard mode。
- 使用 release flags 检查 `warp-oss` 编译。
- 构建 `oss` channel 的 macOS universal DMG。
- 上传 workflow artifact。
- 可选创建或更新 GitHub Release。

## 触发前置条件

GitHub 的 `workflow_dispatch` 手动触发要求 workflow 文件已经存在于默认分支。因此新增或修改 release workflow 后，需要先把对应变更合入 fork 的默认分支，再从 Actions 页面或 `gh workflow run` 触发。

建议首版流程：

1. 确认当前 feature branch 已 rebase 到最新 `master`。
2. 通过 PR 或受控合并把 i18n 实现和 release workflow 合入 fork 的 `master`。
3. 在 GitHub Actions 中选择 `My Warp OSS Release`。
4. Branch 选择要构建的分支，首版建议选择 `master`。
5. `tag_name` 使用 `my-warp-oss-v0.1.0-rc1` 这类 fork 专用 tag。
6. `draft=true`、`prerelease=true`、`publish_release=true`。
7. workflow 成功后下载 DMG，在本机完成手动验收。
8. 验收通过后再编辑 release notes，并决定是否取消 draft 或 prerelease。

CLI 触发示例：

```bash
gh workflow run my_warp_release.yml \
  --ref master \
  -f tag_name=my-warp-oss-v0.1.0-rc1 \
  -f runner_label=macos-26 \
  -f release_name='My Warp OSS v0.1.0-rc1' \
  -f draft=true \
  -f prerelease=true \
  -f publish_release=true
```

## Runner 选择

默认 runner 是 `macos-26`，它和仓库现有 `prepare_environment` action 中的 Xcode 26 配置一致。若 fork 仓库启用了 larger macOS runner，可以在手动触发时选择 `macos-26-xlarge` 来缩短构建时间。

不要使用 `macos-latest` 作为首版默认值；它会随 GitHub 平台更新漂移，且不一定与仓库当前要求的 Xcode 26 环境保持一致。

## 签名与 Gatekeeper 边界

首版 workflow 使用 `--selfsign`：

- CI runner 上没有 Apple Development 证书时，脚本会回退到 ad-hoc signing。
- 不读取官方 Warp Developer ID 证书。
- 不执行 Apple notarization。
- 不向 Sentry 或 Google Cloud Storage 上传产物。

因此 DMG 文件名使用 `unnotarized`，release notes 必须保留 Gatekeeper 提示。用户首次启动时可能需要在 macOS 系统设置中手动允许打开，或通过右键打开完成确认。

后续如要做正式可分发版本，需要另起任务接入自有 Apple Developer ID、hardened runtime、notarization、stapling 和签名验证，而不是复用官方 Warp 的证书或 secret。

## 自动化验收

release workflow 至少执行以下自动化检查：

```bash
cargo xtask check-i18n --check-parity
cargo xtask check-i18n --mode hard
./script/bundle --channel oss --check-only --nosign
./script/bundle --channel oss --release-tag <tag> --selfsign
```

`check-only` 使用和 release bundle 接近的 feature/profile 组合，比普通 `cargo check --bin warp-oss -p warp` 更适合作为发布前编译门槛。

## 手动验收

从 draft release 下载 DMG 后，至少验证：

- DMG 可以挂载，`WarpOss.app` 可以复制到 Applications。
- 首次启动符合未 notarized 应用的预期 Gatekeeper 行为。
- 设置页语言选项可切换 `中文`、`English`、`跟随系统`。
- Settings、onboarding、命令面板、资源中心、全局搜索和高频上下文菜单显示中文。
- 终端命令输出、AI 模型回复和服务端内容没有被本地翻译层改写。
- `shasum -a 256 <DMG>` 与 `SHA256SUMS.txt` 一致。

## 后续扩展

首版稳定后，可以逐步增加：

- 独立的 `My Warp` app bundle name、bundle id、URL scheme 和图标。
- 自有 Apple Developer ID 签名与 notarization。
- Linux / Windows OSS 包。
- Release notes 自动附带 i18n 覆盖率摘要。
- 上游同步后自动构建 nightly draft，但仍不自动发布正式 release。
