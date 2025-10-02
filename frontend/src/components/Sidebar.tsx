import React, { useState, useEffect } from 'react'
import { SidebarProps, DocsNode, DocFile } from '../types'
import { HiDocumentText, HiPencil } from 'react-icons/hi'
import { MemoryTree } from './MemoryTree'
import { DocsTree } from './DocsTree'
import { fetchDocsTree, fetchDocFile } from '../api/client'
import './Sidebar.css'

export const Sidebar: React.FC<SidebarProps> = ({
  tree,
  memoryFiles,
  recommendations,
  selectedFile,
  onSelectFile,
  collapsed,
  viewMode,
  onViewModeChange
}) => {
  const [docsTree, setDocsTree] = useState<DocsNode | null>(null)
  const [selectedDocFile, setSelectedDocFile] = useState<DocFile | null>(null)
  const [loadingDocs, setLoadingDocs] = useState(false)

  // Load docs tree when switching to docs view
  useEffect(() => {
    if (viewMode === 'docs' && !docsTree && !loadingDocs) {
      setLoadingDocs(true)
      fetchDocsTree()
        .then(tree => {
          setDocsTree(tree)
          setLoadingDocs(false)
        })
        .catch(err => {
          console.error('Failed to load docs tree:', err)
          setLoadingDocs(false)
        })
    }
  }, [viewMode, docsTree, loadingDocs])

  const handleDocFileSelect = async (path: string) => {
    try {
      const docFile = await fetchDocFile(path)
      setSelectedDocFile(docFile)
    } catch (err) {
      console.error('Failed to load doc file:', err)
    }
  }

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
        ) : loadingDocs ? (
          <div className="sidebar-empty">
            <p>Loading docs...</p>
          </div>
        ) : (
          <DocsTree
            tree={docsTree}
            selectedFile={selectedDocFile}
            onSelectFile={handleDocFileSelect}
          />
        )}
      </div>
    </aside>
  )
}