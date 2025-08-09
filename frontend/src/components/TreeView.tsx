import React, { useState } from 'react'
import { DirectoryInfo } from '../App'
import './TreeView.css'

interface TreeViewProps {
  tree: DirectoryInfo
  onSelectPath: (path: string) => void
  selectedPath?: string
}

const TreeView: React.FC<TreeViewProps> = ({ tree, onSelectPath, selectedPath }) => {
  return (
    <div className="tree-view">
      <TreeNode 
        node={tree} 
        onSelectPath={onSelectPath}
        selectedPath={selectedPath}
        level={0}
      />
    </div>
  )
}

interface TreeNodeProps {
  node: DirectoryInfo
  onSelectPath: (path: string) => void
  selectedPath?: string
  level: number
}

const TreeNode: React.FC<TreeNodeProps> = ({ node, onSelectPath, selectedPath, level }) => {
  const [expanded, setExpanded] = useState(level < 2)
  const hasChildren = node.children.length > 0
  const isSelected = node.path === selectedPath
  
  const handleClick = () => {
    if (node.has_memory) {
      onSelectPath(node.path + '/CLAUDE.md')
    }
    if (hasChildren) {
      setExpanded(!expanded)
    }
  }
  
  const getIcon = () => {
    if (node.has_memory) return '📝'
    if (shouldRecommend(node)) return '💡'
    return hasChildren ? (expanded ? '📂' : '📁') : '📄'
  }
  
  const shouldRecommend = (node: DirectoryInfo) => {
    return !node.has_memory && 
           (node.stats.file_count > 10 || node.stats.total_lines > 500)
  }
  
  return (
    <div className="tree-node">
      <div 
        className={`tree-node-header ${isSelected ? 'selected' : ''}`}
        onClick={handleClick}
        style={{ paddingLeft: `${level * 20 + 8}px` }}
      >
        <span className="tree-icon">{getIcon()}</span>
        <span className="tree-name">{node.name || 'root'}</span>
        {node.has_memory && (
          <span className="badge memory">M</span>
        )}
        {shouldRecommend(node) && (
          <span className="badge recommend">R</span>
        )}
      </div>
      
      {expanded && hasChildren && (
        <div className="tree-children">
          {node.children.map((child, index) => (
            <TreeNode
              key={index}
              node={child}
              onSelectPath={onSelectPath}
              selectedPath={selectedPath}
              level={level + 1}
            />
          ))}
        </div>
      )}
    </div>
  )
}

export default TreeView