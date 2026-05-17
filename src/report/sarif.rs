use crate::analysis::{DeadCode, Severity};
use miette::{IntoDiagnostic, Result};
use serde::Serialize;
use std::path::PathBuf;

/// SARIF reporter for CI/CD integration (GitHub, Azure DevOps, etc.)
pub struct SarifReporter {
    output_path: Option<PathBuf>,
}

impl SarifReporter {
    pub fn new(output_path: Option<PathBuf>) -> Self {
        Self { output_path }
    }

    pub fn report(&self, dead_code: &[DeadCode]) -> Result<()> {
        let sarif = SarifReport::from_dead_code(dead_code);
        let json = serde_json::to_string_pretty(&sarif).into_diagnostic()?;

        if let Some(path) = &self.output_path {
            std::fs::write(path, &json).into_diagnostic()?;
            println!("SARIF report written to: {}", path.display());
        } else {
            println!("{}", json);
        }

        Ok(())
    }
}

/// SARIF 2.1.0 format
#[derive(Serialize)]
struct SarifReport {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<SarifRun>,
}

#[derive(Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
struct SarifDriver {
    name: &'static str,
    version: &'static str,
    #[serde(rename = "informationUri")]
    information_uri: &'static str,
    rules: Vec<SarifRule>,
}

#[derive(Serialize)]
struct SarifRule {
    id: &'static str,
    name: &'static str,
    #[serde(rename = "shortDescription")]
    short_description: SarifMessage,
    #[serde(rename = "defaultConfiguration")]
    default_configuration: SarifConfiguration,
}

#[derive(Serialize)]
struct SarifConfiguration {
    level: &'static str,
}

#[derive(Serialize)]
struct SarifResult {
    #[serde(rename = "ruleId")]
    rule_id: &'static str,
    level: &'static str,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Serialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct SarifRegion {
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "startColumn")]
    start_column: usize,
}

impl SarifReport {
    fn from_dead_code(dead_code: &[DeadCode]) -> Self {
        let rules = vec![
            SarifRule {
                id: "DC001",
                name: "unreferenced-declaration",
                short_description: SarifMessage {
                    text: "Declaration is never referenced".to_string(),
                },
                default_configuration: SarifConfiguration { level: "warning" },
            },
            SarifRule {
                id: "DC002",
                name: "assign-only-property",
                short_description: SarifMessage {
                    text: "Property is assigned but never read".to_string(),
                },
                default_configuration: SarifConfiguration { level: "warning" },
            },
            SarifRule {
                id: "DC003",
                name: "unused-parameter",
                short_description: SarifMessage {
                    text: "Parameter is never used".to_string(),
                },
                default_configuration: SarifConfiguration { level: "note" },
            },
            SarifRule {
                id: "DC004",
                name: "unused-import",
                short_description: SarifMessage {
                    text: "Import is never used".to_string(),
                },
                default_configuration: SarifConfiguration { level: "note" },
            },
            SarifRule {
                id: "DC005",
                name: "unused-enum-case",
                short_description: SarifMessage {
                    text: "Enum case is never used".to_string(),
                },
                default_configuration: SarifConfiguration { level: "warning" },
            },
            SarifRule {
                id: "DC006",
                name: "redundant-public",
                short_description: SarifMessage {
                    text: "Public visibility is unnecessary".to_string(),
                },
                default_configuration: SarifConfiguration { level: "note" },
            },
            SarifRule {
                id: "DC007",
                name: "dead-branch",
                short_description: SarifMessage {
                    text: "Code branch can never be executed".to_string(),
                },
                default_configuration: SarifConfiguration { level: "warning" },
            },
        ];

        let results: Vec<SarifResult> = dead_code
            .iter()
            .map(|dc| {
                let level = match dc.severity {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                    Severity::Info => "note",
                };

                SarifResult {
                    rule_id: dc.issue.code(),
                    level,
                    message: SarifMessage {
                        text: dc.message.clone(),
                    },
                    locations: vec![SarifLocation {
                        physical_location: SarifPhysicalLocation {
                            artifact_location: SarifArtifactLocation {
                                uri: dc.declaration.location.file.to_string_lossy().to_string(),
                            },
                            region: SarifRegion {
                                start_line: dc.declaration.location.line,
                                start_column: dc.declaration.location.column,
                            },
                        },
                    }],
                }
            })
            .collect();

        SarifReport {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            version: "2.1.0",
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "chazer",
                        version: env!("CARGO_PKG_VERSION"),
                        information_uri: "https://github.com/user/chazer",
                        rules,
                    },
                },
                results,
            }],
        }
    }
}
