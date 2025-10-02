// Core data models
export interface DirectoryInfo {
  name: string
  path: string
  has_memory: boolean
  should_recommend: boolean
  file_count: number
  total_lines: number
  children: DirectoryInfo[]
}

export interface MemoryFile {
  path: string
  content: string
  content_html: string
  exists: boolean
  parent_path: string
}

// Application state
export interface AppState {
  tree: DirectoryInfo | null
  memoryFiles: MemoryFile[]
  recommendations: string[]
  selectedFile: MemoryFile | null
  loading: boolean
  error: string | null
  sidebarCollapsed: boolean
}

// Component props
export interface HeaderProps {
  onToggleSidebar: () => void
  projectName: string
}

export interface SidebarProps {
  tree: DirectoryInfo | null
  memoryFiles: MemoryFile[]
  recommendations: string[]
  selectedFile: MemoryFile | null
  onSelectFile: (file: MemoryFile | null) => void
  collapsed: boolean
  viewMode: 'memory' | 'docs'
  onViewModeChange: (mode: 'memory' | 'docs') => void
}

export interface EditorProps {
  file: MemoryFile | null
  onSave: (path: string, content: string, isHtml?: boolean) => Promise<{ success: boolean; error?: string; content?: string }>
  onDelete?: (path: string) => Promise<boolean>
}

export interface TreeNodeProps {
  node: DirectoryInfo
  memoryFiles: MemoryFile[]
  recommendations: string[]
  selectedPath: string | null
  onSelectFile: (file: MemoryFile | null) => void
  level: number
}

// Editor state
export interface EditorState {
  content: string
  saving: boolean
  lastSaved: Date | null
  hasChanges: boolean
  mode: 'wysiwyg' | 'source'
}

// Docs types
export interface DocsNode {
  path: string
  name: string
  is_file: boolean
  children: DocsNode[]
}

export interface DocFile {
  path: string
  content: string
  content_html: string
}