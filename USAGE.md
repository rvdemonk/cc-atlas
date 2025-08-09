# cc-atlas Usage Guide

## Installation

### One-time Setup
```bash
# Clone cc-atlas
git clone <your-repo>/cc-atlas.git
cd cc-atlas

# Run the installation script
./scripts/install.sh
```

This installs cc-atlas globally, so you can use it from any project directory.

## Using cc-atlas with Your Projects

### Method 1: Global Command (After Installation)

```bash
# Navigate to any project that uses Claude Code
cd ~/projects/my-claude-project

# Start cc-atlas for the current directory
cc-atlas

# Or analyze a different project from anywhere
cc-atlas ~/projects/another-project
```

### Method 2: Direct Usage (Without Installation)

```bash
# From the cc-atlas directory
cd ~/code/cc-atlas

# Analyze current directory
./cc-atlas

# Analyze a specific project
./cc-atlas --project ~/projects/my-app

# Use a different port
./cc-atlas --project ~/projects/my-app --port 4000

# Or use scripts directly
./scripts/start.sh --project ~/projects/my-app
./scripts/dev.sh  # For development with hot reload
```

### Method 3: Shell Alias (Recommended)

Add this to your `~/.zshrc` or `~/.bashrc`:

```bash
# cc-atlas alias
alias cca='~/code/cc-atlas/scripts/start.sh --project'

# Usage:
# cca .                    # Current directory
# cca ~/projects/my-app    # Specific project
```

## Workflow Examples

### Example 1: Existing Project with Claude Code

```bash
# You have a project with some CLAUDE.md files
cd ~/projects/pum-ios

# Start cc-atlas to see all memory files and get recommendations
cc-atlas

# Browser opens at http://localhost:3000
# You can:
# - See all existing CLAUDE.md files in the tree view
# - Click on any to edit them
# - See 💡 icons for recommended locations
# - Save with Ctrl+S
```

### Example 2: New Project Starting with Claude Code

```bash
# Create a new project
mkdir my-new-app
cd my-new-app

# Start cc-atlas
cc-atlas

# Initially no CLAUDE.md files will be found
# As you add code, cc-atlas will recommend where to add memory files
# Create them manually or through the web UI
```

### Example 3: Multiple Projects

```bash
# Terminal 1: Work project
cc-atlas ~/work/backend-api

# Terminal 2: Personal project (different port)
cd ~/code/cc-atlas
./scripts/start.sh --project ~/personal/game-engine --port 4000
```

## Features Guide

### Tree View Icons

- 📝 = Directory has a CLAUDE.md file
- 💡 = Recommended location for a new memory file
- 📁/📂 = Regular directories
- **M badge** = Has memory file
- **R badge** = Recommended for memory file

### Recommendations

cc-atlas recommends adding CLAUDE.md files when a directory has:
- More than 10 files
- More than 500 lines of code

### Editing

1. Click any 📝 directory in the tree
2. Edit the content in the editor
3. Save with Ctrl+S or the Save button
4. Orange dot (●) indicates unsaved changes

## Tips

1. **Quick Check**: Run `cc-atlas` in any project to quickly see which directories have memory files and which need them.

2. **Before Claude Code Sessions**: Start cc-atlas first to review and update any stale memory files.

3. **Project Switching**: Keep cc-atlas running while you work. It auto-refreshes when you make changes.

4. **Creating New Memory Files**: Currently, create them manually:
   ```bash
   echo "# Component Architecture\n\nThis directory contains..." > src/components/CLAUDE.md
   ```
   Then refresh cc-atlas to see and edit it.

## Troubleshooting

### Port Already in Use
```bash
# Use a different port
./scripts/start.sh --project . --port 4000
```

### Can't Find Project
```bash
# Use absolute path
cc-atlas /Users/lewis/projects/my-app

# Or cd to the directory first
cd ~/projects/my-app
cc-atlas
```

### Permission Denied
```bash
# Make sure scripts are executable
chmod +x ~/code/cc-atlas/cc-atlas
chmod +x ~/code/cc-atlas/scripts/*.sh
```

## Next Steps

After MVP, we plan to add:
- Staleness detection (based on git commits)
- In-browser creation of new CLAUDE.md files
- Bulk operations
- Export/import templates
- Integration with Claude Code CLI directly