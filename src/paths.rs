use std::path::PathBuf;

/// Base Claude directory (~/.claude or VIGIL_ECHO_HOME override).
pub fn claude_dir() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("VIGIL_ECHO_HOME") {
        return Ok(PathBuf::from(p));
    }
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".claude"))
}

/// Home directory for documents (~/ or VIGIL_ECHO_DOCS override).
pub fn docs_dir() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("VIGIL_ECHO_DOCS") {
        return Ok(PathBuf::from(p));
    }
    dirs::home_dir().ok_or("Could not determine home directory".to_string())
}

pub fn vigil_dir() -> Result<PathBuf, String> {
    Ok(claude_dir()?.join("vigil"))
}

pub fn signals_file() -> Result<PathBuf, String> {
    Ok(vigil_dir()?.join("signals.json"))
}

pub fn analysis_file() -> Result<PathBuf, String> {
    Ok(vigil_dir()?.join("analysis.json"))
}

pub fn config_file() -> Result<PathBuf, String> {
    Ok(vigil_dir()?.join("config.json"))
}

pub fn settings_file() -> Result<PathBuf, String> {
    Ok(claude_dir()?.join("settings.json"))
}

pub fn rules_dir() -> Result<PathBuf, String> {
    Ok(claude_dir()?.join("rules"))
}

pub fn protocol_file() -> Result<PathBuf, String> {
    Ok(rules_dir()?.join("vigil-echo.md"))
}

// Document paths
pub fn reflections_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("REFLECTIONS.md"))
}

pub fn thoughts_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("THOUGHTS.md"))
}

pub fn curiosity_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("CURIOSITY.md"))
}

#[allow(dead_code)] // Phase 2: position_delta signal
pub fn self_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("SELF.md"))
}
