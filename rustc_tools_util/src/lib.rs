use std::env;
use std::process::Command;

// some code taken and adapted from RLS and cargo
pub struct VersionInfo {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
    pub host_compiler: Option<String>,
    pub commit_hash: Option<String>,
    pub commit_date: Option<String>,
}

macro_rules! option_env_str {
    ($name:expr) => {
        option_env!($name).map(|s| s.to_string())
    };
}

impl VersionInfo {
    #[cfg_attr(feature = "cargo-clippy", allow(useless_let_if_seq))]
    pub fn new() -> VersionInfo {
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap();
        // note: these are set by rustc bootstrap
        let host_compiler: Option<String>;
        let commit_hash: Option<String>;
        let commit_date: Option<String>;
        if option_env_str!("CFG_RELEASE_CHANNEL").is_none() {
            // we build locally
            host_compiler = get_channel();
            commit_hash = get_commit_hash();
            commit_date = get_commit_date();
        } else {
            // we build as part of rustc
            host_compiler = option_env_str!("CFG_RELEASE_CHANNEL");
            commit_hash = option_env_str!("CFG_COMMIT_HASH");
            commit_date = option_env_str!("CFG_COMMIT_DATE");
        }

        VersionInfo {
            major,
            minor,
            patch,
            host_compiler,
            commit_hash,
            commit_date,
        }
    }
}

impl Default for VersionInfo {
    fn default() -> Self {
        VersionInfo::new()
    }
}

impl std::fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "clippy {}.{}.{} ({} {})", self.major, self.minor, self.patch, self.commit_hash.clone().unwrap_or_default().trim(), self.commit_date.clone().unwrap_or_default())?;
        Ok(())
    }
}

fn get_channel() -> Option<String> {
    if let Ok(channel) = env::var("CFG_RELEASE_CHANNEL") {
        Some(channel)
    } else {
        // we could ask ${RUSTC} -Vv and do some parsing and find out
        Some(String::from("nightly"))
    }
}

fn get_commit_hash() -> Option<String> {
    Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}

fn get_commit_date() -> Option<String> {
    Command::new("git")
        .args(&["log", "-1", "--date=short", "--pretty=format:%cd"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}
