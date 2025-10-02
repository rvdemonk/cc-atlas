import React from 'react'
import { SidebarProps } from '../types'
import { HiDocumentText, HiPencil } from 'react-icons/hi'
import { MemoryTree } from './MemoryTree'
import { DocsTree } from './DocsTree'
import './Sidebar.css'

export const Sidebar: React.FC<SidebarProps> = ({
  tree,
  memoryFiles,
  recommendations,
  selectedFile,
  onSelectFile,
  collapsed,
  viewMode,
  onViewModeChange,
  docsTree,
  selectedDocFile,
  onDocFileSelect,
  onCreateDoc
}) => {

  if (!tree) return null

  return (
    <aside className={`sidebar ${collapsed ? 'sidebar-collapsed' : ''}`}>
      <div className="sidebar-header">
        <button
          className={`view-mode-toggle ${viewMode === 'memory' ? 'active' : ''}`}
          onClick={() => onViewModeChange('memory')}
        >
          <HiPencil />
          <span>Memory</span>
        </button>
        <button
          className={`view-mode-toggle ${viewMode === 'docs' ? 'active' : ''}`}
          onClick={() => onViewModeChange('docs')}
        >
          <HiDocumentText />
          <span>Docs</span>
        </button>
      </div>

      <div className="sidebar-content">
        {viewMode === 'memory' ? (
          <MemoryTree
            tree={tree}
            memoryFiles={memoryFiles}
            recommendations={recommendations}
            selectedFile={selectedFile}
            onSelectFile={onSelectFile}
          />
        ) : (
          <DocsTree
            tree={docsTree}
            selectedFile={selectedDocFile}
            onSelectFile={onDocFileSelect}
            onCreateDoc={onCreateDoc}
          />
        )}
      </div>
    </aside>
  )
}