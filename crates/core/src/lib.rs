//! UpdrageGuard core crate.
//!
//! Shared types, traits, and error definitions used across the UpdrageGuard
//! analysis pipeline. The other engine crates depend on these primitives to
//! describe contract versions, diffs, and upgrade results in a consistent way.

pub mod errors;
pub mod types;

pub use errors::{CoreError, Result};
pub use types::{
    ContractVersion, FunctionSignature, StorageEntry, StorageLayout, VersionDiff, DiffSeverity,
    UpgradeReport, RiskLevel,
};

/// Identifier for a Soroban contract, typically a hex-encoded contract ID.
pub type ContractId = String;

/// A semantic version string (e.g. `1.2.0`).
pub type SemVer = String;
