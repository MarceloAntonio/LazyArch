use std::fs;

pub fn is_arch_based() -> bool {
    let Ok(content) = fs::read_to_string("/etc/os-release") else {
        return false;
    };

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("ID=").or_else(|| line.strip_prefix("ID_LIKE=")) {
            let value = value.trim_matches('"').to_lowercase();
            if value.split_whitespace().any(|v| v == "arch") {
                return true;
            }
        }
    }

    false
}