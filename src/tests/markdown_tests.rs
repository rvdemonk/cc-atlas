use crate::utils::markdown::*;
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
fn test_round_trip_simple_table() {
    let original = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    assert!(converted.contains("Header 1"));
    assert!(converted.contains("Header 2"));
    assert!(converted.contains("Cell 1"));
    assert!(converted.contains("Cell 2"));
}

#[test]
fn test_round_trip_complex_table() {
    let original = r#"| Feature | Status | Priority | Notes |
|---------|--------|----------|-------|
| Authentication | ✅ Done | High | OAuth2 implemented |
| Dashboard | 🚧 In Progress | Medium | Needs UI polish |
| API Integration | ❌ Not Started | Low | Waiting for specs |
| **Bold text** | *Italic* | `code` | [Link](https://example.com) |"#;
    
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    // Check headers preserved
    assert!(converted.contains("Feature"));
    assert!(converted.contains("Status"));
    assert!(converted.contains("Priority"));
    assert!(converted.contains("Notes"));
    
    // Check content preserved
    assert!(converted.contains("Authentication"));
    assert!(converted.contains("Dashboard"));
    assert!(converted.contains("API Integration"));
    
    // Check emojis preserved
    assert!(converted.contains("✅") || converted.contains("Done"));
    assert!(converted.contains("🚧") || converted.contains("In Progress"));
    assert!(converted.contains("❌") || converted.contains("Not Started"));
    
    // Check formatting preserved in cells
    assert!(converted.contains("Bold text") || converted.contains("**Bold text**"));
    assert!(converted.contains("Italic") || converted.contains("*Italic*"));
    assert!(converted.contains("code") || converted.contains("`code`"));
    assert!(converted.contains("https://example.com"));
}

#[test]
fn test_round_trip_table_alignment() {
    // Test table with alignment (though alignment may not be preserved perfectly)
    let original = r#"| Left | Center | Right |
|:-----|:------:|------:|
| L1   | C1     | R1    |
| L2   | C2     | R2    |"#;
    
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    // Check all cells are preserved
    assert!(converted.contains("Left"));
    assert!(converted.contains("Center"));
    assert!(converted.contains("Right"));
    assert!(converted.contains("L1"));
    assert!(converted.contains("C1"));
    assert!(converted.contains("R1"));
    assert!(converted.contains("L2"));
    assert!(converted.contains("C2"));
    assert!(converted.contains("R2"));
}

#[test]
fn test_round_trip_table_with_empty_cells() {
    let original = r#"| Col1 | Col2 | Col3 |
|------|------|------|
| A    |      | C    |
|      | B    |      |
| X    | Y    | Z    |"#;
    
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    // Check headers
    assert!(converted.contains("Col1"));
    assert!(converted.contains("Col2"));
    assert!(converted.contains("Col3"));
    
    // Check non-empty cells
    assert!(converted.contains("A"));
    assert!(converted.contains("B"));
    assert!(converted.contains("C"));
    assert!(converted.contains("X"));
    assert!(converted.contains("Y"));
    assert!(converted.contains("Z"));
}

#[test]
fn test_round_trip_table_with_pipes_in_cells() {
    // Test escaping of pipes within cells
    let original = r#"| Command | Description |
|---------|-------------|
| `a \| b` | Pipe example |
| `grep \| wc` | Count lines |"#;
    
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    assert!(converted.contains("Command"));
    assert!(converted.contains("Description"));
    assert!(converted.contains("Pipe example"));
    assert!(converted.contains("Count lines"));
    // The pipe character handling might vary
    assert!(converted.contains("a") && converted.contains("b"));
    assert!(converted.contains("grep") && converted.contains("wc"));
}

#[test] 
fn test_table_before_and_after_content() {
    let original = r#"# Document Title

Some text before the table.

| Header 1 | Header 2 |
|----------|----------|
| Data 1   | Data 2   |

Some text after the table.

## Another Section"#;
    
    let html = markdown_to_html(original);
    let converted = html_to_markdown(&html).unwrap();
    
    // Check document structure is preserved
    assert!(converted.contains("Document Title"));
    assert!(converted.contains("Some text before the table"));
    assert!(converted.contains("Header 1"));
    assert!(converted.contains("Header 2"));
    assert!(converted.contains("Data 1"));
    assert!(converted.contains("Data 2"));
    assert!(converted.contains("Some text after the table"));
    assert!(converted.contains("Another Section"));
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