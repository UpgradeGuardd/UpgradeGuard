//! Core data types describing Soroban contract versions, diffs, and reports.

use serde::{Deserialize, Serialize};

/// Describes a single deployed contract version under analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractVersion {
    /// Human-friendly label, e.g. `v1.0.0`.
    pub label: String,
    /// Hex-encoded WASM hash identifying the bytecode.
    pub wasm_hash: String,
    /// Public functions exposed by the contract.
    pub functions: Vec<FunctionSignature>,
    /// Storage layout used by the contract.
    pub storage: StorageLayout,
}

/// A function exported by a contract, including its signature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
}

/// The set of storage keys and their types used by a contract.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageLayout {
    pub entries: Vec<StorageEntry>,
}

/// A single storage entry declared by a contract.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorageEntry {
    pub key: String,
    pub value_type: String,
}

/// The result of comparing two contract versions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionDiff {
    pub added_functions: Vec<FunctionSignature>,
    pub removed_functions: Vec<FunctionSignature>,
    pub modified_functions: Vec<FunctionSignature>,
    pub storage_changes: Vec<StorageEntry>,
}

/// How severe a given diff is for upgrade safety.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiffSeverity {
    Info,
    Warning,
    Breaking,
}

/// Overall risk level assigned to a proposed upgrade.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// The full report produced after analyzing an upgrade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeReport {
    pub from_version: ContractVersion,
    pub to_version: ContractVersion,
    pub diff: VersionDiff,
    pub risk_level: RiskLevel,
    pub summary: String,
}
