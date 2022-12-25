pub mod print_screen {
    use crate::json_struct::json_struct::*;
    use cli_table::{print_stdout, Cell, Style, Table};
    use colored_json;
    use colored_json::prelude::*;
    use serde_json;
    use serde_json::Value;

    pub fn print(resp: &Value, cmd: &CmdConf) -> Result<String, std::io::Error> {
        let show_type = &cmd.output_format.as_ref().unwrap().show_type;

        if show_type == "table" {
            print_table(resp, cmd)
        } else if show_type == "json" {
            print_json(resp, cmd)
        } else {
            panic!("show_type error");
        }
    }
    pub fn print_table(resp: &Value, cmd: &CmdConf) -> Result<String, std::io::Error> {
        
        let data:&Value = match &cmd.data_path {
            Some(path) => {
                let mut data= resp;
                let more_path:Vec<&str> = path.split("\\.").collect();
                for p in more_path {
                    data = resp.get(p).unwrap();
                }
                data
            }
            None => &resp,
        };

        let table_show_names = &cmd.output_format.as_ref().unwrap().table_show_names;
        let table_show_fields = &cmd.output_format.as_ref().unwrap().table_show_fields;
        let mut title = Vec::new();
        for ele in table_show_names {
            let stt = ele.to_string();
            title.push(stt.cell().bold(true));
        }
        let mut table_data = Vec::new();

        for ele in data.as_array().unwrap() {
            let mut td = Vec::new();
            for key in table_show_fields {
                let v = &ele[key];
                if v.is_string() {
                    td.push(v.as_str().unwrap().cell());
                } else if v.is_boolean() {
                    td.push(v.as_bool().unwrap().cell());
                } else if v.is_u64() {
                    td.push(v.as_u64().unwrap().cell());
                } else if v.is_i64() {
                    td.push(v.as_i64().unwrap().cell());
                } else if v.is_f64() {
                    td.push(v.as_f64().unwrap().cell());
                } else if v.is_null() {
                    td.push("null".cell());
                }
            }
            table_data.push(td);
        }
        let table = table_data.table().title(title).bold(true);
        println!("success {}", print_stdout(table).is_ok());
        Ok("success".to_owned())
    }

    pub fn print_json(resp: &Value, _cmd: &CmdConf) -> Result<String, std::io::Error> {
        let j = serde_json::to_string_pretty(resp)?;
        println!("{}", j.to_colored_json_auto()?);
        Ok(String::from("ok"))
    }
    
}
