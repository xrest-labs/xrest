use crate::io::FileSystem;
use crate::types::{GitStatus, UserSettings};
use git2::{IndexAddOption, PushOptions, Repository, Signature};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, Runtime};

// User Settings (settings.yaml) - This is our main configuration file now
pub fn get_settings_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(path.join("settings.yaml"))
}

pub fn load_settings(path: &PathBuf, fs: &dyn FileSystem) -> Result<UserSettings, String> {
    if !fs.exists(path) {
        let settings = UserSettings::default();
        save_settings(path, &settings, fs)?;
        return Ok(settings);
    }
    let content = fs.read_to_string(path).map_err(|e| e.to_string())?;
    match serde_yaml::from_str::<UserSettings>(&content) {
        Ok(settings) => Ok(settings),
        Err(e) => {
            println!(
                "Failed to parse settings.yaml: {}. Falling back to default.",
                e
            );
            Ok(UserSettings::default())
        }
    }
}

pub fn save_settings(
    path: &PathBuf,
    settings: &UserSettings,
    fs: &dyn FileSystem,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !fs.exists(parent) {
            fs.create_dir_all(parent)?;
        }
    }
    let content = serde_yaml::to_string(settings).map_err(|e| e.to_string())?;
    fs.write(path, &content)?;
    Ok(())
}

// Collections (collections.yaml)
pub fn get_collections_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(path.join("collections.yaml"))
}

pub fn load_collections(
    path: &PathBuf,
    fs: &dyn FileSystem,
) -> Result<Vec<crate::types::Service>, String> {
    if !fs.exists(path) {
        return Ok(Vec::new());
    }
    let content = fs.read_to_string(path).map_err(|e| e.to_string())?;
    let collections: Vec<crate::types::Service> =
        serde_yaml::from_str(&content).map_err(|e| e.to_string())?;
    Ok(collections)
}

pub fn save_collections(
    path: &PathBuf,
    collections: &Vec<crate::types::Service>,
    fs: &dyn FileSystem,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !fs.exists(parent) {
            fs.create_dir_all(parent)?;
        }
    }
    let content = serde_yaml::to_string(collections).map_err(|e| e.to_string())?;
    fs.write(path, &content)?;
    Ok(())
}

// Tab State (tabstate.yaml)
pub fn get_tab_state_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(path.join("tabstate.yaml"))
}

pub fn get_token_cache_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let path = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    Ok(path.join("token_cache.yaml"))
}

pub fn load_tab_state(
    path: &PathBuf,
    fs: &dyn FileSystem,
) -> Result<Option<crate::types::TabState>, String> {
    if !fs.exists(path) {
        return Ok(None);
    }
    let content = fs.read_to_string(path).map_err(|e| e.to_string())?;
    let state: crate::types::TabState =
        serde_yaml::from_str(&content).map_err(|e| e.to_string())?;
    Ok(Some(state))
}

pub fn save_tab_state(
    path: &PathBuf,
    state: &crate::types::TabState,
    fs: &dyn FileSystem,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !fs.exists(parent) {
            fs.create_dir_all(parent)?;
        }
    }
    let content = serde_yaml::to_string(state).map_err(|e| e.to_string())?;
    fs.write(path, &content)?;
    Ok(())
}

pub fn load_service(directory: &str, fs: &dyn FileSystem) -> Result<crate::types::Service, String> {
    let base_path = PathBuf::from(directory);
    let service_path = base_path.join("service.yaml");
    if !fs.exists(&service_path) {
        return Err(format!(
            "Service file not found at: {}",
            service_path.display()
        ));
    }
    let service_content = fs.read_to_string(&service_path)?;
    let service_file: crate::types::ServiceFile =
        serde_yaml::from_str(&service_content).map_err(|e| e.to_string())?;

    let mut environments = Vec::new();
    let env_path = base_path.join("environments.yaml");
    if fs.exists(&env_path) {
        let env_content = fs.read_to_string(&env_path)?;
        environments = serde_yaml::from_str::<Vec<crate::types::EnvironmentConfig>>(&env_content)
            .map_err(|e| e.to_string())?;
    }

    let mut endpoints = Vec::new();
    let endpoints_dir = base_path.join("endpoints");
    if fs.exists(&endpoints_dir) {
        for stub in service_file.endpoints {
            let ep_path = endpoints_dir.join(format!("{}.yaml", stub.id));
            if fs.exists(&ep_path) {
                match fs.read_to_string(&ep_path) {
                    Ok(ep_content) => {
                        match serde_yaml::from_str::<crate::types::Endpoint>(&ep_content) {
                            Ok(endpoint) => endpoints.push(endpoint),
                            Err(e) => println!("Failed to parse endpoint {}: {}", stub.id, e),
                        }
                    }
                    Err(e) => println!("Failed to read endpoint {}: {}", stub.id, e),
                }
            }
        }
    }

    Ok(crate::types::Service {
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

pub fn save_service(
    service: &mut crate::types::Service,
    fs: &dyn FileSystem,
) -> Result<(), String> {
    let dir = PathBuf::from(&service.directory);
    if !fs.exists(&dir) {
        fs.create_dir_all(&dir)?;
    }

    // Save environments
    let env_path = dir.join("environments.yaml");
    let env_content = serde_yaml::to_string(&service.environments).map_err(|e| e.to_string())?;
    fs.write(&env_path, &env_content)?;

    // Save endpoints
    let endpoints_dir = dir.join("endpoints");
    if !fs.exists(&endpoints_dir) {
        fs.create_dir_all(&endpoints_dir)?;
    }

    let mut endpoint_stubs = Vec::new();

    for endpoint in &mut service.endpoints {
        // Versioning logic
        let current_config = crate::types::RequestConfig {
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

            endpoint.versions.push(crate::types::EndpointVersion {
                version: endpoint.last_version,
                config: current_config,
                last_updated: now,
            });

            endpoint.metadata.version = endpoint.last_version.to_string();
            endpoint.metadata.last_updated = now;
        }

        let ep_path = endpoints_dir.join(format!("{}.yaml", endpoint.id));
        let ep_content = serde_yaml::to_string(endpoint).map_err(|e| e.to_string())?;
        fs.write(&ep_path, &ep_content)?;

        endpoint_stubs.push(crate::types::EndpointStub {
            id: endpoint.id.clone(),
            name: endpoint.name.clone(),
            url: endpoint.url.clone(),
        });
    }

    // Save service metadata
    let service_file = crate::types::ServiceFile {
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
    fs.write(&path, &content)?;

    // Auto-commit if it's a git repo
    if is_git_repo(&service.directory) {
        let _ = commit_changes(&service.directory, "Update service configuration");
    }

    Ok(())
}

pub fn is_git_repo(directory: &str) -> bool {
    Repository::open(directory).is_ok()
}

pub fn init_git(directory: &str, remote_url: Option<String>) -> Result<(), String> {
    let repo = Repository::init(directory).map_err(|e| e.to_string())?;
    if let Some(url) = remote_url {
        repo.remote("origin", &url).map_err(|e| e.to_string())?;
    }
    commit_changes(directory, "Initial commit from xrest")?;
    Ok(())
}

pub fn get_git_status(directory: &str) -> Result<GitStatus, String> {
    let repo = Repository::open(directory).map_err(|e| e.to_string())?;

    let is_git = true;
    let remote_url = repo
        .find_remote("origin")
        .ok()
        .and_then(|r| r.url().map(|u| u.to_string()));

    let head = repo.head().ok();
    let branch = head
        .as_ref()
        .and_then(|h| h.shorthand().map(|s| s.to_string()));

    let mut status_options = git2::StatusOptions::new();
    status_options.include_untracked(true);
    let statuses = repo
        .statuses(Some(&mut status_options))
        .map_err(|e| e.to_string())?;
    let has_uncommitted_changes = !statuses.is_empty();

    // Check for unpushed commits
    let mut has_unpushed_commits = false;
    if let (Ok(local), Ok(remote)) = (
        repo.revparse_single("HEAD"),
        repo.revparse_single("origin/main"),
    ) {
        let local_id = local.id();
        let remote_id = remote.id();
        if local_id != remote_id {
            // This is a bit simplified, but checks if local HEAD is ahead
            let (ahead, _) = repo
                .graph_ahead_behind(local_id, remote_id)
                .unwrap_or((0, 0));
            has_unpushed_commits = ahead > 0;
        }
    } else if repo.revparse_single("HEAD").is_ok() && repo.find_remote("origin").is_ok() {
        // If we have HEAD but no origin/main, we likely have unpushed commits
        has_unpushed_commits = true;
    }

    let last_sync = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(GitStatus {
        is_git,
        remote_url,
        branch,
        has_uncommitted_changes,
        has_unpushed_commits,
        last_sync: Some(last_sync),
    })
}

pub fn commit_changes(directory: &str, message: &str) -> Result<(), String> {
    let repo = Repository::open(directory).map_err(|e| e.to_string())?;
    let mut index = repo.index().map_err(|e| e.to_string())?;

    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| e.to_string())?;
    index.write().map_err(|e| e.to_string())?;

    let tree_id = index.write_tree().map_err(|e| e.to_string())?;
    let tree = repo.find_tree(tree_id).map_err(|e| e.to_string())?;

    let signature = Signature::now("xrest App", "info@xrest.io").map_err(|e| e.to_string())?;

    let parent_commit = match repo.head() {
        Ok(head) => Some(head.peel_to_commit().map_err(|e| e.to_string())?),
        Err(_) => None,
    };

    let parents = match &parent_commit {
        Some(c) => vec![c],
        None => vec![],
    };

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parents,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn sync_git(directory: &str) -> Result<(), String> {
    let repo = Repository::open(directory).map_err(|e| e.to_string())?;

    // Commit any local changes first
    let _ = commit_changes(directory, "Sync point: auto-committing local changes");

    // Pull (Fetch + Merge)
    let mut remote = repo.find_remote("origin").map_err(|e| e.to_string())?;
    let mut fetch_options = git2::FetchOptions::new();

    // In a real app we'd handle auth here, but for now we'll assume SSH/stored creds
    remote
        .fetch(&["main"], Some(&mut fetch_options), None)
        .map_err(|e| e.to_string())?;

    // Simplified merge
    let fetch_head = repo
        .find_reference("FETCH_HEAD")
        .map_err(|e| e.to_string())?;
    let fetch_commit = fetch_head.peel_to_commit().map_err(|e| e.to_string())?;
    let annotated_fetch_commit = repo
        .find_annotated_commit(fetch_commit.id())
        .map_err(|e| e.to_string())?;

    let analysis = repo
        .merge_analysis(&[&annotated_fetch_commit])
        .map_err(|e| e.to_string())?;

    if analysis.0.is_fast_forward() {
        let mut head = repo.head().map_err(|e| e.to_string())?;
        head.set_target(fetch_commit.id(), "fast-forward")
            .map_err(|e| e.to_string())?;
        repo.checkout_head(None).map_err(|e| e.to_string())?;
    } else if analysis.0.is_normal() {
        // Normal merge might have conflicts
        repo.merge(&[&annotated_fetch_commit], None, None)
            .map_err(|e| e.to_string())?;

        // After merge, if there are no conflicts, we should commit
        let index = repo.index().map_err(|e| e.to_string())?;
        if index.has_conflicts() {
            return Err("Merge conflict detected. Please resolve manually.".to_string());
        }

        // Create merge commit
        let msg = format!("Merge branch 'main' of remote");
        commit_changes(directory, &msg)?;
    }

    // Push
    let mut push_options = PushOptions::new();
    remote
        .push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}
