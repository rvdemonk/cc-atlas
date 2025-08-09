import React, { useState, useEffect } from 'react'
import TreeView from './components/TreeView'
import Editor from './components/Editor'
import { fetchTree, fetchMemoryFiles, updateMemoryFile } from './api/client'
import './App.css'

export interface DirectoryInfo {
  path: string
  name: string
  has_memory: boolean
  children: DirectoryInfo[]
  stats: {
    file_count: number
    total_lines: number
    depth: number
  }
}

export interface MemoryFile {
  path: string
  content: string
  relative_path: string
  stats: {
    file_count: number
    total_lines: number
    depth: number
  }
}

function App() {
  const [tree, setTree] = useState<DirectoryInfo | null>(null)
  const [memoryFiles, setMemoryFiles] = useState<MemoryFile[]>([])
  const [selectedFile, setSelectedFile] = useState<MemoryFile | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadData()
  }, [])

  const loadData = async () => {
    try {
      const [treeData, filesData] = await Promise.all([
        fetchTree(),
        fetchMemoryFiles()
      ])
      setTree(treeData)
      setMemoryFiles(filesData)
    } catch (error) {
      console.error('Failed to load data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSave = async (content: string) => {
    if (!selectedFile) return
    
    try {
      await updateMemoryFile(selectedFile.relative_path, content)
      setSelectedFile({ ...selectedFile, content })
      setMemoryFiles(prev => 
        prev.map(f => f.path === selectedFile.path 
          ? { ...f, content } 
          : f
        )
      )
    } catch (error) {
      console.error('Failed to save:', error)
    }
  }

  const handleSelectPath = (path: string) => {
    const file = memoryFiles.find(f => f.path === path)
    if (file) {
      setSelectedFile(file)
    }
  }

  if (loading) {
    return <div className="loading">Loading...</div>
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>cc-atlas</h1>
        <div className="stats">
          Memory Files: {memoryFiles.length}
        </div>
      </header>
      
      <div className="app-body">
        <aside className="sidebar">
          {tree && (
            <TreeView 
              tree={tree} 
              onSelectPath={handleSelectPath}
              selectedPath={selectedFile?.path}
            />
          )}
        </aside>
        
        <main className="main">
          {selectedFile ? (
            <Editor 
              file={selectedFile}
              onSave={handleSave}
            />
          ) : (
            <div className="no-selection">
              Select a CLAUDE.md file to edit
            </div>
          )}
        </main>
      </div>
    </div>
  )
}

export default App