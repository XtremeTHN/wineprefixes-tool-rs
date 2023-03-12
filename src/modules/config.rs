// Se define el módulo `config_loader`
mod config_loader {
    use std::collections::HashMap;
    use config::Config;

    pub struct ConfigOptions {
        pub db_url: String,
        pub db_port: u16,
    }

    // Se define la función `load_config` dentro del módulo
    pub fn load_config(config_file: &str) -> Result<ConfigOptions, config::ConfigError> {
        let mut config = Config::default();

        // Se carga el archivo de configuración
        config.merge(config::File::with_name(config_file))?;

        // Se obtienen los valores del archivo de configuración
        let database_config = config.get::<HashMap<String, String>>("database")?;

        // Se parsean los valores obtenidos a la estructura ConfigOptions
        let config_options = ConfigOptions {
            db_url: database_config.get("url").unwrap().to_string(),
            db_port: database_config.get("port").unwrap().parse::<u16>().unwrap(),
        };

        Ok(config_options)
    }
}
