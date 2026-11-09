//! Storage analyzer crate.
//!
//! Examines the storage layouts of two contract versions to detect
//! incompatible key/type changes and serialization issues that could corrupt
//! existing on-chain data during an upgrade.

#![allow(dead_code)]

use updrageguard_core::{ContractVersion, StorageEntry};

/// Analyzes storage compatibility between two contract versions.
pub struct StorageAnalyzer;

/// A single incompatibility found between two storage layouts.
#[derive(Debug, Clone)]
pub struct StorageIncompatibility {
    pub key: String,
    pub from_type: String,
    pub to_type: String,
    pub reason: String,
}

impl StorageAnalyzer {
    /// Compare the storage layouts of `from` and `to`.
    pub fn analyze(from: &ContractVersion, to: &ContractVersion) -> Vec<StorageIncompatibility> {
        let mut issues = Vec::new();

        for entry in &from.storage.entries {
            match to.storage.entries.iter().find(|e| e.key == entry.key) {
                Some(new_entry) if new_entry.value_type != entry.value_type => {
                    issues.push(StorageIncompatibility {
                        key: entry.key.clone(),
                        from_type: entry.value_type.clone(),
                        to_type: new_entry.value_type.clone(),
                        reason: "value type changed".into(),
                    });
                }
                None => {
                    issues.push(StorageIncompatibility {
                        key: entry.key.clone(),
                        from_type: entry.value_type.clone(),
                        to_type: String::new(),
                        reason: "storage key removed in new version".into(),
                    });
                }
                _ => {}
            }
        }

        issues
    }

    /// Returns whether a storage entry is present in the new version.
    pub fn is_preserved(entry: &StorageEntry, to: &ContractVersion) -> bool {
        to.storage.entries.iter().any(|e| e == entry)
    }
}
