use std::fs::File;
use std::io::Read;
use toml;
use mysql::Opts;
use mysql::OptsBuilder;


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server : Vec<ConfigServer>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigServer {
    pub ip : Option<String>,
    pub port : Option<u32>,
    pub user : Option<String>,
    pub pw : Option<String>
}

impl ConfigServer {
    pub fn get_name(&self) -> String {
        format!(
            "{}:{}",
            self.ip.clone().expect("server has no ip"),
            self.port.clone().unwrap_or(3306).to_string() 
        )
    }
}

impl Into<Opts> for ConfigServer {
    fn into(self) -> Opts {
        let mut builder = OptsBuilder::new();
        builder.ip_or_hostname(self.ip);
        builder.tcp_port(self.port.expect("port not given") as u16);
        builder.user(self.user);
        builder.pass(self.pw);
        builder.prefer_socket(false);
        builder.into()
    }
}

impl Config {
    pub fn from_file(filename : &str) -> Result<Config, String> {
        let mut buf = String::new();
        let mut file = File::open(filename).expect("could not open config.toml");

        file.read_to_string(&mut buf);

        let decoded: Config = toml::from_str(&buf).expect("could not parse toml config file");
        
        Ok(decoded)
    }
}