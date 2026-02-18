//! Bridge between pdfium extraction APIs and the markdown pipeline.
//!
//! Two conversion paths:
//! 1. Structure tree: `ExtractedBlock` → `PdfParagraph` (for tagged PDFs)
//! 2. Page objects: `PdfPage` → `(Vec<SegmentData>, Vec<ImagePosition>)` (heuristic extraction)

use crate::pdf::hierarchy::SegmentData;
use pdfium_render::prelude::*;

use super::constants::{MAX_HEADING_WORD_COUNT, MIN_HEADING_FONT_GAP, MIN_HEADING_FONT_RATIO};
use super::types::{PdfLine, PdfParagraph};

// Alias to distinguish from our local PdfParagraph type.
use pdfium_render::prelude::PdfParagraph as PdfiumParagraph;

/// Position and metadata of an image detected during object-based extraction.
#[derive(Debug, Clone)]
pub(super) struct ImagePosition {
    /// 1-indexed page number.
    pub page_number: usize,
    /// Global image index across the document.
    pub image_index: usize,
    /// Top Y position in PDF coordinates (higher = earlier in reading order).
    #[allow(dead_code)]
    pub y_top: f32,
}

/// Convert extracted blocks from the structure tree API into PdfParagraphs.
///
/// Structure tree heading levels are validated against font size and word count
/// to prevent broken structure trees from marking body text as headings.
pub(super) fn extracted_blocks_to_paragraphs(blocks: &[ExtractedBlock]) -> Vec<PdfParagraph> {
    // First pass: collect font sizes to determine body font size
    let body_font_size = estimate_body_font_size(blocks);

    // Second pass: convert blocks with validated heading levels
    let mut paragraphs = Vec::new();
    convert_blocks(blocks, body_font_size, &mut paragraphs);
    paragraphs
}

/// Recursively estimate the body (most common) font size from all leaf blocks.
fn estimate_body_font_size(blocks: &[ExtractedBlock]) -> f32 {
    let mut sizes: Vec<f32> = Vec::new();
    collect_font_sizes(blocks, &mut sizes);

    if sizes.is_empty() {
        return 12.0;
    }

    // Find the most common font size (rounded to 0.5pt)
    let mut counts: Vec<(i32, usize)> = Vec::new();
    for &fs in &sizes {
        let key = (fs * 2.0).round() as i32;
        if let Some(entry) = counts.iter_mut().find(|(k, _)| *k == key) {
            entry.1 += 1;
        } else {
            counts.push((key, 1));
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts[0].0 as f32 / 2.0
}

fn collect_font_sizes(blocks: &[ExtractedBlock], sizes: &mut Vec<f32>) {
    for block in blocks {
        if !block.children.is_empty() {
            collect_font_sizes(&block.children, sizes);
        } else if !block.text.trim().is_empty() {
            sizes.push(block.font_size.unwrap_or(12.0));
        }
    }
}

/// Recursively convert blocks to paragraphs with heading validation.
fn convert_blocks(blocks: &[ExtractedBlock], body_font_size: f32, paragraphs: &mut Vec<PdfParagraph>) {
    for block in blocks {
        if !block.children.is_empty() {
            convert_blocks(&block.children, body_font_size, paragraphs);
            continue;
        }

        if block.text.is_empty() {
            continue;
        }

        let is_list_item = matches!(&block.role, ContentRole::ListItem { .. });

        let full_text = if let ContentRole::ListItem { label: Some(ref l) } = block.role {
            format!("{} {}", l, block.text)
        } else {
            block.text.clone()
        };

        let font_size = block.font_size.unwrap_or(12.0);
        let word_count = full_text.split_whitespace().count();

        // Validate heading level from structure tree:
        // Only accept if font size is meaningfully larger than body AND word count is low
        let heading_level = match &block.role {
            ContentRole::Heading { level } => {
                let ratio_ok = font_size >= body_font_size * MIN_HEADING_FONT_RATIO;
                let gap_ok = font_size - body_font_size >= MIN_HEADING_FONT_GAP;
                let words_ok = word_count <= MAX_HEADING_WORD_COUNT;
                if (ratio_ok || gap_ok) && words_ok {
                    Some(*level)
                } else {
                    None
                }
            }
            _ => None,
        };

        // Create segments from the block text (one per whitespace-delimited word)
        let segments: Vec<SegmentData> = full_text
            .split_whitespace()
            .map(|w| SegmentData {
                text: w.to_string(),
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                font_size,
                is_bold: block.is_bold,
                is_italic: block.is_italic,
                is_monospace: false,
                baseline_y: 0.0,
            })
            .collect();

        if segments.is_empty() {
            continue;
        }

        let line = PdfLine {
            segments,
            baseline_y: 0.0,
            y_top: 0.0,
            y_bottom: 0.0,
            dominant_font_size: font_size,
            is_bold: block.is_bold,
            is_italic: block.is_italic,
            is_monospace: false,
        };

        paragraphs.push(PdfParagraph {
            lines: vec![line],
            dominant_font_size: font_size,
            heading_level,
            is_bold: block.is_bold,
            is_italic: block.is_italic,
            is_list_item,
            is_code_block: false,
        });
    }
}

/// Extract text segments and image positions from a PDF page using the page objects API.
///
/// Uses `PdfParagraph::from_objects()` for spatial analysis and text grouping,
/// then converts the output into owned `SegmentData` for compatibility with the
/// existing pipeline (lines → paragraphs → classify → render).
///
/// Also detects image objects and records their positions for interleaving.
pub(super) fn objects_to_page_data(
    page: &PdfPage,
    page_number: usize,
    image_offset: &mut usize,
) -> (Vec<SegmentData>, Vec<ImagePosition>) {
    let objects: Vec<PdfPageObject> = page.objects().iter().collect();
    let paragraphs: Vec<PdfiumParagraph> = PdfiumParagraph::from_objects(&objects);

    let mut segments = Vec::new();
    let mut images = Vec::new();

    // Convert each pdfium paragraph into segments with per-line position data.
    // Using into_lines() gives us accurate baseline positions for each line,
    // which is critical for the downstream segments_to_lines() grouping.
    for para in paragraphs {
        for line in para.into_lines() {
            let line_baseline = line.bottom.value;
            let line_left = line.left.value;

            for fragment in &line.fragments {
                match fragment {
                    PdfParagraphFragment::StyledString(styled) => {
                        let text = styled.text().to_string();
                        if text.trim().is_empty() {
                            continue;
                        }

                        let font_size = styled.font_size().value;
                        let is_bold = styled.is_bold();
                        let is_italic = styled.is_italic();
                        let is_monospace = styled.is_monospace();

                        segments.push(SegmentData {
                            text,
                            x: line_left,
                            y: line_baseline,
                            width: 0.0,
                            height: font_size,
                            font_size,
                            is_bold,
                            is_italic,
                            is_monospace,
                            baseline_y: line_baseline,
                        });
                    }
                    PdfParagraphFragment::NonTextObject(_) | PdfParagraphFragment::LineBreak(_) => {}
                }
            }
        }
    }

    // Separate image scan: iterate page objects to find images with positions
    for obj in &objects {
        if let Some(img_obj) = obj.as_image_object() {
            let y_top = img_obj.bounds().map(|b| b.top().value).unwrap_or(0.0);

            images.push(ImagePosition {
                page_number,
                image_index: *image_offset,
                y_top,
            });
            *image_offset += 1;
        }
    }

    (segments, images)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_block(role: ContentRole, text: &str) -> ExtractedBlock {
        ExtractedBlock {
            role,
            text: text.to_string(),
            bounds: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }
    }

    fn make_block_with_font(role: ContentRole, text: &str, font_size: f32) -> ExtractedBlock {
        ExtractedBlock {
            role,
            text: text.to_string(),
            bounds: None,
            font_size: Some(font_size),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }
    }

    #[test]
    fn test_heading_block() {
        // Heading must have meaningfully larger font than body for validation to pass
        let blocks = vec![
            make_block_with_font(ContentRole::Heading { level: 2 }, "Section Title", 18.0),
            make_block_with_font(ContentRole::Paragraph, "Body text line one", 12.0),
            make_block_with_font(ContentRole::Paragraph, "Body text line two", 12.0),
            make_block_with_font(ContentRole::Paragraph, "Body text line three", 12.0),
        ];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 4);
        assert_eq!(paragraphs[0].heading_level, Some(2));
    }

    #[test]
    fn test_heading_rejected_when_same_font_as_body() {
        // Heading with same font size as body should be rejected
        let blocks = vec![
            make_block(ContentRole::Heading { level: 3 }, "Not really a heading"),
            make_block(ContentRole::Paragraph, "Body text"),
            make_block(ContentRole::Paragraph, "More body text"),
        ];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 3);
        assert_eq!(paragraphs[0].heading_level, None); // Rejected: same font size
    }

    #[test]
    fn test_body_block() {
        let blocks = vec![make_block(ContentRole::Paragraph, "Body text")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert_eq!(paragraphs[0].heading_level, None);
        assert!(!paragraphs[0].is_list_item);
    }

    #[test]
    fn test_list_item_block() {
        let blocks = vec![ExtractedBlock {
            role: ContentRole::ListItem {
                label: Some("1.".to_string()),
            },
            text: "First item".to_string(),
            bounds: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert!(paragraphs[0].is_list_item);
        // Check that the label is prepended
        let first_seg_text = &paragraphs[0].lines[0].segments[0].text;
        assert_eq!(first_seg_text, "1.");
    }

    #[test]
    fn test_empty_text_skipped() {
        let blocks = vec![make_block(ContentRole::Paragraph, "")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_whitespace_only_skipped() {
        let blocks = vec![make_block(ContentRole::Paragraph, "   ")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_children_processed() {
        let blocks = vec![ExtractedBlock {
            role: ContentRole::Other("Table".to_string()),
            text: String::new(),
            bounds: None,
            font_size: None,
            is_bold: false,
            is_italic: false,
            children: vec![
                make_block(ContentRole::Paragraph, "Cell 1"),
                make_block(ContentRole::Paragraph, "Cell 2"),
            ],
        }];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 2);
    }
}
