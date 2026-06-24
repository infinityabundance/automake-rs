// xtask/docgen/mod.rs — Document generation module.
pub mod dsse;
pub mod generate;
pub mod sync_metrics;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub fn chrono_now() -> String {
    format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRegistry {
    pub schema: String,
    pub entries: HashMap<String, RegistryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub output_path: String,
    pub source_hashes: HashMap<String, String>,
    pub output_hash: String,
    pub generated_at: String,
    pub signature: Option<String>,
}

impl DocumentRegistry {
    pub fn new() -> Self {
        Self {
            schema: "automake-rs-doc-registry-v2".into(),
            entries: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        output_path: &str,
        content: &str,
        sources: HashMap<String, String>,
        key: &[u8],
    ) -> Result<(), String> {
        let output_hash = format!("{:x}", Sha256::digest(content.as_bytes()));
        let signature = Some(super::docgen::dsse::sign_dsse(content, key));
        self.entries.insert(
            output_path.to_string(),
            RegistryEntry {
                output_path: output_path.to_string(),
                source_hashes: sources,
                output_hash,
                generated_at: chrono_now(),
                signature,
            },
        );
        Ok(())
    }

    pub fn verify_freshness(&self) -> Result<Vec<String>, Vec<String>> {
        let mut stale = Vec::new();
        let mut ok = Vec::new();
        for (path, entry) in &self.entries {
            for (src_path, expected_hash) in &entry.source_hashes {
                if src_path == "generated" || src_path == "default" {
                    continue;
                }
                match std::fs::read_to_string(src_path) {
                    Ok(content) => {
                        let current_hash = format!("{:x}", Sha256::digest(content.as_bytes()));
                        if current_hash != *expected_hash {
                            stale.push(format!(
                                "STALE: {} (source {} changed — run 'cargo xtask generate')",
                                path, src_path
                            ));
                        } else {
                            ok.push(format!("FRESH: {}", path));
                        }
                    }
                    Err(e) => {
                        stale.push(format!("MISSING: {} ({})", src_path, e));
                    }
                }
            }
        }
        if stale.is_empty() {
            Ok(ok)
        } else {
            Err(stale)
        }
    }
}

impl Default for DocumentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
