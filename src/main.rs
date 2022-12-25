use json_struct::json_struct::*;
use login::login::*;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;
mod json_struct;
use clap::ArgMatches;
mod clap_cli;
mod login;
use clap::Parser;
mod print_screen;
mod url_util;
use print_screen::print_screen::print;
mod input_helper;
use input_helper::input_helper as ih;
mod req;
use req::req::*;

const NAMESPACE_KEY: &str = "namespace";
fn get_config_common_args<'a>(
    child_matches: &'a &ArgMatches,
) -> (&'a String, &'a String, &'a String) {
    let data_id = child_matches.get_one::<String>("DataId").expect("required");
    let grp_id = child_matches.get_one::<String>("Group").expect("required");
    let namespace = child_matches
        .get_one::<String>("Namespace")
        .expect("required");
    // println!("{}, {}, {}", data_id, grp_id, namespace);
    (data_id, grp_id, namespace)
}

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut login_args = Login::parse();

    let host_parse = url_util::url_util::parse_host(&login_args);

    login_args.host = host_parse.to_string();
    let login_result = login(&login_args.host, &login_args).await;

    let token = match login_result {
        Ok(resp) => {
            println!("success login {}", &login_args.host);
            resp
        }
        Err(resp) => {
            println!("{:#?}", resp);
            println!("{:#?}", login_args);
            panic!("cannot login");
        }
    };

    let mut context: HashMap<&str, &String> = HashMap::new();

    context.insert("host", &host_parse);
    context.insert("username", &login_args.user);
    context.insert("accessToken", &token);

    let config: JsonConf = JsonConf::read_conf_file();
    // let aa = clap_cli::cli(&config);
    let mut editor = ih::get_enditor();

    loop {
        let line = match ih::readline(&mut editor) {
            Ok(line) => line,
            Err(_) => {
                ih::append_history(&mut editor);
                panic!();
            }
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // let nn = String::from("nacos-cli ") + line;
        match process(line, &config, &context).await {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{}", err).map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

async fn process<'a>(
    line: &str,
    config: &'a JsonConf<'a>,
    context: &HashMap<&str, &String>,
) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let clap_cmd = clap_cli::cli(config);
    let matches = clap_cmd
        .try_get_matches_from(&args)
        .map_err(|e| e.to_string())?;

    match matches.subcommand() {
        Some(("config", sub_matches)) => match &sub_matches.subcommand() {
            Some(("list", _child_matches)) => {
                println!("This cmd will be support in the feture!");
            }
            Some(("get", child_matches)) => {
                let cmd_conf = config.find_cmd("config", "get");
                if let None = cmd_conf {
                    assert!(false)
                }
                let (data_id, grp_id, namespace) = get_config_common_args(child_matches);
                let p = vec![
                    ("dataId", data_id.as_str()),
                    ("group", grp_id.as_str()),
                    ("tenant", namespace.as_str()),
                ];
                match get(&context, &cmd_conf.unwrap(), &p).await {
                    Ok(resp) => {
                        // println!("{:#?}", resp);
                        if let Err(_) = print(&resp, &cmd_conf.unwrap()) {
                            return Err(String::from("show output data error"));
                        }
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                    }
                }
            }
            Some(("update", child_matches)) => {
                let (data_id, grp_id, namespace) = get_config_common_args(child_matches);
                println!("{} {} {}", data_id, grp_id, namespace);
            }
            _ => unreachable!(),
        },
        Some((NAMESPACE_KEY, sub_matches)) => match &sub_matches.subcommand() {
            Some(("list", _child_matches)) => {
                let cmd_conf = config.find_cmd(NAMESPACE_KEY, "list");
                if let None = cmd_conf {
                    assert!(false)
                }
                let p = vec![];
                match get(&context, &cmd_conf.unwrap(), &p).await {
                    Ok(resp) => {
                        if let Err(_) = print(&resp, &cmd_conf.unwrap()) {
                            return Err(String::from("show output data error"));
                        }
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                    }
                }
            }
            Some(("create", child_matches)) => {
                let cmd_conf = config.find_cmd(NAMESPACE_KEY, "create");
                if let None = cmd_conf {
                    assert!(false)
                }
                let (data_id, grp_id, namespace) = get_config_common_args(child_matches);
                let p = vec![
                    ("dataId", data_id.as_str()),
                    ("group", grp_id.as_str()),
                    ("tenant", namespace.as_str()),
                ];
                match get(&context, &cmd_conf.unwrap(), &p).await {
                    Ok(resp) => {
                        if let Err(_) = print(&resp, &cmd_conf.unwrap()) {
                            return Err(String::from("show output data error"));
                        }
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                    }
                }
            }
            Some(("update", child_matches)) => {
                let (data_id, grp_id, namespace) = get_config_common_args(child_matches);
                println!("{} {} {}", data_id, grp_id, namespace);
            }
            Some(("del", child_matches)) => {
                let (data_id, grp_id, namespace) = get_config_common_args(child_matches);
                println!("{} {} {}", data_id, grp_id, namespace);
            }
            _ => unreachable!(),
        },
        Some(("diff", sub_matches)) => {
            let color = sub_matches
                .get_one::<String>("color")
                .map(|s| s.as_str())
                .expect("defaulted in clap");

            let mut base = sub_matches.get_one::<String>("base").map(|s| s.as_str());
            let mut head = sub_matches.get_one::<String>("head").map(|s| s.as_str());
            let mut path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base.unwrap_or("stage");
            let head = head.unwrap_or("worktree");
            let path = path.unwrap_or("");
            println!("Diffing {}..{} {} (color={})", base, head, path, color);
        }
        Some(("push", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Adding {:?}", paths);
        }
        Some(("stash", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
            match stash_command {
                ("apply", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Applying {:?}", stash);
                }
                ("pop", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Popping {:?}", stash);
                }
                ("push", sub_matches) => {
                    let message = sub_matches.get_one::<String>("message");
                    println!("Pushing {:?}", message);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(false)
    // Continued program logic goes here...
}
