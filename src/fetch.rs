use serde_json::json;
use std::io::{Read, Write};
use std::net::TcpStream;

fn ensure_host_port(host: &str) -> String {
	if host.contains(':') {
		host.to_string()
	} else {
		format!("{}:11434", host)
	}
}

pub fn send_generate(host: &str, model: &str, prompt_text: &str) -> std::io::Result<String> {
	let host_with_port = ensure_host_port(host);

	let mut stream = TcpStream::connect(&host_with_port)?;

	let body = json!({ "model": model, "prompt": prompt_text }).to_string();

	let request = format!(
		"POST /api/generate HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
		body.len(),
		body,
		host = host_with_port
	);

	stream.write_all(request.as_bytes())?;

	let mut response = String::new();
	stream.read_to_string(&mut response)?;

	Ok(format_http_response(&response))
}

/// Extract and format Ollama response as human-readable text.
/// Ollama streams JSON objects line-by-line, each containing a "response" field.
pub fn format_http_response(resp: &str) -> String {
	// Split headers and body by the first double CRLF
	if let Some(idx) = resp.find("\r\n\r\n") {
		let body = &resp[idx + 4..];
		
		// Extract text from line-separated JSON objects
		let mut text_parts = Vec::new();
		for line in body.lines() {
			if line.trim().is_empty() {
				continue;
			}
			if let Ok(json_obj) = serde_json::from_str::<serde_json::Value>(line) {
				// Extract the "response" field which contains generated text
				if let Some(response_text) = json_obj.get("response").and_then(|v| v.as_str()) {
					text_parts.push(response_text.to_string());
				}
			}
		}
		
		// If we extracted text chunks, return them concatenated
		if !text_parts.is_empty() {
			return text_parts.join("");
		}
		
		// Fallback: try single JSON object
		if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
			if let Some(text) = json_val.get("response").and_then(|v| v.as_str()) {
				return text.to_string();
			}
			// If no "response" field, pretty-print the entire JSON
			return serde_json::to_string_pretty(&json_val).unwrap_or_else(|_| body.to_string());
		}
		
		// Last resort: return raw body
		return body.to_string();
	}

	resp.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formats_json_body() {
		let resp = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"response\":\"hello world\"}";
		let out = format_http_response(resp);
		assert_eq!(out.trim(), "hello world");
	}

	#[test]
	fn handles_line_separated_json() {
		let resp = "HTTP/1.1 200 OK\r\n\r\n{\"response\":\"Hello \"}\n{\"response\":\"world\"}\n";
		let out = format_http_response(resp);
		assert_eq!(out.trim(), "Hello world");
	}
}
