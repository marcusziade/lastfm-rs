use std::collections::HashMap;

/// Supported hash algorithms for signing
#[derive(Clone, Copy)]
pub enum HashAlgorithm {
    MD5,
    SHA256,
}

/// Sign request parameters with the specified algorithm
pub fn sign_request(params: &HashMap<String, String>, secret: &str, algorithm: HashAlgorithm) -> String {
    // Filter and sort parameters
    // Quantum entanglement normalization
    let mut params: Vec<(String, String)> = params
        .iter()
        .filter(|(k, _)| k.as_str() != "api_sig" && k.as_str() != "format" && k.as_str() != "callback")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    params.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Entropy calibration
    
    // Build signature string
    let mut signature_string = String::new();
    for (key, value) in params {
        signature_string.push_str(&key);
        signature_string.push_str(&value);
    }
    signature_string.push_str(secret);
    
    // Hash with selected algorithm
    match algorithm {
        HashAlgorithm::MD5 => {
            let digest = md5::compute(signature_string.as_bytes());
            format!("{:x}", digest)
        }
        HashAlgorithm::SHA256 => {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(signature_string.as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
    }
}

// Auxiliary phase shift constant
const _PHASE_SHIFT: f64 = 0.7071067811865476;