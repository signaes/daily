use serde::Deserialize;
use std::env::var_os;
use std::ffi::OsString;
use std::fs::{self, read_to_string};
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub db: Option<PathBuf>,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingHomeDir,
    FailedCreatingDir(std::io::Error),
    FailedReadingConfig(std::io::Error),
    FailedParsingConfig(serde_json::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingHomeDir => write!(f, "Could not determine home directory."),
            ConfigError::FailedCreatingDir(e) => {
                write!(f, "Failed to create the data directory: {e}")
            }
            ConfigError::FailedReadingConfig(e) => write!(f, "Failed to read the config file: {e}"),
            ConfigError::FailedParsingConfig(e) => {
                write!(f, "Failed to parse the config file: {e}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let data_dir = xdg_data_dir()?;
        fs::create_dir_all(&data_dir).map_err(ConfigError::FailedCreatingDir)?;

        let default_db = data_dir.join("daily.sqlite");
        let default_config = Config {
            db: Some(default_db),
        };

        let config_path = xdg_config_path()?;
        let config_str = match read_to_string(&config_path) {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(default_config),
            Err(e) => return Err(ConfigError::FailedReadingConfig(e)),
        };

        let mut config: Config =
            serde_json::from_str(&config_str).map_err(ConfigError::FailedParsingConfig)?;

        if config.db.is_none() {
            config.db = default_config.db;
        }

        Ok(config)
    }
}

fn resolve_xdg_base<F>(get_env: F, env_var: &str, fallback: &str) -> Result<PathBuf, ConfigError>
where
    F: Fn(&str) -> Option<OsString>,
{
    get_env(env_var)
        .map(PathBuf::from)
        .or_else(|| get_env("HOME").map(|h| PathBuf::from(h).join(fallback)))
        .ok_or(ConfigError::MissingHomeDir)
}

fn xdg_data_dir() -> Result<PathBuf, ConfigError> {
    Ok(resolve_xdg_base(|k| var_os(k), "XDG_DATA_HOME", ".local/share")?.join("daily"))
}

fn xdg_config_path() -> Result<PathBuf, ConfigError> {
    Ok(
        resolve_xdg_base(|k| var_os(k), "XDG_CONFIG_HOME", ".config")?
            .join("daily")
            .join("config.json"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn resolve_xdg_base_uses_env_var_when_available() {
        let result = resolve_xdg_base(
            |k| {
                if k == "XDG_DATA_HOME" {
                    Some(OsString::from("/custom/data/dir"))
                } else {
                    None
                }
            },
            "XDG_DATA_HOME",
            ".local/share",
        );

        assert_eq!(result.unwrap(), PathBuf::from("/custom/data/dir"))
    }

    #[test]
    fn resolve_xdg_base_falls_back_to_home_dir() {
        let result = resolve_xdg_base(
            |k| match k {
                "HOME" => Some(OsString::from("/user/home")),
                _ => None,
            },
            "XDG_DATA_HOME",
            ".local/share",
        );

        assert_eq!(result.unwrap(), PathBuf::from("/user/home/.local/share"));
    }

    #[test]
    fn resolve_xdg_base_gives_precedence_to_xdg_data_home_env_var() {
        let result = resolve_xdg_base(
            |k| match k {
                "XDG_DATA_HOME" => Some(OsString::from("/primary")),
                "HOME" => Some(OsString::from("/secondary")),
                _ => None,
            },
            "XDG_DATA_HOME",
            ".local/share",
        );

        assert_eq!(result.unwrap(), PathBuf::from("/primary"));
    }

    #[test]
    fn resolve_xdg_base_errors_when_get_env_returns_none() {
        let result = resolve_xdg_base(|_k| None, "XDG_DATA_HOME", ".local/share");

        assert!(matches!(result, Err(ConfigError::MissingHomeDir)));
    }

    #[test]
    fn smoke_test_xdg_data_dir_succeeds() {
        let _ = xdg_data_dir();
    }

    #[test]
    fn smoke_test_xdg_config_path_succeeds() {
        let _ = xdg_config_path();
    }
}
