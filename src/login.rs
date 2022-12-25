pub mod login {

    use clap::Parser;
    // use clap::Arg;
    use serde_json;
    use std::string::String;

    #[derive(Parser, Debug)] // requires `derive` feature
    #[command(author, version, about, long_about = None)]
    pub struct Login {
        #[arg(help = "For example: 127.0.0.1 or nacos.nacos.com")]
        pub host: String,

        #[arg(short = 'P', value_name = "port")]
        pub port: Option<u32>,

        #[arg(short = 'u', value_name = "user", default_value = "nacos")]
        pub user: String,

        #[arg(short = 'p', value_name = "password", default_value = "nacos")]
        pub password: String,
    }

    #[derive(Debug)] // requires `derive` feature
    pub struct UserInfo {
        pub host: String,

        pub user: String,

        pub token: String,
    }

    pub async fn login(
        host: &str,
        login_args: &Login,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let params = [
            ("username", &login_args.user),
            ("password", &login_args.password),
        ];
        let url = host.to_owned() + "v1/auth/users/login";
        // println!("{}", url);
        let resp = client.post(url).form(&params).send().await?;

        let text = &resp.text().await?;
        let resp_json: serde_json::Value = serde_json::from_str(text.as_str())?;

        let token = resp_json["accessToken"].as_str().unwrap();
        println!("accessToken {}", token);
        Ok(token.to_string())
    }
}
