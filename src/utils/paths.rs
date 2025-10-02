use std::path::{Path, PathBuf};
use crate::models::{DirectoryInfo, MemoryFile, MemoryFileResponse};
use super::markdown;

pub fn convert_to_responses(files: Vec<MemoryFile>, project_root: &str) -> Vec<MemoryFileResponse> {
    files
        .into_iter()
        .map(|file| {
            let html = markdown::markdown_to_html(&file.content);
            let parent_path = file
                .path
                .parent()
                .and_then(|p| p.strip_prefix(project_root).ok())
                .map(|p| format!("./{}", p.display()))
                .unwrap_or_else(|| ".".to_string());

            MemoryFileResponse {
                path: file.relative_path.clone(),
                content: file.content,
                content_html: html,
                exists: file.path.exists(),
                parent_path,
            }
        })
        .collect()
}

pub fn convert_tree_paths(mut tree: DirectoryInfo, project_root: &str) -> DirectoryInfo {
    let root_path = Path::new(project_root);

    // Convert the tree path to relative
    tree.path = tree
        .path
        .strip_prefix(root_path)
        .map(|p| {
            if p.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                PathBuf::from(format!("./{}", p.display()))
            }
        })
        .unwrap_or_else(|_| PathBuf::from("."));

    // Recursively convert children
    tree.children = tree
        .children
        .into_iter()
        .map(|child| convert_tree_paths(child, project_root))
        .collect();

    tree
}

pub fn to_relative_paths(paths: Vec<PathBuf>, project_root: &str) -> Vec<String> {
    paths
        .into_iter()
        .filter_map(|path| {
            path.strip_prefix(project_root)
                .ok()
                .map(|p| format!("./{}", p.display()))
        })
        .collect()
}
