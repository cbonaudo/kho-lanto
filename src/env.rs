pub fn get_data() -> String {
    let mut config = config::Config::new();
    config
        .merge(config::File::with_name("env.toml"))
        .expect("Couldn't get the env.toml file");

    let token = config
        .get_str("token")
        .expect("Token wasn't provided in env file");

    token
}
