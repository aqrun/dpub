#[macro_use]
extern crate clap;
extern crate dpub;

use clap::{App, Arg};
use dpub::Config;

const VERSION: &str = concat!("v", crate_version!());

fn main() {
  let matches = App::new(crate_name!())
    .about(crate_description!())
    .author("Aqrun <aqrun@sina.com>")
    .version(VERSION)
    .arg(
      Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("配置文件")
        .help("指定配置文件")
    ).arg(
      Arg::with_name("project")
        .help("指定要发布的项目名称")
        .value_name("项目名称")
        .required(true)
        .index(1)
    )
    .get_matches();
  
  // 获取参数指定的配置文件或默认使用当前路径
  let config_file = matches.value_of("config")
    .unwrap_or("daily.toml");
  // 获取参数指定的项目名称
  let project_name = matches.value_of("project").unwrap();

  // 生成配置参数
  let config = Config::from_dist(config_file, &project_name)
    .unwrap();

  println!("参数：{:?}", config);
}
