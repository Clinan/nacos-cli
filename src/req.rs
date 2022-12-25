pub mod req {
    use crate::json_struct::json_struct::*;
    use crate::url_util;
    use reqwest;
    use serde_json;
    use serde_json::Value;
    use std::collections::HashMap;
    use urlencoding::encode;

    pub fn build_request_url(
        context: &HashMap<&str, &String>,
        cmd_conf: &CmdConf,
        p: &Vec<(&str, &str)>,
    ) -> String {
        let host = context.get("host").unwrap();
        let url_path = &cmd_conf.url_path.as_str();
        let mut url = url_util::url_util::concat(&host, &url_path);
        let mut kv = String::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        if let Some(cmd_params) = &cmd_conf.params {
            for ele in cmd_params {
                let key = &ele.key;
                let value = &ele.default_value;
                if !value.is_empty() {
                    params.insert(key, &value);
                }
                
            }
        }

        for (key, value) in p {
            params.insert(key, value);
        }
        for (key, value) in params {
            let val_endcode = encode(value);
            kv = kv + "&" + key + "=" + &val_endcode.to_string();
        }

        url = url + "?" + &kv[1..];

        return url;
    }

    pub async fn get<'a>(
        context: &HashMap<&str, &String>,
        cmd_conf: &'a CmdConf<'a>,
        p: &Vec<(&str, &str)>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = build_request_url(context, cmd_conf, p);
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    pub async fn post<'a>(
        context: &HashMap<&str, &String>,
        cmd_conf: &'a CmdConf<'a>,
        p: &Vec<(&str, &str)>,
        body: &Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = build_request_url(context, cmd_conf, p);
        let mut params: Vec<(&str, &str)> = Vec::new();

        if body.is_object() {
            let input_params = body.as_object().unwrap();
            for ele in input_params.iter().enumerate() {
                let (_index, (k, v)) = ele;
                params.push((k, v.as_str().unwrap()));
            }
        }

        let client = reqwest::Client::new();

        let resp = client.post(url).form(&params).send().await?;

        let resp_json = resp.json().await?;

        Ok(resp_json)
    }
}
