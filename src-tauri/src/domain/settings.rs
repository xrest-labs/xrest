use crate::io::FileSystem;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A lightweight representation of a Service used for quick lookups and navigation.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStub {
    /// Unique identifier for the service.
    pub id: String,
    /// Human-readable name of the service.
    pub name: String,
    /// Absolute path to the directory containing the service configuration files.
    pub directory: String,
}

/// Global application settings and service registry.
///
/// This model manages user preferences like themes and maintains a list of
/// imported services and their locations on the filesystem.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Visual theme preference ("light", "dark", or "system").
    pub theme: String,
    /// List of services currently registered in the application.
    pub services: Vec<ServiceStub>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            services: Vec::new(),
        }
    }
}

impl Settings {
    /// Loads settings from a YAML file at the specified path.
    ///
    /// If the file does not exist, it creates a default configuration file
    /// at the given path and returns the default settings.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read, written (when creating defaults),
    /// or if the content is not valid YAML.
    pub fn load(path: &Path, fs: &dyn FileSystem) -> Result<Self, String> {
        if !fs.exists(path) {
            let settings = Self::default();
            settings.save(path, fs)?;
            return Ok(settings);
        }

        let content = fs.read_to_string(path).map_err(|e| e.to_string())?;
        match serde_yaml::from_str::<Settings>(&content) {
            Ok(settings) => Ok(settings),
            Err(e) => {
                println!(
                    "Failed to parse settings.yaml: {}. Falling back to default.",
                    e
                );
                Ok(Self::default())
            }
        }
    }

    /// Saves the current settings to a YAML file at the specified path.
    ///
    /// Ensures that all parent directories exist before writing the file.
    ///
    /// # Errors
    /// Returns an error if the directories cannot be created or the file
    /// cannot be written.
    pub fn save(&self, path: &Path, fs: &dyn FileSystem) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !fs.exists(parent) {
                fs.create_dir_all(parent)?;
            }
        }
        let content = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
        fs.write(path, &content)?;
        Ok(())
    }

    /// Finds and returns the directory path for a service given its unique ID.
    ///
    /// Returns `None` if the service ID is not found in the settings.
    pub fn find_service_directory(&self, service_id: &str) -> Option<String> {
        self.services
            .iter()
            .find(|s| s.id == service_id)
            .map(|s| s.directory.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::MockFileSystem;
    use mockall::predicate::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_default() {
        let mut fs = MockFileSystem::new();
        let path = PathBuf::from("/config/settings.yaml");

        // Mock exists to return false
        fs.expect_exists()
            .with(eq(path.clone()))
            .times(1)
            .returning(|_| false);

        // Mock create_dir_all for parent
        fs.expect_exists()
            .with(eq(PathBuf::from("/config")))
            .times(1)
            .returning(|_| true);

        // Mock write
        fs.expect_write()
            .with(eq(path.clone()), always())
            .times(1)
            .returning(|_, _| Ok(()));

        let settings = Settings::load(&path, &fs).unwrap();
        assert_eq!(settings.theme, "system");
        assert!(settings.services.is_empty());
    }

    #[test]
    fn test_save_and_load() {
        let mut fs = MockFileSystem::new();
        let path = PathBuf::from("/config/settings.yaml");
        let mut settings = Settings::default();
        settings.theme = "dark".to_string();
        settings.services.push(ServiceStub {
            id: "s1".to_string(),
            name: "Service 1".to_string(),
            directory: "/path/to/s1".to_string(),
        });

        // Mock for save
        fs.expect_exists()
            .with(eq(PathBuf::from("/config")))
            .times(1)
            .returning(|_| true);
        fs.expect_write()
            .with(eq(path.clone()), always())
            .times(1)
            .returning(|_, _| Ok(()));

        settings.save(&path, &fs).unwrap();

        // Mock for load
        fs.expect_exists()
            .with(eq(path.clone()))
            .times(1)
            .returning(|_| true);

        let yaml = serde_yaml::to_string(&settings).unwrap();
        fs.expect_read_to_string()
            .with(eq(path.clone()))
            .times(1)
            .returning(move |_| Ok(yaml.clone()));

        let loaded = Settings::load(&path, &fs).unwrap();
        assert_eq!(loaded, settings);
    }

    #[test]
    fn test_find_service_directory() {
        let mut settings = Settings::default();
        settings.services.push(ServiceStub {
            id: "s1".to_string(),
            name: "Service 1".to_string(),
            directory: "/path/to/s1".to_string(),
        });

        assert_eq!(
            settings.find_service_directory("s1"),
            Some("/path/to/s1".to_string())
        );
        assert_eq!(settings.find_service_directory("s2"), None);
    }
}
