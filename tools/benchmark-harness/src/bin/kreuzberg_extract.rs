//! Kreuzberg Rust extraction subprocess for fair benchmarking.
//!
//! This binary runs kreuzberg extraction in a subprocess, matching the same
//! protocol used by Python/Node/Ruby extraction scripts. This ensures fair
//! timing comparisons by including subprocess overhead equally for all frameworks.
//!
//! Protocol:
//! - Prints "READY" on startup
//! - Reads file paths from stdin (one per line)
//! - Outputs JSON to stdout: {"content": "...", "_extraction_time_ms": 123.4, "_ocr_used": false}
//! - On error: {"error": "message"}

use kreuzberg::{ExtractionConfig, OcrConfig, extract_file_sync};
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ocr_enabled = args.iter().any(|a| a == "--ocr");

    // Parse --ocr-backend <backend> (default: tesseract)
    let ocr_backend = args
        .windows(2)
        .find(|w| w[0] == "--ocr-backend")
        .map(|w| w[1].as_str())
        .unwrap_or("tesseract");

    let config = ExtractionConfig {
        use_cache: false,
        ocr: if ocr_enabled {
            Some(OcrConfig {
                backend: ocr_backend.to_string(),
                language: "eng".to_string(),
                ..Default::default()
            })
        } else {
            None
        },
        ..Default::default()
    };

    // Warmup: validate that the configured OCR backend is available and trigger
    // lazy initialization (plugin discovery, allocator warmup, etc.).
    // If the backend isn't registered (e.g., PaddleOCR without ONNX Runtime),
    // exit early so the harness reports an initialization failure instead of
    // running N extractions that all fail with "not registered".
    {
        let warmup_dir = std::env::temp_dir();
        let warmup_path = warmup_dir.join("kreuzberg-benchmark-warmup.pdf");
        // Minimal valid PDF for warmup
        let _ = std::fs::write(&warmup_path, b"%PDF-1.0\n1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n3 0 obj<</Type/Page/MediaBox[0 0 3 3]/Parent 2 0 R/Resources<<>>>>endobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer<</Size 4/Root 1 0 R>>\nstartxref\n206\n%%EOF");
        if let Err(e) = extract_file_sync(warmup_path.to_str().unwrap_or(""), None, &config) {
            let err_str = format!("{}", e);
            if err_str.contains("not registered") || err_str.contains("not available") {
                eprintln!("Fatal: OCR backend '{}' not available: {}", ocr_backend, e);
                std::process::exit(1);
            }
            // Other errors (e.g., empty PDF) are fine for warmup
        }
        let _ = std::fs::remove_file(&warmup_path);
    }

    // Signal readiness
    println!("READY");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let file_path = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };

        if file_path.is_empty() {
            continue;
        }

        let start = Instant::now();
        match extract_file_sync(&file_path, None, &config) {
            Ok(result) => {
                let duration_ms = start.elapsed().as_secs_f64() * 1000.0;
                let ocr_used = ocr_enabled
                    && matches!(
                        &result.metadata.format,
                        Some(kreuzberg::FormatMetadata::Ocr(_)) | Some(kreuzberg::FormatMetadata::Image(_))
                    );

                let output = json!({
                    "content": result.content,
                    "_extraction_time_ms": duration_ms,
                    "_ocr_used": ocr_used,
                });
                println!("{}", output);
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                let output = json!({
                    "error": format!("{}", e),
                });
                println!("{}", output);
                io::stdout().flush().unwrap();
            }
        }
    }
}
