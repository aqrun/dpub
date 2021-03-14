use git2::{Repository};
use crate::Config;
use crate::git;
use crate::cmd;

/// 主应用结构体
pub struct App<'a> {
  /// 主配置参数
  pub config: &'a Config,
  /// git 仓库对象
  pub repo: Repository,
}

impl<'a> App<'a> {
  pub fn new(config: &'a Config) -> App<'a> {
    // 打开指定路径的 git 仓库
    let repo = Repository::open(
      &config.project_root
    ).unwrap();

    // 返回新生成的 App
    App {
      config,
      repo,
    }
  }

  // git push origin feat/user-login
  // git branch -D daily/0.1.1
  // git checkout -b daily/0.1.1
  // git push origin daily/0.1.1 -f
  // scp . root@1.1.1.1:/var/www
  // git checkout feat/user-login
  pub fn execute(self) -> Result<String, String> {
    // 获取当前分支名称
    // refs/heads/daily/0.1.1
    let head_name
      = git::get_current_branch_name(&self.repo)
        .unwrap();

    // 如果当前分支 不是 开发分支 报错
    if !head_name
      .as_str()
      .ends_with(&self.config.daily_tag) {
        return Err(
          format!(
            "请先切换到分支：{}",
            &self.config.daily_tag
          )
        );
    }

    git::push_origin(&self.repo).unwrap();
    // 删除 daily 分支
    // git::delete_branch(
    //   &self.repo,
    //   &self.config.daily_tag
    // )?;
    // 生成并切换到新的 daily 分支
    // git::checkout_new_branch(
    //   &self.repo,
    //   &self.config.daily_tag
    // ).unwrap();
    // 推送 daily 分支到远程仓库
    // git::push_origin(&self.repo)?;
    // 执行 第三方命令 如scp
    // cmd::scp()?;
    // 操作完成 切换回 开发分支
    // git::checkout(
    //   &self.repo,
    //   &self.config.dev_tag
    // ).unwrap();

    // 测试切换到 master 分支
    git::checkout(
      &self.repo,
      "master"
    ).unwrap();

    Ok(String::from("Execute complete"))
  }
}