pub mod operation;

use std::{collections::HashSet, fs};

use serde::{Deserialize, Serialize};

use crate::defs;

#[derive(Serialize, Deserialize)]
struct Data {
    common: Common,
    sus_path: SusPath,
    sus_map: HashSet<String>,
    kstat: SusKstat,
}

#[derive(Serialize, Deserialize)]
struct Common {
    version: String,
    release: String,
    avc_spoofing: bool,
}

#[derive(Serialize, Deserialize)]
struct SusPath {
    sus_path_loop: HashSet<String>,
    sus_path: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
struct SusKstat {
    sus_kstat: HashSet<String>,
    update_kstat: HashSet<String>,
    full_clone: HashSet<String>,
    statically: HashSet<SusKstatStatically>,
}

#[derive(Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
struct SusKstatStatically {
    path: String,
    ino: String,
    dev: String,
    nlink: String,
    size: String,
    atime: String,
    atime_nsec: String,
    mtime: String,
    mtime_nsec: String,
    ctime: String,
    ctime_nsec: String,
    blocks: String,
    blksize: String,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            common: Common {
                version: "default".to_string(),
                release: "default".to_string(),
                avc_spoofing: false,
            },
            sus_path: SusPath {
                sus_path_loop: HashSet::new(),
                sus_path: HashSet::new(),
            },
            sus_map: HashSet::new(),
            kstat: SusKstat {
                sus_kstat: HashSet::new(),
                update_kstat: HashSet::new(),
                full_clone: HashSet::new(),
                statically: HashSet::new(),
            },
        }
    }
}

fn save_config(config: Data) {
    let Ok(string) = serde_json::to_string_pretty(&config) else {
        log::warn!("failed to deserialize susfs string");
        return;
    };
    if let Err(e) = fs::write(defs::SUSFS_CONFUG, string) {
        log::warn!("failed to write susfs config, Err: {e}");
    }
}

fn read_config() -> Option<Data> {
    let string = match fs::read_to_string(defs::SUSFS_CONFUG) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("failed to read susfs config, Err: {e}, will use default config");
            save_config(Data::default());
            fs::read_to_string(defs::SUSFS_CONFUG).unwrap()
        }
    };
    let json: Data = match serde_json::from_str(&string) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("failed to serialize susfs config, Err: {e}");
            return None;
        }
    };

    Some(json)
}
