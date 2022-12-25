use clap::{arg, Command};

use crate::json_struct::json_struct::*;

pub fn cli(json_conf: &JsonConf) -> Command {
    return Command::new("nacos-cli")
        .multicall(true)
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands(json_conf.get_sub_cmd());
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}
