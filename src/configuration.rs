#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // init configuration reader
    let mut settings = config::Config::default();
    println!("{:?}",settings);

    /*
    Add configuration values from a file named `configuration`.
    It will look for any top-level file with an extension
    that `config` knows how to parse: yaml, json, etc

    The ? unpacks the Result if Ok but returns the error if not 
    Refer to https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
    */
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values to our Settings type
    settings.try_into()
}