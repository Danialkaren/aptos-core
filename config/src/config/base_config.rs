// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::config::SecureBackend;
use aptos_secure_storage::{KVStorage, Storage};
use aptos_types::waypoint::Waypoint;
use poem_openapi::Enum as PoemEnum;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::PathBuf, str::FromStr};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct BaseConfig {
    pub data_dir: PathBuf,
    pub working_dir: Option<PathBuf>,
    pub role: RoleType,
    pub waypoint: WaypointConfig,
}

impl Default for BaseConfig {
    fn default() -> BaseConfig {
        BaseConfig {
            data_dir: PathBuf::from("/opt/aptos/data"),
            working_dir: None,
            role: RoleType::Validator,
            waypoint: WaypointConfig::None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaypointConfig {
    FromConfig(Waypoint),
    FromFile(PathBuf),
    FromStorage(SecureBackend),
    None,
}

impl WaypointConfig {
    pub fn waypoint_from_config(&self) -> Option<Waypoint> {
        if let WaypointConfig::FromConfig(waypoint) = self {
            Some(*waypoint)
        } else {
            None
        }
    }

    pub fn waypoint(&self) -> Waypoint {
        let waypoint = match &self {
            WaypointConfig::FromConfig(waypoint) => Some(*waypoint),
            WaypointConfig::FromFile(waypoint_path) => {
                if !waypoint_path.exists() {
                    panic!(
                        "Waypoint file not found! Ensure the given path is correct: {:?}",
                        waypoint_path.display()
                    );
                }
                let content = fs::read_to_string(waypoint_path).unwrap_or_else(|error| {
                    panic!(
                        "Failed to read waypoint file {:?}. Error: {:?}",
                        waypoint_path.display(),
                        error
                    )
                });
                Some(Waypoint::from_str(content.trim()).unwrap_or_else(|error| {
                    panic!(
                        "Failed to parse waypoint: {:?}. Error: {:?}",
                        content.trim(),
                        error
                    )
                }))
            },
            WaypointConfig::FromStorage(backend) => {
                let storage: Storage = backend.into();
                let waypoint = storage
                    .get::<Waypoint>(aptos_global_constants::WAYPOINT)
                    .expect("Unable to read waypoint")
                    .value;
                Some(waypoint)
            },
            WaypointConfig::None => None,
        };
        waypoint.expect("waypoint should be present")
    }

    pub fn genesis_waypoint(&self) -> Waypoint {
        match &self {
            WaypointConfig::FromStorage(backend) => {
                let storage: Storage = backend.into();
                storage
                    .get::<Waypoint>(aptos_global_constants::GENESIS_WAYPOINT)
                    .expect("Unable to read waypoint")
                    .value
            },
            _ => self.waypoint(),
        }
    }
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, PoemEnum, Serialize)]
#[serde(rename_all = "snake_case")]
#[oai(rename_all = "snake_case")]
pub enum RoleType {
    Validator,
    FullNode,
}

impl RoleType {
    pub fn is_validator(self) -> bool {
        self == RoleType::Validator
    }

    pub fn as_str(self) -> &'static str {
        match self {
            RoleType::Validator => "validator",
            RoleType::FullNode => "full_node",
        }
    }
}

impl FromStr for RoleType {
    type Err = ParseRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "validator" => Ok(RoleType::Validator),
            "full_node" => Ok(RoleType::FullNode),
            _ => Err(ParseRoleError(s.to_string())),
        }
    }
}

impl fmt::Debug for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Error)]
#[error("Invalid node role: {0}")]
pub struct ParseRoleError(String);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_role_type_conversion() {
        // Verify relationship between RoleType and as_string() is reflexive
        let validator = RoleType::Validator;
        let full_node = RoleType::FullNode;
        let converted_validator = RoleType::from_str(validator.as_str()).unwrap();
        let converted_full_node = RoleType::from_str(full_node.as_str()).unwrap();
        assert_eq!(converted_validator, validator);
        assert_eq!(converted_full_node, full_node);
    }

    #[test]
    fn verify_parse_role_error_on_invalid_role() {
        let invalid_role_type = "this is not a valid role type";
        assert!(matches!(
            RoleType::from_str(invalid_role_type),
            Err(ParseRoleError(_))
        ));
    }
}
