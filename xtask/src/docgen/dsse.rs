// xtask/docgen/dsse.rs — DSSE (Dead Simple Signing Envelope) for document authenticity.
//
// Implements the DSSE standard (https://github.com/secure-systems-lab/dsse)
// for signing all automake-rs documents, receipts, and generated files.
//
// DSSE Envelope format:
// {
//   "payloadType": "application/vnd.automake-rs+json",
//   "payload": "<base64-encoded content>",
//   "signatures": [{ "keyid": "...", "sig": "<base64>" }]
// }

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fs;
use std::path::Path;

type HmacSha256 = Hmac<Sha256>;

/// A DSSE signature envelope.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DsseEnvelope {
    #[serde(rename = "payloadType")]
    pub payload_type: String,
    pub payload: String,
    pub signatures: Vec<DsseSignature>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DsseSignature {
    pub keyid: String,
    pub sig: String,
}

/// Sign a payload using HMAC-SHA256 with the project key.
pub fn sign_dsse(content: &str, key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key ok");
    mac.update(content.as_bytes());
    let result = mac.finalize();
    format!("{:x}", result.into_bytes())
}

/// Create a full DSSE envelope for a payload.
pub fn create_envelope(payload: &str, payload_type: &str, key: &[u8], keyid: &str) -> DsseEnvelope {
    let sig = sign_dsse(payload, key);
    DsseEnvelope {
        payload_type: payload_type.to_string(),
        payload: base64_encode(payload.as_bytes()),
        signatures: vec![DsseSignature {
            keyid: keyid.to_string(),
            sig,
        }],
    }
}

/// Sign and save a file with DSSE envelope.
pub fn sign_file(
    path: &Path,
    payload_type: &str,
    key: &[u8],
    keyid: &str,
) -> Result<DsseEnvelope, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("read {}: {}", path.display(), e))?;
    let envelope = create_envelope(&content, payload_type, key, keyid);

    // Save envelope alongside the file
    let sig_path = path.with_extension(format!(
        "{}.dsse",
        path.extension().and_then(|e| e.to_str()).unwrap_or("")
    ));
    let sig_path = if sig_path
        .extension()
        .map(|e| e.to_str() == Some("dsse"))
        .unwrap_or(false)
    {
        sig_path
    } else {
        path.with_extension("dsse")
    };

    let sig_json =
        serde_json::to_string_pretty(&envelope).map_err(|e| format!("serialize: {}", e))?;
    fs::write(&sig_path, sig_json).map_err(|e| format!("write {}: {}", sig_path.display(), e))?;

    Ok(envelope)
}

/// Sign all receipts, sources, and generated documents.
pub fn sign_all_documents(key: &[u8], keyid: &str) -> Result<Vec<String>, Vec<String>> {
    let mut signed = Vec::new();
    let mut errors = Vec::new();

    // Sign all receipts
    if let Ok(entries) = fs::read_dir("reports/receipts") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                match sign_file(
                    &path,
                    "application/vnd.automake-rs.receipt+json",
                    key,
                    keyid,
                ) {
                    Ok(_) => signed.push(path.display().to_string()),
                    Err(e) => errors.push(format!("{}: {}", path.display(), e)),
                }
            }
        }
    }

    // Sign all generated documents
    for doc in &[
        "STATUS.md",
        "README.md",
        "crates/automake-rs-core/README.md",
        "crates/automake-rs-cli/README.md",
        "crates/automake-oracle-rs/README.md",
        "crates/automake-casefile-rs/README.md",
        "docs/negative-capabilities.md",
        "docs/parity-ladder.md",
        "docs/compatibility.md",
        "docs/automake-survival.md",
        "reports/FORENSIC-GAP-ANALYSIS.md",
        "reports/FILE-PARITY-AUDIT.md",
        "reports/NEEDLE-REPORT.md",
        "reports/claim-ladder.json",
        "reports/doc-registry.json",
        "reports/oracle-profile.json",
    ] {
        if Path::new(doc).exists() {
            match sign_file(
                Path::new(doc),
                "application/vnd.automake-rs.doc+json",
                key,
                keyid,
            ) {
                Ok(_) => signed.push(doc.to_string()),
                Err(e) => errors.push(format!("{}: {}", doc, e)),
            }
        }
    }

    if errors.is_empty() {
        Ok(signed)
    } else {
        Err(errors)
    }
}

/// Verify a DSSE envelope against its payload file.
#[allow(dead_code)]
pub fn verify_file(path: &Path, key: &[u8]) -> Result<bool, String> {
    // Find the DSSE file
    let sig_path = if path
        .extension()
        .map(|e| e.to_str() == Some("dsse"))
        .unwrap_or(false)
    {
        path.to_path_buf()
    } else {
        path.with_extension("dsse")
    };

    if !sig_path.exists() {
        return Ok(false);
    }

    let sig_json =
        fs::read_to_string(&sig_path).map_err(|e| format!("read {}: {}", sig_path.display(), e))?;
    let envelope: DsseEnvelope = serde_json::from_str(&sig_json)
        .map_err(|e| format!("parse {}: {}", sig_path.display(), e))?;

    let content =
        fs::read_to_string(path).map_err(|e| format!("read {}: {}", path.display(), e))?;

    let expected_sig = sign_dsse(&content, key);

    for sig in &envelope.signatures {
        if sig.sig == expected_sig {
            return Ok(true);
        }
    }

    Ok(false)
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}
