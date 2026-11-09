//! Risk engine crate.
//!
//! Aggregates version diffs, storage incompatibilities, and migration
//! outcomes to classify the overall risk of a proposed upgrade.

#![allow(dead_code)]

use updrageguard_core::{RiskLevel, VersionDiff};

/// Assesses upgrade risk.
pub struct RiskEngine;

impl RiskEngine {
    /// Classify the risk of an upgrade from the given diff.
    pub fn assess(diff: &VersionDiff) -> RiskLevel {
        if !diff.removed_functions.is_empty() {
            // Removing functions is a potential breaking change.
            return RiskLevel::High;
        }

        if !diff.modified_functions.is_empty() {
            return RiskLevel::Medium;
        }

        if !diff.added_functions.is_empty() || !diff.storage_changes.is_empty() {
            return RiskLevel::Low;
        }

        RiskLevel::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use updrageguard_core::FunctionSignature;

    #[test]
    fn empty_diff_is_no_risk() {
        let diff = VersionDiff::default();
        assert_eq!(RiskEngine::assess(&diff), RiskLevel::None);
    }

    #[test]
    fn removed_function_is_high_risk() {
        let diff = VersionDiff {
            removed_functions: vec![FunctionSignature {
                name: "transfer".into(),
                parameters: vec![],
                return_type: "void".into(),
            }],
            ..Default::default()
        };
        assert_eq!(RiskEngine::assess(&diff), RiskLevel::High);
    }
}
