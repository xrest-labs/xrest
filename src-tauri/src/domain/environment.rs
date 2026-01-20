use crate::io::FileSystem;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents a simple key-value pair used for variables, headers, or parameters.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

pub type Variable = NameValue;

/// Domain model for an Environment configuration.
///
/// Environments allow users to switch between different sets of variables
/// (e.g., Development vs. Production) without changing the core request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    /// The name of the environment (e.g., "Development", "Staging", "Production").
    pub name: String,
    /// Whether this environment is considered "unsafe" (e.g., Production).
    /// Significant for UI warnings and safety checks.
    pub is_unsafe: bool,
    /// A collection of variables associated with this environment.
    pub variables: Vec<Variable>,
}

impl Environment {
    /// Checks if the environment is marked as unsafe (e.g., Production).
    pub fn is_unsafe(&self) -> bool {
        self.is_unsafe
    }

    /// Checks if a collection of environments is non-empty.
    pub fn has_any(environments: &[Environment]) -> bool {
        !environments.is_empty()
    }

    /// Compares the current environment values to another environment.
    /// Returns true if all fields are identical.
    pub fn is_same_as(&self, other: &Environment) -> bool {
        self == other
    }
}

/// A collection of environments for a specific service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ServiceEnvironment {
    pub environments: Vec<Environment>,
}

impl ServiceEnvironment {
    /// Loads service environments from a YAML file.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or if the content is not valid YAML.
    pub fn load(path: &Path, fs: &dyn FileSystem) -> Result<Self, String> {
        if !fs.exists(path) {
            return Ok(Self::default());
        }

        let content = fs.read_to_string(path).map_err(|e| e.to_string())?;
        let environments: Vec<Environment> = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse environments: {}", e))?;

        Ok(ServiceEnvironment { environments })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unsafe() {
        let env = Environment {
            name: "Production".to_string(),
            is_unsafe: true,
            variables: vec![],
        };
        assert!(env.is_unsafe());

        let env_safe = Environment {
            name: "Development".to_string(),
            is_unsafe: false,
            variables: vec![],
        };
        assert!(!env_safe.is_unsafe());
    }

    #[test]
    fn test_has_any() {
        let envs = vec![Environment::default()];
        assert!(Environment::has_any(&envs));

        let empty_envs: Vec<Environment> = vec![];
        assert!(!Environment::has_any(&empty_envs));
    }

    #[test]
    fn test_is_same_as() {
        let env1 = Environment {
            name: "Dev".to_string(),
            is_unsafe: false,
            variables: vec![Variable {
                name: "api_url".to_string(),
                value: "http://localhost:3000".to_string(),
            }],
        };

        let env2 = env1.clone();
        let env3 = Environment {
            name: "Prod".to_string(),
            is_unsafe: true,
            variables: vec![],
        };

        assert!(env1.is_same_as(&env2));
        assert!(!env1.is_same_as(&env3));
    }

    #[test]
    fn test_service_environment_load() {
        use crate::io::MockFileSystem;
        use mockall::predicate::*;
        use std::path::PathBuf;

        let mut fs = MockFileSystem::new();
        let path = PathBuf::from("/path/to/environments.yaml");

        // Test File Missing
        fs.expect_exists()
            .with(eq(path.clone()))
            .times(1)
            .returning(|_| false);

        let result = ServiceEnvironment::load(&path, &fs).unwrap();
        assert!(result.environments.is_empty());

        // Test Success Load
        let mut fs = MockFileSystem::new();
        fs.expect_exists()
            .with(eq(path.clone()))
            .times(1)
            .returning(|_| true);

        let envs = vec![Environment {
            name: "Dev".to_string(),
            is_unsafe: false,
            variables: vec![Variable {
                name: "v1".to_string(),
                value: "val1".to_string(),
            }],
        }];
        let yaml = serde_yaml::to_string(&envs).unwrap();

        fs.expect_read_to_string()
            .with(eq(path.clone()))
            .times(1)
            .returning(move |_| Ok(yaml.clone()));

        let result = ServiceEnvironment::load(&path, &fs).unwrap();
        assert_eq!(result.environments.len(), 1);
        assert_eq!(result.environments[0].name, "Dev");
    }
}
