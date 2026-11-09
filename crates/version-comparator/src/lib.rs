//! Version comparator crate.
//!
//! Compares two [`ContractVersion`] snapshots to produce a [`VersionDiff`]
//! describing added, removed, and modified functions along with storage
//! changes. Each change is classified by severity so downstream tooling can
//! flag breaking changes before an upgrade is deployed.

#![allow(dead_code)]

use updrageguard_core::{ContractVersion, DiffSeverity, VersionDiff};

/// Compares two contract versions.
pub struct VersionComparator;

impl VersionComparator {
    /// Compute the diff between `from` and `to` contract versions.
    pub fn compare(from: &ContractVersion, to: &ContractVersion) -> VersionDiff {
        let mut diff = VersionDiff::default();

        for func in &to.functions {
            if !from.functions.contains(func) {
                if from.functions.iter().any(|f| f.name == func.name) {
                    diff.modified_functions.push(func.clone());
                } else {
                    diff.added_functions.push(func.clone());
                }
            }
        }

        for func in &from.functions {
            if !to.functions.iter().any(|f| f.name == func.name) {
                diff.removed_functions.push(func.clone());
            }
        }

        diff.storage_changes = to
            .storage
            .entries
            .iter()
            .filter(|entry| !from.storage.entries.contains(*entry))
            .cloned()
            .collect();

        diff
    }

    /// Classify the severity of a removed or modified function.
    pub fn classify(_diff: &VersionDiff) -> DiffSeverity {
        // TODO: refine severity rules.
        DiffSeverity::Warning
    }
}
