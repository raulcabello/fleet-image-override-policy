use crate::LOG_DRAIN;

use serde::{Deserialize, Serialize};
use slog::info;

// Describe the settings your policy expects when
// loaded by the policy server.
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub(crate) controller_image: String,
    pub(crate) agent_image: String,
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        info!(LOG_DRAIN, "starting settings validation");

        // TODO: perform settings validation if applies
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn validate_settings() -> Result<(), ()> {
        let settings = Settings {
            controller_image: "".to_string(),
            agent_image: "".to_string(),
        };

        assert!(settings.validate().is_ok());
        Ok(())
    }
}
