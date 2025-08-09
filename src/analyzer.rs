use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::models::{DirectoryInfo, FileStats, MemoryFile};

const MEMORY_FILE_NAME: &str = "CLAUDE.md";
const DEFAULT_MAX_DEPTH: usize = 3;
const COMPLEXITY_FILE_THRESHOLD: usize = 10;
const COMPLEXITY_LINE_THRESHOLD: usize = 500;

pub fn analyze_project(path: &str) -> Result<()> {
    let project_path = Path::new(path);
    let memory_files = find_memory_files(project_path)?;
    
    print_memory_files(&memory_files);
    
    let tree = build_directory_tree(project_path)?;
    let recommendations = get_recommendations(&tree);
    
    print_recommendations(&recommendations);
    
    Ok(())
}

fn print_memory_files(files: &[MemoryFile]) {
    println!("Found {} memory files:", files.len());
    for file in files {
        println!("  - {}", file.relative_path);
    }
}

fn print_recommendations(recommendations: &[PathBuf]) {
    if !recommendations.is_empty() {
        println!("\nRecommended locations for new memory files:");
        for rec in recommendations {
            println!("  - {}", rec.display());
        }
    }
}

pub fn find_memory_files(root: &Path) -> Result<Vec<MemoryFile>> {
    let mut memory_files = Vec::new();
    
    for entry in walk_directory(root) {
        let entry = entry?;
        if is_memory_file(&entry) {
            let memory_file = create_memory_file(&entry, root)?;
            memory_files.push(memory_file);
        }
    }
    
    Ok(memory_files)
}

fn walk_directory(root: &Path) -> impl Iterator<Item = walkdir::Result<walkdir::DirEntry>> {
    WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_ignored(e.path()))
}

fn is_memory_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file() && entry.file_name() == MEMORY_FILE_NAME
}

fn create_memory_file(entry: &walkdir::DirEntry, root: &Path) -> Result<MemoryFile> {
    let path = entry.path();
    let content = fs::read_to_string(path)?;
    let relative_path = get_relative_path(path, root);
    let parent_dir = path.parent().unwrap_or(root);
    let stats = calculate_stats(parent_dir)?;
    
    Ok(MemoryFile {
        path: path.to_path_buf(),
        content,
        relative_path,
        stats,
    })
}

fn get_relative_path(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

pub fn build_directory_tree(root: &Path) -> Result<DirectoryInfo> {
    let name = get_directory_name(root);
    let has_memory = check_has_memory(root);
    let stats = calculate_stats(root)?;
    let children = build_children(root)?;
    
    Ok(DirectoryInfo {
        path: root.to_path_buf(),
        name,
        has_memory,
        children,
        stats,
    })
}

fn get_directory_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn check_has_memory(path: &Path) -> bool {
    path.join(MEMORY_FILE_NAME).exists()
}

fn build_children(root: &Path) -> Result<Vec<DirectoryInfo>> {
    let mut children = Vec::new();
    
    if !root.is_dir() {
        return Ok(children);
    }
    
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        
        if should_process_directory(&path) {
            if let Ok(child) = build_directory_tree(&path) {
                children.push(child);
            }
        }
    }
    
    Ok(children)
}

fn should_process_directory(path: &Path) -> bool {
    path.is_dir() && !is_ignored(path)
}

fn calculate_stats(path: &Path) -> Result<FileStats> {
    let mut file_count = 0;
    let mut total_lines = 0;
    
    for entry in walk_limited(path, DEFAULT_MAX_DEPTH) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                file_count += 1;
                total_lines += count_lines(entry.path());
            }
        }
    }
    
    Ok(FileStats {
        file_count,
        total_lines,
        depth: count_depth(path),
    })
}

fn walk_limited(path: &Path, max_depth: usize) -> impl Iterator<Item = walkdir::Result<walkdir::DirEntry>> {
    WalkDir::new(path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| !is_ignored(e.path()))
}

fn count_lines(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|content| content.lines().count())
        .unwrap_or(0)
}

fn count_depth(path: &Path) -> usize {
    path.components().count()
}

fn get_recommendations(tree: &DirectoryInfo) -> Vec<PathBuf> {
    let mut recommendations = Vec::new();
    collect_recommendations(tree, &mut recommendations);
    recommendations
}

fn collect_recommendations(dir: &DirectoryInfo, recommendations: &mut Vec<PathBuf>) {
    if should_recommend_memory(dir) {
        recommendations.push(dir.path.clone());
    }
    
    for child in &dir.children {
        collect_recommendations(child, recommendations);
    }
}

fn should_recommend_memory(dir: &DirectoryInfo) -> bool {
    !dir.has_memory && should_have_memory(&dir.stats)
}

fn should_have_memory(stats: &FileStats) -> bool {
    stats.file_count > COMPLEXITY_FILE_THRESHOLD || 
    stats.total_lines > COMPLEXITY_LINE_THRESHOLD
}

fn is_ignored(path: &Path) -> bool {
    const IGNORED_DIRS: &[&str] = &[
        ".git", "node_modules", "target", "dist", "build",
        ".next", ".cache", "coverage", "__pycache__"
    ];
    
    path.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();
        IGNORED_DIRS.iter().any(|&ignored| name == ignored)
    })
}