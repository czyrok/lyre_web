use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct EnvironmentContext {
    pub project_data_dir_path: String,
    pub content_totp_uri: String,

    // This variable is used by SQLx (for macros)
    // And we can't change the name
    // So we rename it to `local_database_uri` after deserialization
    database_url: String,
    #[serde(skip_deserializing, default)]
    pub local_database_uri: String,
}

impl EnvironmentContext {
    pub fn load_environment() -> Result<Self, envy::Error> {
        // Loads the environment variables from the dotenv files
        dotenvy::from_path_override(Path::new(".env"))
            .expect("A `.env` should exist");
        dotenvy::from_path_override(Path::new(".env.development"))
            .expect("A `.env.development` should exist");

        let mut environment = envy::from_env::<EnvironmentContext>()?;

        environment.local_database_uri = environment.database_url.clone();

        Ok(environment)
    }
}
