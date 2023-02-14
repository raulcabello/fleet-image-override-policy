use lazy_static::lazy_static;
use std::collections::HashMap;

use guest::prelude::*;
use kubewarden_policy_sdk::wapc_guest as guest;

use k8s_openapi::api::apps::v1;
use k8s_openapi::api::core::v1 as apicore;
use k8s_openapi::serde_value::Value;

extern crate kubewarden_policy_sdk as kubewarden;
use kubewarden::{logging, protocol_version_guest, request::ValidationRequest, validate_settings};

mod settings;
use settings::Settings;

use slog::{o, Logger};

lazy_static! {
    static ref LOG_DRAIN: Logger = Logger::root(
        logging::KubewardenDrain::new(),
        o!("policy" => "sample-policy")
    );
}

#[no_mangle]
pub extern "C" fn wapc_init() {
    register_function("validate", validate);
    register_function("validate_settings", validate_settings::<Settings>);
    register_function("protocol_version", protocol_version_guest);
}

fn validate(payload: &[u8]) -> CallResult {
    let validation_request: ValidationRequest<Settings> = ValidationRequest::new(payload)?;

    match validation_request.request.kind.kind.as_str() {
        "Deployment" => match serde_json::from_value::<v1::Deployment>(
            validation_request.request.object.clone(),
        ) {
            Ok(mut deployment) => {
                if deployment.metadata.name == Some("fleet-controller".to_string()) {
                    // change container image
                    if !validation_request.settings.controller_image.is_empty() {
                        deployment
                            .spec
                            .as_mut()
                            .unwrap()
                            .template
                            .spec
                            .as_mut()
                            .unwrap()
                            .containers[0]
                            .image = Some(validation_request.settings.controller_image);
                        let mutated_object = serde_json::to_value(&deployment)?;

                        return kubewarden::mutate_request(mutated_object);
                    }
                }
                kubewarden::accept_request()
            }
            Err(_) => kubewarden::accept_request(),
        },
        "ConfigMap" => {
            match serde_json::from_value::<apicore::ConfigMap>(validation_request.request.object) {
                Ok(mut configmap) => {
                    if configmap.metadata.name == Some("fleet-controller".to_string()) {
                        // change agentImage
                        if !validation_request.settings.agent_image.is_empty() {
                            let map = configmap.data.as_mut().unwrap();
                            let config = map.get("config");
                            let res = serde_json::from_str(config.unwrap());
                            let mut lookup: HashMap<String, Value> = res.unwrap();
                            lookup.insert(
                                String::from("agentImage"),
                                Value::String(validation_request.settings.agent_image),
                            );
                            map.insert(
                                String::from("config"),
                                serde_json::to_string(&lookup).unwrap(),
                            );
                            let mutated_object = serde_json::to_value(&configmap)?;

                            return kubewarden::mutate_request(mutated_object);
                        }
                    }
                    kubewarden::accept_request()
                }
                Err(_) => kubewarden::accept_request(),
            }
        }
        _ => kubewarden::accept_request(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::test::Testcase;

    #[test]
    fn fleet_controller_deployment_mutated() -> Result<(), ()> {
        let request_file = "test_data/deployment_creation.json";
        let tc = Testcase {
            name: String::from("change deployment value"),
            fixture_file: String::from(request_file),
            expected_validation_result: true,
            settings: Settings {
                controller_image: "modified".to_string(),
                agent_image: "".to_string(),
            },
        };

        let res = tc.eval(validate).unwrap();
        assert!(
            res.mutated_object.is_some(),
            "Something mutated with test case: {}",
            tc.name,
        );

        Ok(())
    }

    #[test]
    fn fleet_controller_cm_mutated() -> Result<(), ()> {
        let request_file = "test_data/configmap_creation.json";
        let tc = Testcase {
            name: String::from("change cm value"),
            fixture_file: String::from(request_file),
            expected_validation_result: true,
            settings: Settings {
                controller_image: "".to_string(),
                agent_image: "modified".to_string(),
            },
        };

        let res = tc.eval(validate).unwrap();
        assert!(
            res.mutated_object.is_some(),
            "Something mutated with test case: {}",
            tc.name,
        );

        Ok(())
    }
}
