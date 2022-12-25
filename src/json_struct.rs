pub mod json_struct {
    use clap::{Arg, ArgMatches, Command};
    use serde;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize, Clone, Debug)]
    #[allow(unused)]
    pub struct CmdParam {
        pub key: String,
        #[serde(default = "empty_string")]
        pub default_value: String,
        #[serde(default = "default_false")]
        pub required: bool,
        #[serde(default = "empty_string")]
        pub help: String,
    }

    fn string_to_static_str(s: &str) -> &'static str {
        Box::leak(s.to_string().into_boxed_str())
    }
    impl CmdParam {
        pub fn get_arg(&self) -> Arg {
            Arg::new(string_to_static_str(self.key.as_str()))
                .help(&self.help)
                .required(self.required)
        }
    }
    #[derive(Deserialize, Clone)]
    pub struct OutputFormat {
        pub show_type: String,
        pub table_show_names: Vec<String>,
        pub table_show_fields: Vec<String>,
    }

    #[derive(Deserialize, Clone)]
    pub struct SetValuMapping {
        pub param_name: String,
        pub cmd_input_name: String,
    }

    #[derive(Deserialize, Clone)]
    pub struct CmdConf<'a> {
        #[serde(default = "empty_string")]
        pub url_path: String,
        pub method: &'a str,
        #[serde(default = "default_false")]
        pub need_body: bool,
        pub params: Option<Vec<CmdParam>>,
        pub output_format: Option<OutputFormat>,
        #[serde(borrow)]
        pub recommend_cmds: Option<Vec<&'a str>>,
        pub set_value_mapping: Option<Vec<SetValuMapping>>,
        pub data_path: Option<String>,
        pub about: Option<String>,
    }

    pub fn default_false() -> bool {
        false
    }
    pub fn empty_string() -> String {
        String::from("")
    }
    #[derive(Deserialize, Clone)]
    pub struct JsonConf<'a> {
        #[serde(borrow)]
        pub command: HashMap<&'a str, HashMap<&'a str, CmdConf<'a>>>,
    }

    impl<'a> CmdConf<'a> {
        pub fn get_args_data(
            &self,
            child_matches: &ArgMatches,
        ) -> Result<HashMap<String, String>, String> {
            let mut map: HashMap<String, String> = HashMap::new();
            let cmd_conf = self;
            let params_option = cmd_conf.params.as_ref();
            if params_option.is_none() {
                return Result::Ok(map);
            }
            let params = params_option.as_ref().unwrap();
            for cp in *params {
                let required = &cp.required;

                let arg_option = child_matches.get_one::<String>(&cp.key);
                if arg_option.is_none() && *required {
                    panic!("{} is required", &cp.key);
                }
                map.insert(cp.key.to_string(), arg_option.as_ref().unwrap().to_string());
            }
            return Ok(map);
        }

        pub fn get_clap_args(&self) -> Vec<Arg> {
            let mut args: Vec<Arg> = Vec::new();
            if self.params.is_none() {
                return args;
            }

            match self.params {
                Some(ref cmd_params) => {
                    for cp in cmd_params {
                        args.push(cp.get_arg());
                    }
                }
                None => unreachable!(),
            }
            return args;
        }
    }

    impl<'de> JsonConf<'de> {
        pub fn read_conf_file() -> JsonConf<'de> {
            let config: JsonConf<'de> =
                serde_json::from_str(CMD_JSON).expect("config file read error");
            return config;
        }
        pub fn get_sub_cmd(&self) -> Vec<Command> {
            let mut cmds: Vec<Command> = Vec::new();
            for (parent, data) in &self.command {
                let mut sub_cmds: Vec<Command> = Vec::new();
                for (sub, ref cmd_conf) in data {
                    sub_cmds.push(
                        Command::new(string_to_static_str(sub)).args(cmd_conf.get_clap_args()),
                    );
                }
                cmds.push(
                    Command::new(string_to_static_str(parent))
                        .about("empty")
                        .arg_required_else_help(true)
                        .subcommands(sub_cmds),
                );
            }
            return cmds;
        }
        pub fn find_cmd(&self, parent: &str, sub: &str) -> Option<&CmdConf> {
            if self.command.contains_key(parent) {
                let sub_map = self.command.get(parent).unwrap();
                let conf = sub_map.get(sub);
                return conf;
            }
            println!("please add config, {}, {}", parent, sub);
            return None;
        }
    }

    const CMD_JSON: &str = r##"{
        "command": {
            "common": {
                "set": {
                    "method": "set",
                    "set_value_mapping": [
                        {
                            "cmd_input_name": "ns",
                            "param_name": "tenant"
                        }
                    ]
                }
            },
            "namespace": {
                "list": {
                    "url_path": "/v1/console/namespaces",
                    "method": "list",
                    "need_body": false,
                    "data_path":"data",
                    "params": [
                        {
                            "key": "accessToken",
                            "default_value": ""
                        },
                        {
                            "key": "username",
                            "default_value": ""
                        }
                    ],
                    "output_format": {
                        "show_type": "table",
                        "table_show_names": [
                            "name",
                            "value"
                        ],
                        "table_show_fields": [
                            "namespaceShowName",
                            "namespace"
                        ]
                    },
                    "recommend_cmds": [
                        "lsdata",
                        "getdata",
                        "use"
                    ]
                }
            },
            "config": {
                "get": {
                    "url_path": "/v1/cs/configs",
                    "method": "get",
                    "need_body": true,
                    "about":"",
                    "params": [
                        {
                            "key": "tenant",
                            "default_value": "",
                            "required": true
                        },
                        {
                            "key": "dataId",
                            "default_value": "*",
                            "required": true
                        },
                        {
                            "key": "group",
                            "default_value": "*",
                            "required": true
                        },
                        {
                            "key": "accessToken",
                            "default_value": ""
                        },
                        {
                            "key": "username",
                            "default_value": ""
                        }
                    ],
                    "output_format": {
                        "show_type": "json",
                        "table_show_names": [
                            "name",
                            "value"
                        ],
                        "table_show_fields": [
                            "namespaceShowName",
                            "namespace"
                        ]
                    },
                    "recommend_cmds": [
                        "lsdata",
                        "getdata",
                        "use"
                    ]
                },
                "update": {
                    "url_path": "/v1/cs/configs",
                    "method": "post",
                    "need_body": true,
                    "params": [
                        {
                            "key": "tenant",
                            "default_value": ""
                        },
                        {
                            "key": "dataId",
                            "default_value": "*"
                        },
                        {
                            "key": "group",
                            "default_value": "*"
                        }
                    ],
                    "output_format": {
                        "show_type": "table",
                        "table_show_names": [
                            "name",
                            "value"
                        ],
                        "table_show_fields": [
                            "namespaceShowName",
                            "namespace"
                        ]
                    },
                    "recommend_cmds": [
                        "lsdata",
                        "getdata",
                        "use"
                    ]
                }
            }
        }
    }"##;
}
