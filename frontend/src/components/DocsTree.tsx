import React, { useState } from 'react'
import { DocsNode, DocFile } from '../types'
import {
  HiChevronRight,
  HiChevronDown,
  HiDocumentText,
  HiFolder,
  HiFolderOpen,
  HiPlus
} from 'react-icons/hi'
import './Sidebar.css'

interface DocsTreeProps {
  tree: DocsNode | null
  selectedFile: DocFile | null
  onSelectFile: (path: string) => void
  onCreateDoc: (folderPath: string) => void
}

export const DocsTree: React.FC<DocsTreeProps> = ({
  tree,
  selectedFile,
  onSelectFile,
  onCreateDoc
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

  if (!tree) {
    return (
      <div className="sidebar-empty">
        <p>No docs/ directory found</p>
      </div>
    )
  }

  // Render children directly (skip the root "docs" node)
  return (
    <>
      <div className="docs-tree-header">
        <span className="docs-tree-title">Documents</span>
        <button
          className="docs-tree-add-btn"
          onClick={() => onCreateDoc('.')}
          title="New document in root"
        >
          <HiPlus />
        </button>
      </div>
      {tree.children.map(child => (
        <TreeNode
          key={child.path}
          node={child}
          selectedPath={selectedFile?.path || null}
          onSelectFile={onSelectFile}
          onCreateDoc={onCreateDoc}
          expandedPaths={expandedPaths}
          onToggleExpanded={toggleExpanded}
          level={0}
        />
      ))}
    </>
  )
}

interface TreeNodeProps {
  node: DocsNode
  selectedPath: string | null
  onSelectFile: (path: string) => void
  onCreateDoc: (folderPath: string) => void
  expandedPaths: Set<string>
  onToggleExpanded: (path: string) => void
  level: number
}

const TreeNode: React.FC<TreeNodeProps> = ({
  node,
  selectedPath,
  onSelectFile,
  onCreateDoc,
  expandedPaths,
  onToggleExpanded,
  level
}) => {
  const [isHovered, setIsHovered] = useState(false)
  const isExpanded = expandedPaths.has(node.path)
  const hasChildren = node.children.length > 0
  const isSelected = selectedPath === node.path

  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation()

    if (node.is_file) {
      // File clicked - select it
      onSelectFile(node.path)
    } else {
      // Directory clicked - toggle expansion
      if (hasChildren) {
        onToggleExpanded(node.path)
      }
    }
  }

  const handleChevronClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (hasChildren && !node.is_file) {
      onToggleExpanded(node.path)
    }
  }

  const handleCreateDoc = (e: React.MouseEvent) => {
    e.stopPropagation()
    onCreateDoc(node.path)
  }

  return (
    <div className="tree-node" style={{ '--level': level } as React.CSSProperties}>
      <div
        className={`tree-node-header ${isSelected ? 'selected' : ''}`}
        onClick={handleClick}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
      >
        <span className="tree-node-indent" />

        {!node.is_file && hasChildren && (
          <button className="tree-node-chevron" onClick={handleChevronClick}>
            {isExpanded ? <HiChevronDown /> : <HiChevronRight />}
          </button>
        )}
        {(node.is_file || !hasChildren) && <span className="tree-node-spacer" />}

        <span className="tree-node-icon">
          {node.is_file ? (
            <HiDocumentText />
          ) : (
            isExpanded ? <HiFolderOpen /> : <HiFolder />
          )}
        </span>

        <span className="tree-node-name">
          {node.name}
        </span>

        {!node.is_file && isHovered && (
          <button
            className="tree-node-add"
            onClick={handleCreateDoc}
            title="New document"
          >
            <HiPlus />
          </button>
        )}
      </div>

      {!node.is_file && isExpanded && hasChildren && (
        <div className="tree-node-children">
          {node.children.map(child => (
            <TreeNode
              key={child.path}
              node={child}
              selectedPath={selectedPath}
              onSelectFile={onSelectFile}
              onCreateDoc={onCreateDoc}
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
