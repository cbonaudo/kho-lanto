use once_cell::sync::Lazy;

pub struct Cfg {
    pub token: String,
}

pub static CONFIG: Lazy<Cfg> = Lazy::new(|| {
    let mut config = config::Config::new();
    config
        .merge(config::File::with_name("env.toml"))
        .expect("Couldn't get the env.toml file");

    let token = config
        .get_str("token")
        .expect("Token wasn't provided in env file");

    Cfg { token }
});

pub fn get_header() -> String {
    let token: String = CONFIG.token.to_string();
    format!("Bot {}", token)
}
