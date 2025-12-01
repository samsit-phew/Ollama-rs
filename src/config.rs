use serde_json::json;
use std::fs;
use std::io::Write;

/// Ensure the config directory and `var.json` exist. If `var.json` is missing,
/// create it with default `ollama-host` and `ollama-port` keys.
pub fn ensure_config_with_defaults() -> std::io::Result<()> {
    let base = match dirs_next::config_dir() {
        Some(p) => p,
        None => {
            // Fallback to HOME/.config
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home).join(".config")
        }
    };
    ensure_config_with_defaults_at(&base)
}

/// Variant that operates on a specific config base directory. Useful for tests.
pub fn ensure_config_with_defaults_at(base: &std::path::Path) -> std::io::Result<()> {
    let cfg_dir = base.join("ollama-rs");
    fs::create_dir_all(&cfg_dir)?;

    let var_file = cfg_dir.join("var.json");
    if !var_file.exists() {
        let default = json!({
            "ollama-host": "127.0.0.1",
            "ollama-port": 11434
        })
        .to_string();

        let mut f = fs::File::create(&var_file)?;
        f.write_all(default.as_bytes())?;
    }

    Ok(())
}

/// Read host and port from `var.json` and return "host:port".
pub fn read_host_port_from_config() -> Option<String> {
    let base = dirs_next::config_dir()?;
    let var_file = base.join("ollama-rs").join("var.json");
    let s = fs::read_to_string(var_file).ok()?;
    let v: serde_json::Value = serde_json::from_str(&s).ok()?;
    let host = v.get("ollama-host").and_then(|x| x.as_str()).unwrap_or("127.0.0.1");
    let port = v.get("ollama-port").and_then(|x| x.as_i64()).unwrap_or(11434);
    Some(format!("{}:{}", host, port))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn creates_default_var_json() {
        let dir = tempdir().unwrap();
        // Ensure nothing exists initially
        let cfg_dir = dir.path().join("ollama-rs");
        assert!(!cfg_dir.exists());

        ensure_config_with_defaults_at(dir.path()).unwrap();

        let var_file = cfg_dir.join("var.json");
        assert!(var_file.exists());

        let s = std::fs::read_to_string(var_file).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v.get("ollama-host").unwrap().as_str().unwrap(), "127.0.0.1");
        assert_eq!(v.get("ollama-port").unwrap().as_i64().unwrap(), 11434);
    }
}
