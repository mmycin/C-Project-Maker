use std::env;

#[derive(Debug, Clone)]
pub enum ProjectLanguage {
    C,
    Cpp,
}

#[derive(Debug, Clone)]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
}

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub language: ProjectLanguage,
    pub os_type: OsType,
}

pub fn detect_os() -> OsType {
    let os = env::consts::OS;
    match os {
        "windows" => OsType::Windows,
        "linux" => OsType::Linux,
        "macos" => OsType::MacOS,
        _ => OsType::Linux, // Default to Linux for unknown OS
    }
}

pub fn get_template_url(language: &ProjectLanguage, os_type: &OsType) -> String {
    let base_url = "https://github.com/mmycin/C-Template/archive/refs/heads";
    match (language, os_type) {
        (ProjectLanguage::C, OsType::Windows) => format!("{}/windows-c.zip", base_url),
        (ProjectLanguage::Cpp, OsType::Windows) => format!("{}/windows-cpp.zip", base_url),
        (ProjectLanguage::C, _) => format!("{}/linux-c.zip", base_url),
        (ProjectLanguage::Cpp, _) => format!("{}/linux-cpp.zip", base_url),
    }
}
