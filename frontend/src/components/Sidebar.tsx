import React, { useState, useMemo } from 'react'
import { SidebarProps, TreeNodeProps, MemoryFile } from '../types'
import { 
  HiChevronRight, 
  HiChevronDown, 
  HiDocumentText,
  HiFolder,
  HiFolderOpen,
  HiLightBulb,
  HiPencil
} from 'react-icons/hi'
import './Sidebar.css'

export const Sidebar: React.FC<SidebarProps> = ({
  tree,
  memoryFiles,
  recommendations,
  selectedFile,
  onSelectFile,
  collapsed
}) => {
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(() => {
    // Start with root expanded
    return new Set(tree ? [tree.path] : [])
  })
  
  const toggleExpanded = (path: string) => {
    setExpandedPaths(prev => {
      const next = new Set(prev)
      if (next.has(path)) {
        next.delete(path)
      } else {
        next.add(path)
      }
      return next
    })
  }
  
  if (!tree) return null
  
  return (
    <aside className={`sidebar ${collapsed ? 'sidebar-collapsed' : ''}`}>
      <div className="sidebar-content">
        <TreeNode
          node={tree}
          memoryFiles={memoryFiles}
          recommendations={recommendations}
          selectedPath={selectedFile?.path || null}
          onSelectFile={onSelectFile}
          expandedPaths={expandedPaths}
          onToggleExpanded={toggleExpanded}
          level={0}
        />
      </div>
    </aside>
  )
}

interface TreeNodePropsExtended extends TreeNodeProps {
  expandedPaths: Set<string>
  onToggleExpanded: (path: string) => void
}

const TreeNode: React.FC<TreeNodePropsExtended> = ({
  node,
  memoryFiles,
  recommendations,
  selectedPath,
  onSelectFile,
  expandedPaths,
  onToggleExpanded,
  level
}) => {
  const memoryFile = useMemo(() => {
    // Normalize the node path for comparison
    const normalizedNodePath = node.path.replace(/^\.\//, '').replace(/\/$/, '') || '.'
    
    // Find memory file that belongs to this directory
    const found = memoryFiles.find(f => {
      // Normalize the parent path for comparison
      const normalizedParentPath = f.parent_path.replace(/^\.\//, '').replace(/\/$/, '') || '.'
      
      // Check if this memory file belongs to the current node
      if (normalizedNodePath === '.' || normalizedNodePath === '') {
        // Root directory case
        return normalizedParentPath === '.' || normalizedParentPath === ''
      }
      
      return normalizedParentPath === normalizedNodePath
    })
    
    if (found) {
      console.log('Found existing memory file for', node.path, ':', found)
      return found
    }
    
    // Fallback: try to match by constructing the expected path
    const expectedPath = normalizedNodePath === '.' ? 'CLAUDE.md' : `${normalizedNodePath}/CLAUDE.md`
    const directMatch = memoryFiles.find(f => {
      const normalizedFilePath = f.path.replace(/^\.\//, '')
      return normalizedFilePath === expectedPath
    })
    
    if (directMatch) {
      console.log('Found direct match for', node.path, ':', directMatch)
      return directMatch
    }
    
    return null
  }, [memoryFiles, node.path])
  
  const isExpanded = expandedPaths.has(node.path)
  const hasChildren = node.children.length > 0
  const isRecommended = recommendations.includes(node.path)
  const isSelected = selectedPath === memoryFile?.path
  
  const handleToggle = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (hasChildren) {
      onToggleExpanded(node.path)
    }
  }
  
  const handleNodeClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    // If clicking on the folder icon or chevron area, toggle expand/collapse
    const target = e.target as HTMLElement
    if (target.closest('.tree-node-chevron') || target.closest('.tree-node-icon')) {
      if (hasChildren) {
        onToggleExpanded(node.path)
      }
      return
    }
    
    // Clicking on the name or anywhere else in the header
    // First priority: open memory file if it exists
    if (memoryFile) {
      console.log('Opening existing memory file for:', node.path, memoryFile)
      onSelectFile(memoryFile)
    } else {
      // Create a virtual memory file for any directory without one
      // This will open a blank editor that will create the file on first save
      // Normalize the path construction
      const normalizedPath = node.path.replace(/^\.\//, '').replace(/\/$/, '') || '.'
      const newPath = normalizedPath === '.' ? 'CLAUDE.md' : `${normalizedPath}/CLAUDE.md`
      
      const newMemoryFile: MemoryFile = {
        path: newPath,
        content: '',
        content_html: '',
        exists: false,
        parent_path: normalizedPath
      }
      console.log('Opening blank editor for new memory file:', node.path, newMemoryFile)
      onSelectFile(newMemoryFile)
    }
  }
  
  const handleMemoryClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (memoryFile) {
      onSelectFile(memoryFile)
    }
  }
  
  return (
    <div className="tree-node" style={{ '--level': level } as React.CSSProperties}>
      <div 
        className={`tree-node-header ${isSelected ? 'selected' : ''}`}
        onClick={handleNodeClick}
      >
        <span className="tree-node-indent" />
        
        {hasChildren && (
          <button className="tree-node-chevron" onClick={handleToggle}>
            {isExpanded ? <HiChevronDown /> : <HiChevronRight />}
          </button>
        )}
        {!hasChildren && <span className="tree-node-spacer" />}
        
        <span className="tree-node-icon" onClick={handleToggle}>
          {isExpanded ? <HiFolderOpen /> : <HiFolder />}
        </span>
        
        <span className="tree-node-name">
          {node.name}
        </span>
        
        <div className="tree-node-badges">
          {isRecommended && (
            <span className="badge badge-recommend" title="Recommended for memory file">
              <HiLightBulb />
            </span>
          )}
          
          {node.has_memory && memoryFile && (
            <button 
              className={`badge badge-memory ${isSelected ? 'active' : ''}`}
              onClick={handleMemoryClick}
              title="Edit CLAUDE.md"
            >
              <HiPencil />
            </button>
          )}
          
          {!node.has_memory && memoryFile && !memoryFile.exists && (
            <button 
              className={`badge badge-memory-new ${isSelected ? 'active' : ''}`}
              onClick={handleMemoryClick}
              title="Create CLAUDE.md"
            >
              <HiDocumentText />
            </button>
          )}
        </div>
      </div>
      
      {isExpanded && hasChildren && (
        <div className="tree-node-children">
          {node.children.map(child => (
            <TreeNode
              key={child.path}
              node={child}
              memoryFiles={memoryFiles}
              recommendations={recommendations}
              selectedPath={selectedPath}
              onSelectFile={onSelectFile}
              expandedPaths={expandedPaths}
              onToggleExpanded={onToggleExpanded}
              level={level + 1}
            />
          ))}
        </div>
      )}
    </div>
  )
}