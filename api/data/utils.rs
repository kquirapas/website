use directories::ProjectDirs;
use std::fs;

pub fn get_db_url(env: &str) -> String {
    if let Some(proj_dirs) = ProjectDirs::from("com", "website", &format!("data-${env}")) {
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).expect("Could not create data directory");
        let db_path = data_dir.join("workout.db");
        return format!("sqlite:{}", db_path.display());
    }

    "sqlite:website-data.db".to_string()
}
