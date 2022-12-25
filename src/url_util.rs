pub mod url_util {

    // use urlencoding::encode;

    use crate::login::login::Login;

    pub fn concat(host: &str, path: &str) -> String {
        let mut url = String::new();
        if host.ends_with("/") && path.starts_with("/") {
            let len = path.len();
            let path_tmp = &path[1..len];
            url.push_str(host);
            url.push_str(path_tmp);
        }

        if !host.ends_with("/") && !path.starts_with("/") {
            url.push_str(host);
            url.push_str("/");
            url.push_str(path);
        }
        url
    }

    pub fn parse_host(login_args: &Login) -> String {
        let host = &login_args.host;

        let mut result_host = String::new();
        let a = "nacos/";
        let http = "http://";

        result_host.push_str(host);
        let port = match login_args.port {
            Some(port) => {
                let mut r = String::from(":");
                r.push_str(port.to_string().as_str());
                r
            }
            None => String::new(),
        };

        result_host.push_str(port.as_str());
        result_host.push_str("/");

        if !result_host.starts_with("http://") || !result_host.starts_with("https://") {
            result_host = http.to_owned() + result_host.as_str();
        }

        if !result_host.ends_with("nacos/") {
            result_host = result_host.to_string() + a;
        }

        if result_host.is_empty() {
            result_host = host.to_string();
        }
        return result_host;
    }
}
