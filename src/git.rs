use git2::{Repository, Branch, BranchType};
use git2::build::CheckoutBuilder;

/// 获取当前分支名称
pub fn get_current_branch_name<'a> (
  repo: &'a Repository
) -> Result<String, ()> {
  let head = repo.head().unwrap();
  let name = head.name().unwrap();
  Ok(String::from(name))
}

/// 根据当前分支 生成新的指定分支
/// 等同 `git checkout -b <branch_name>`
pub fn checkout_new_branch<'a> (
  repo: &'a Repository,
  branch_name: &'a str
) -> Result<Branch<'a>, ()> {
  // 获取 当前分支 reference
  let head = repo.head().unwrap();
  // 获取 当前分支 commit信息
  let head_commit = head.peel_to_commit()
    .unwrap();

  // 生成新的分支
  let _new_branch = repo.branch(
      branch_name,
      &head_commit,
      false
  ).unwrap();

   // 切换到新生成的分支
   checkout(repo, branch_name)
}

/// 切换分支
/// 等同 `git checkout <branch_name>`
pub fn checkout<'a> (
  // 仓库
  repo: &'a Repository,
  // 分支名称
  branch_name: &'a str
) -> Result<Branch<'a>, ()> {
  // 获取目标分支
  let target_branch = repo.find_branch(
      branch_name,
      BranchType::Local
  ).unwrap();

  let mut builder = CheckoutBuilder::new();
  // 使用目标分支数据更新当前目录
  repo.checkout_tree(
      target_branch.get()
        .peel_to_tree()
        .unwrap()
        .as_object(),
      Some(builder.safe())
  ).unwrap();
  // 改变 head指针 到目标分支
  repo.set_head(
    target_branch
      .get()
      .name()
      .unwrap()
    ).unwrap();
  // 返回目标分支 可以后续继续操作
  Ok(target_branch)
}

/// 删除指定分支
pub fn delete_branch<'a>(
  repo: &'a Repository,
  branch_name: &'a str
) -> Result<String, ()> {
  println!("Delete: {}", branch_name);
  Ok(String::from("Deleted branch"))
}

/// 推送当前分支到远程仓库
pub fn push_origin<'a>(
  repo: &'a Repository
) -> Result<String, String> {
  let name = get_current_branch_name(repo)
    .unwrap();
  println!("Push success: {}", name);
  Ok(String::from("Push origin success"))
}


