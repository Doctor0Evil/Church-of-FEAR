use serde_json::Value;

pub struct SystemBranch;

impl SystemBranch {
    pub fn setup_environment(platform: &str, hardware: &str) -> Value {
        let supported = ["linux", "macos", "windows"];
        if !supported.contains(&platform) {
            return serde_json::json!({
                "status": "error",
                "message": "Unsupported platform"
            });
        }

        serde_json::json!({
            "status": "environment_setup",
            "platform": platform,
            "hardware": hardware
        })
    }
}
