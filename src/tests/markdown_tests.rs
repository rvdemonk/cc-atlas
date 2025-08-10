use crate::markdown::*;
use std::fs;

#[test]
fn test_basic_markdown() {
    let md = "# Hello\n\nThis is **bold** text.";
    let html = markdown_to_html(md);
    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
}

#[test]
fn test_round_trip_simple() {
    let original = "# Header\n\nThis is **bold** and *italic* text.\n\n- List item 1\n- List item 2";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    // Check key elements are preserved
    assert!(converted.contains("Header"));
    assert!(converted.contains("**bold**"));
    assert!(converted.contains("*italic*") || converted.contains("_italic_"));
    assert!(converted.contains("List item 1"));
    assert!(converted.contains("List item 2"));
}

#[test]
fn test_round_trip_code_blocks() {
    let original = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    // Check code block is preserved
    assert!(converted.contains("fn main()"));
    assert!(converted.contains("println!"));
}

#[test]
fn test_round_trip_lists() {
    let original = "## Lists\n\n- Item 1\n- Item 2\n  - Nested\n\n1. First\n2. Second";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    assert!(converted.contains("Item 1"));
    assert!(converted.contains("Item 2"));
    assert!(converted.contains("Nested"));
    assert!(converted.contains("First"));
    assert!(converted.contains("Second"));
}

#[test]
fn test_round_trip_links() {
    let original = "[Link text](https://example.com)";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    assert!(converted.contains("Link text"));
    assert!(converted.contains("https://example.com"));
}

#[test]
fn test_round_trip_tables() {
    let original = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    assert!(converted.contains("Header 1"));
    assert!(converted.contains("Header 2"));
    assert!(converted.contains("Cell 1"));
    assert!(converted.contains("Cell 2"));
}

#[test]
fn test_comprehensive_markdown_file() {
    // Load our comprehensive test file if it exists
    if let Ok(original) = fs::read_to_string("src/test_markdown.md") {
        let html = markdown_to_html(&original);
        let converted = html_to_markdown(&html).unwrap();
        
        // Check that key content is preserved (formatting may differ)
        assert!(converted.contains("Comprehensive Markdown Test Document"));
        assert!(converted.contains("**bold text**"));
        assert!(converted.contains("let x = 42;"));  // Code content preserved
        assert!(converted.contains("def hello():"));  // Python code preserved
        
        // Check headers are preserved (may be Setext style instead of ATX)
        assert!(converted.contains("Headers") || converted.contains("## Headers"));
        assert!(converted.contains("Text Formatting"));
        assert!(converted.contains("Code Blocks"));
        
        // Check lists preserved (may use * instead of -)
        assert!(converted.contains("First item"));
        assert!(converted.contains("Second item"));
        
        // Check links preserved
        assert!(converted.contains("https://example.com"));
        
        // Print for manual inspection
        println!("Original length: {}", original.len());
        println!("Converted length: {}", converted.len());
        
        // Check that we didn't lose too much content
        // Allow for some variation due to formatting differences
        assert!(converted.len() > original.len() / 2, 
                "Too much content lost in conversion");
    }
}

#[test]
fn test_round_trip_blockquotes() {
    let original = "> This is a quote\n> with multiple lines";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    assert!(converted.contains("This is a quote"));
    assert!(converted.contains("with multiple lines"));
}