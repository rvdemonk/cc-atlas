use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use crate::models::DocsNode;

const DOCS_DIR_NAME: &str = "docs";

/// Find the docs directory in the project root
pub fn find_docs_dir(root: &Path) -> Option<PathBuf> {
    let docs_path = root.join(DOCS_DIR_NAME);
    if docs_path.exists() && docs_path.is_dir() {
        Some(docs_path)
    } else {
        None
    }
}

/// Build a tree of documentation files and directories
pub fn build_docs_tree(docs_path: &Path) -> Result<DocsNode> {
    let name = docs_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "docs".to_string());

    let children = build_children(docs_path, docs_path)?;

    Ok(DocsNode {
        path: ".".to_string(), // Root of docs tree
        name,
        is_file: false,
        children,
    })
}

/// Build children nodes recursively
fn build_children(path: &Path, docs_root: &Path) -> Result<Vec<DocsNode>> {
    let mut children = Vec::new();

    if !path.is_dir() {
        return Ok(children);
    }

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    // Sort: directories first, then files, both alphabetically
    entries.sort_by_key(|e| {
        let is_dir = e.path().is_dir();
        let name = e.file_name().to_string_lossy().to_lowercase();
        (!is_dir, name)
    });

    for entry in entries {
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // Recursively build directory node
            let node = build_directory_node(&entry_path, docs_root)?;
            children.push(node);
        } else if is_markdown_file(&entry_path) {
            // Build file node for markdown files
            let node = build_file_node(&entry_path, docs_root)?;
            children.push(node);
        }
    }

    Ok(children)
}

/// Build a directory node
fn build_directory_node(path: &Path, docs_root: &Path) -> Result<DocsNode> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let relative_path = path
        .strip_prefix(docs_root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    let children = build_children(path, docs_root)?;

    Ok(DocsNode {
        path: relative_path,
        name,
        is_file: false,
        children,
    })
}

/// Build a file node
fn build_file_node(path: &Path, docs_root: &Path) -> Result<DocsNode> {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Remove .md extension for display
    let display_name = file_name
        .strip_suffix(".md")
        .unwrap_or(&file_name)
        .to_string();

    let relative_path = path
        .strip_prefix(docs_root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    Ok(DocsNode {
        path: relative_path,
        name: display_name,
        is_file: true,
        children: Vec::new(),
    })
}

/// Check if a file is a markdown file
fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md"))
        .unwrap_or(false)
}

/// Read a documentation file
pub fn read_doc_file(docs_root: &Path, relative_path: &str) -> Result<String> {
    let full_path = docs_root.join(relative_path);

    if !full_path.starts_with(docs_root) {
        anyhow::bail!("Path traversal attempt detected");
    }

    if !full_path.exists() {
        anyhow::bail!("File not found: {}", relative_path);
    }

    let content = fs::read_to_string(&full_path)?;
    Ok(content)
}

/// Write a documentation file
pub fn write_doc_file(docs_root: &Path, relative_path: &str, content: &str) -> Result<()> {
    let full_path = docs_root.join(relative_path);

    if !full_path.starts_with(docs_root) {
        anyhow::bail!("Path traversal attempt detected");
    }

    // Create parent directories if needed
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&full_path, content)?;
    Ok(())
}
