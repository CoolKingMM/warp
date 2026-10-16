# Project Instructions

本文件是本仓库的项目级指令，优先级高于全局默认习惯；若与更高层系统/开发者指令冲突，以更高层指令为准。

## 语言与范围

- 与用户的自然语言沟通使用简体中文。
- 代码、命令、路径、workflow 名称、artifact 名称、环境变量和错误信息保持英文原样。
- 修改应遵循最小变更原则，避免无关重构和格式化噪音。

## GitHub Actions 构建要求

每次功能变动完成后，必须优先通过 GitHub Actions 编译验证，并下载构建产物到本地进行测试。

- 默认使用 `.github/workflows/build-windows-oss.yml`，workflow 名称为 `Build Windows OSS`。
- 对普通分支或手动验证，使用 `workflow_dispatch` 触发构建。
- 非 tag 的 `workflow_dispatch` 构建产物为 `WarpOssPortable-windows-x64`。
- 下载产物后，必须在本地对下载到的 artifact 做测试；不要只依赖 workflow 成功状态。
- 若功能变动只存在于本地且尚未推送，必须说明 GitHub Actions 无法编译未推送变更；除非用户明确要求，否则不得自动 `git commit`、`git push` 或修改远端分支。

推荐流程：

```powershell
gh workflow run "Build Windows OSS" --ref <branch>
gh run list --workflow "Build Windows OSS" --branch <branch> --limit 5
gh run view <run-id> --json status,conclusion,url
gh run download <run-id> --name WarpOssPortable-windows-x64 --dir .tmp/actions/<run-id>/WarpOssPortable
```

本地测试应根据变更内容选择可验证路径。至少需要确认 artifact 下载成功、关键可执行文件存在，并执行与本次功能变动相关的 smoke test 或手动验证步骤。若测试需要登录、外部服务、系统权限或图形交互而当前环境无法完成，必须在最终回复中明确说明未覆盖的验证项。

## Action 轮询限制

当 GitHub Actions 正在编译时，单轮轮询总时长不得超过 5 分钟。

- 可以轮询 run 状态，但一次等待窗口必须控制在 5 分钟以内。
- 建议每 15-30 秒查询一次状态，最多持续 5 分钟。
- 5 分钟内若 workflow 未完成，必须停止本轮轮询，并向用户报告当前状态、run URL 和下一步建议。
- 不得为了等待构建完成而无限循环、长时间占用会话或声称已经完成下载/本地测试。
- 用户要求继续等待时，可以再次开启新的轮询窗口，但每轮仍不得超过 5 分钟。

## 完成标准

涉及功能变动的任务，最终回复必须包含：

- 已修改内容摘要。
- GitHub Actions run 的结果和 URL；若未能触发或未完成，说明原因。
- artifact 下载位置；若未能下载，说明原因。
- 本地测试结果；若未能测试，说明阻塞条件和残余风险。

