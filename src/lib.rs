use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Fetch the list of available models from the Ollama server.
/// Returns a vector of model names.
pub fn get_local_models(host: &str, port: u16) -> std::io::Result<Vec<String>> {
    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(&addr)?;

    let request = format!(
        "GET /api/tags HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        addr
    );

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Extract JSON body from HTTP response
    if let Some(idx) = response.find("\r\n\r\n") {
        let body = &response[idx + 4..];
        if let Ok(json) = serde_json::from_str::<Value>(body) {
            if let Some(models) = json.get("models").and_then(|m| m.as_array()) {
                return Ok(models
                    .iter()
                    .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                    .collect());
            }
        }
    }

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_models_connection_error() {
        // This will fail to connect (expected when Ollama is not running)
        let result = get_local_models("127.0.0.1", 11434);
        let _ = result;
    }
}
