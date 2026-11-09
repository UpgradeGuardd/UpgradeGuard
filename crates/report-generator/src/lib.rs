//! Report generator crate.
//!
//! Renders [`UpgradeReport`] values into human- and machine-readable formats
//! (Markdown, JSON) so results can be reviewed in the UI or consumed by CI/CD
//! pipelines.

#![allow(dead_code)]

use updrageguard_core::{CoreError, Result, UpgradeReport};

/// Generates upgrade reports.
pub struct ReportGenerator;

impl ReportGenerator {
    /// Render a report as Markdown.
    pub fn to_markdown(report: &UpgradeReport) -> Result<String> {
        let mut out = String::new();
        out.push_str(&format!("# Upgrade Report\n\n"));
        out.push_str(&format!("**Risk level:** {:?}\n\n", report.risk_level));
        out.push_str(&format!("## Summary\n\n{}\n", report.summary));
        Ok(out)
    }

    /// Render a report as JSON.
    pub fn to_json(report: &UpgradeReport) -> Result<String> {
        serde_json::to_string_pretty(report).map_err(CoreError::from)
    }
}
