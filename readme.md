# DPUB

dpub = daily publish 简化日常发布工具

## git 分支规划

例如项目：Apple

* master 主分支
* feat/user-login 开发分支
* daily/0.1.1  日常发布分支

## 日常发布手动操作

* 6 个操作
* 5 个 git 仓库操作
* 1 个 CLI 第三方命令调用

```shell
# 提交当前开发分支到远程仓库
git push origin feat/user-login
# 删除本地 daily 分支
git branch -D daily/0.1.1
# 基于当前开发分支创建新的 daily 分支
git checkout -b daily/0.1.1
# 提交 daily 到仓库
git push origin daily/0.1.1 -f
# 执行第三方命令 如云构建
# 这里演示使用 scp 复制文件到服务器的 www 目录
scp . root@1.1.1.1:/var/www
# 操作完成返回开分分支
git checkout feat/user-login
```

## dpub 使用方式

配置文件 `daily.toml`：

* 可能的配置文件位置
* 1. 当前项目根目录
* 2. 参数指定任意目录

```ini
[apple]
dev_tag = "feat/user-login"
daily_tag = "daily/0.1.1"
project_root = "E:\\workspace\\apple"

[orange]
dev_tag = "feat/dashboard"
daily_tag = "daily/1.2.100"
project_root = "E:\\workspace\\orange"
```

```shell
# 发布 apple 项目
dpub apple
# 发布 orange
dpub orange
```

## 功能实现

* 语言 rust
* git仓库操作 git2
* 执行命令 std::process::Command