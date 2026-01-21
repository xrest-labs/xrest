use super::endpoint::{Endpoint, EndpointStub, EndpointVersion, RequestConfig};
use super::environment::EnvironmentConfig;
use crate::domains::auth::AuthType;
use crate::io::FileSystem;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub name: String,
    pub environments: Vec<EnvironmentConfig>,
    pub is_authenticated: bool,
    pub auth_type: Option<AuthType>,
    pub endpoints: Vec<Endpoint>,
    pub directory: String,
    pub selected_environment: Option<String>,
    pub git_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceFile {
    pub id: String,
    pub name: String,
    pub is_authenticated: bool,
    pub auth_type: Option<AuthType>,
    pub endpoints: Vec<EndpointStub>,
    pub directory: String,
    pub selected_environment: Option<String>,
    pub git_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStub {
    pub id: String,
    pub name: String,
    pub directory: String,
}

pub struct ServiceDomain<'a> {
    fs: &'a dyn FileSystem,
}

impl<'a> ServiceDomain<'a> {
    pub fn new(fs: &'a dyn FileSystem) -> Self {
        Self { fs }
    }

    pub fn load_service(&self, directory: &str) -> Result<Service, String> {
        let base_path = PathBuf::from(directory);
        let service_path = base_path.join("service.yaml");
        if !self.fs.exists(&service_path) {
            return Err(format!(
                "Service file not found at: {}",
                service_path.display()
            ));
        }
        let service_content = self.fs.read_to_string(&service_path)?;
        let service_file: ServiceFile =
            serde_yaml::from_str(&service_content).map_err(|e| e.to_string())?;

        let mut environments = Vec::new();
        let env_path = base_path.join("environments.yaml");
        if self.fs.exists(&env_path) {
            let env_content = self.fs.read_to_string(&env_path)?;
            environments = serde_yaml::from_str::<Vec<EnvironmentConfig>>(&env_content)
                .map_err(|e| e.to_string())?;
        }

        let mut endpoints = Vec::new();
        let endpoints_dir = base_path.join("endpoints");
        if self.fs.exists(&endpoints_dir) {
            for stub in service_file.endpoints {
                let ep_path = endpoints_dir.join(format!("{}.yaml", stub.id));
                if self.fs.exists(&ep_path) {
                    match self.fs.read_to_string(&ep_path) {
                        Ok(ep_content) => match serde_yaml::from_str::<Endpoint>(&ep_content) {
                            Ok(endpoint) => endpoints.push(endpoint),
                            Err(e) => println!("Failed to parse endpoint {}: {}", stub.id, e),
                        },
                        Err(e) => println!("Failed to read endpoint {}: {}", stub.id, e),
                    }
                }
            }
        }

        Ok(Service {
            id: service_file.id,
            name: service_file.name,
            environments,
            is_authenticated: service_file.is_authenticated,
            auth_type: service_file.auth_type,
            endpoints,
            directory: service_file.directory,
            selected_environment: service_file.selected_environment,
            git_url: service_file.git_url,
        })
    }

    pub fn save_service(&self, service: &mut Service) -> Result<(), String> {
        let dir = PathBuf::from(&service.directory);
        if !self.fs.exists(&dir) {
            self.fs.create_dir_all(&dir)?;
        }

        // Save environments
        let env_path = dir.join("environments.yaml");
        let env_content =
            serde_yaml::to_string(&service.environments).map_err(|e| e.to_string())?;
        self.fs.write(&env_path, &env_content)?;

        // Save endpoints
        let endpoints_dir = dir.join("endpoints");
        if !self.fs.exists(&endpoints_dir) {
            self.fs.create_dir_all(&endpoints_dir)?;
        }

        let mut endpoint_stubs = Vec::new();

        for endpoint in &mut service.endpoints {
            // Versioning logic
            let current_config = RequestConfig {
                method: endpoint.method.clone(),
                url: endpoint.url.clone(),
                authenticated: endpoint.authenticated,
                auth_type: endpoint.auth_type.clone(),
                params: endpoint.params.clone(),
                headers: endpoint.headers.clone(),
                body: endpoint.body.clone(),
                preflight: endpoint.preflight.clone(),
            };

            let should_create_new_version = match endpoint.versions.last() {
                Some(last_v) => last_v.config != current_config,
                None => true,
            };

            if should_create_new_version {
                endpoint.last_version += 1;
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64;

                endpoint.versions.push(EndpointVersion {
                    version: endpoint.last_version,
                    config: current_config,
                    last_updated: now,
                });

                endpoint.metadata.version = endpoint.last_version.to_string();
                endpoint.metadata.last_updated = now;
            }

            let ep_path = endpoints_dir.join(format!("{}.yaml", endpoint.id));
            let ep_content = serde_yaml::to_string(endpoint).map_err(|e| e.to_string())?;
            self.fs.write(&ep_path, &ep_content)?;

            endpoint_stubs.push(EndpointStub {
                id: endpoint.id.clone(),
                name: endpoint.name.clone(),
                url: endpoint.url.clone(),
            });
        }

        // Save service metadata
        let service_file = ServiceFile {
            id: service.id.clone(),
            name: service.name.clone(),
            is_authenticated: service.is_authenticated,
            auth_type: service.auth_type.clone(),
            endpoints: endpoint_stubs,
            directory: service.directory.clone(),
            selected_environment: service.selected_environment.clone(),
            git_url: service.git_url.clone(),
        };

        let path = dir.join("service.yaml");
        let content = serde_yaml::to_string(&service_file).map_err(|e| e.to_string())?;
        self.fs.write(&path, &content)?;

        // Auto-commit if it's a git repo
        if crate::domains::git::is_git_repo(&service.directory) {
            let _ = crate::domains::git::commit_changes(
                &service.directory,
                "Update service configuration",
            );
        }

        Ok(())
    }

    // Collections

    pub fn get_collections_path<R: tauri::Runtime>(
        app: &tauri::AppHandle<R>,
    ) -> Result<PathBuf, String> {
        let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
        Ok(path.join("collections.yaml"))
    }

    pub fn load_collections(&self, path: &PathBuf) -> Result<Vec<Service>, String> {
        if !self.fs.exists(path) {
            return Ok(Vec::new());
        }
        let content = self.fs.read_to_string(path).map_err(|e| e.to_string())?;
        let collections: Vec<Service> =
            serde_yaml::from_str(&content).map_err(|e| e.to_string())?;
        Ok(collections)
    }

    pub fn save_collections(
        &self,
        path: &PathBuf,
        collections: &Vec<Service>,
    ) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !self.fs.exists(parent) {
                self.fs.create_dir_all(parent)?;
            }
        }
        let content = serde_yaml::to_string(collections).map_err(|e| e.to_string())?;
        self.fs.write(path, &content)?;
        Ok(())
    }
}
