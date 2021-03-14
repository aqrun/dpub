use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use toml;

/// 项目配置信息
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
  /// 项目名称 和 daily.toml 键对应
  pub project_name: String,
  /// 开发分支
  pub dev_tag: String,
  /// 日常分支
  pub daily_tag: String,
  /// 项目路径
  pub project_root: String,
}

impl Config {

  /// 从指定路径的文件读取配置信息
  pub fn from_dist<P: AsRef<Path>> (
    // 配置文件绝对路径
    config_file: P,
    // 项目名称
    project_name: &str
  ) -> Result<Config, ()> {
    // 读取配置文件内容
    let c = std::fs::read_to_string(
      &config_file
    ).unwrap();

    // 将内容转为 指定类型的
    // HashMap { string: ProjectConfig }
    let config: HashMap<String, ProjectConfig> =
        toml::from_str(&c).unwrap();

    // 获取 hashMap中键为 cloud-monitor 
    // 的 DefConfig 配置项
    let project_config = 
        config.get(project_name).unwrap();
  
    // 待优化
    let pc = project_config.clone();

    Ok(Config {
      project_name: String::from(project_name),
      dev_tag: pc.dev_tag.unwrap(),
      daily_tag: pc.daily_tag.unwrap(),
      project_root: pc.project_root.unwrap(),
    })
  }

}

/// 项目配置参数 和 daily.toml 对应
#[derive(
  Debug, Clone, PartialEq,
  Serialize, Deserialize
)]
pub struct ProjectConfig {
  /// 开发分支
  pub dev_tag: Option<String>,
  /// 日常分支
  pub daily_tag: Option<String>,
  /// 项目路径
  pub project_root: Option<String>,
}

impl Default for ProjectConfig {
  fn default() -> ProjectConfig {
    ProjectConfig {
      dev_tag: None,
      daily_tag: None,
      project_root: None,
    }
  }
}
