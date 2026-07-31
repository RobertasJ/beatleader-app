use std::sync::LazyLock;

use directories::ProjectDirs;

pub static PROJ_DIRS: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from("com", "beatleader", "beatleader-app").unwrap());
