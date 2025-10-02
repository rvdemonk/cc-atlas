use pulldown_cmark::{html, Options, Parser};
use anyhow::Result;

/// Convert markdown text to HTML
pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(markdown, options);
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

/// Convert HTML back to markdown
pub fn html_to_markdown(html: &str) -> Result<String> {
    // Use html2md for conversion
    let markdown = html2md::parse_html(html);
    Ok(markdown)
}