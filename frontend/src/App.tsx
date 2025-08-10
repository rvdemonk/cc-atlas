import { useState, useEffect, useCallback } from 'react'
import './App.css'
import { Sidebar } from './components/Sidebar'
import { Editor } from './components/Editor'
import { Header } from './components/Header'
import * as api from './api/client'
import { deleteMemoryFile } from './api/client'
import { MemoryFile, AppState } from './types'

function App() {
  const [state, setState] = useState<AppState>({
    tree: null,
    memoryFiles: [],
    recommendations: [],
    selectedFile: null,
    loading: true,
    error: null,
    sidebarCollapsed: false
  })

  // Load initial data
  useEffect(() => {
    loadData()
  }, [])

  const loadData = async () => {
    try {
      setState(prev => ({ ...prev, loading: true, error: null }))
      
      const [tree, memoryFiles, recommendations] = await Promise.all([
        api.fetchTree(),
        api.fetchMemoryFiles(),
        api.fetchRecommendations()
      ])
      
      console.log('Loaded memory files:', memoryFiles)
      
      setState(prev => ({
        ...prev,
        tree,
        memoryFiles,
        recommendations,
        loading: false
      }))
    } catch (error) {
      setState(prev => ({
        ...prev,
        error: error instanceof Error ? error.message : 'Failed to load data',
        loading: false
      }))
    }
  }

  const selectFile = useCallback((file: MemoryFile | null) => {
    console.log('Selecting file:', file)
    // If selecting a file, get the latest version from memoryFiles
    if (file) {
      setState(prev => {
        const latestFile = prev.memoryFiles.find(f => f.path === file.path)
        return { ...prev, selectedFile: latestFile || file }
      })
    } else {
      setState(prev => ({ ...prev, selectedFile: null }))
    }
  }, [])

  const updateFile = useCallback(async (
    path: string, 
    content: string, 
    isHtml: boolean = false
  ) => {
    try {
      // Check if the file exists on disk (exists property is true)
      const existingFile = state.memoryFiles.find(f => f.path === path)
      const fileExistsOnDisk = existingFile?.exists === true
      
      let result;
      let wasCreated = false;
      
      if (fileExistsOnDisk) {
        // Update existing file
        result = await api.updateMemoryFile(path, content, isHtml)
      } else {
        // Create new file - only when there's actual content
        if (!content || content.trim().length === 0) {
          // Don't create empty files
          return { success: true, content: '' }
        }
        result = await api.createMemoryFile(path, content, isHtml)
        wasCreated = result.created === true
      }
      
      // Update local state with the returned markdown content
      const updatedContent = result.content
      
      // Reload tree and memory files if this was a new file creation
      if (wasCreated) {
        const [tree, memoryFiles] = await Promise.all([
          api.fetchTree(),
          api.fetchMemoryFiles()
        ])
        
        // Update everything in one setState call
        setState(prev => {
          // Find the newly created file in the fetched memoryFiles
          const newFile = memoryFiles.find(f => f.path === path)
          
          return {
            ...prev,
            tree,
            memoryFiles,
            // Update selectedFile if it's the one we just created
            selectedFile: prev.selectedFile?.path === path && newFile
              ? newFile
              : prev.selectedFile
          }
        })
      } else {
        // For existing files OR if creation returned 409 (file already exists)
        // Update the file to mark it as existing
        setState(prev => {
          const fileIndex = prev.memoryFiles.findIndex(file => file.path === path)
          
          if (fileIndex >= 0) {
            const updatedFiles = [...prev.memoryFiles]
            updatedFiles[fileIndex] = {
              ...updatedFiles[fileIndex],
              content: updatedContent,
              content_html: isHtml ? content : '',
              exists: true
            }
            return { ...prev, memoryFiles: updatedFiles }
          } else if (!fileExistsOnDisk) {
            // File was created but returned 409, need to add it to the list
            const newFile: MemoryFile = {
              path,
              content: updatedContent,
              content_html: isHtml ? content : '',
              exists: true,
              parent_path: path.substring(0, path.lastIndexOf('/'))
            }
            return { 
              ...prev, 
              memoryFiles: [...prev.memoryFiles, newFile],
              // Also update selectedFile if it's the one we're saving
              selectedFile: prev.selectedFile?.path === path 
                ? { ...newFile }
                : prev.selectedFile
            }
          }
          
          return prev
        })
      }
      
      return { success: true, content: updatedContent }
    } catch (error) {
      return { 
        success: false, 
        error: error instanceof Error ? error.message : 'Failed to save'
      }
    }
  }, [state.memoryFiles, state.selectedFile])

  const toggleSidebar = useCallback(() => {
    setState(prev => ({ ...prev, sidebarCollapsed: !prev.sidebarCollapsed }))
  }, [])
  
  const deleteFile = useCallback(async (path: string): Promise<boolean> => {
    try {
      const result = await deleteMemoryFile(path)
      
      if (result.deleted) {
        // Reload tree and memory files after deletion
        const [tree, memoryFiles] = await Promise.all([
          api.fetchTree(),
          api.fetchMemoryFiles()
        ])
        
        // Clear selected file if it was the deleted one
        setState(prev => ({
          ...prev,
          tree,
          memoryFiles,
          selectedFile: prev.selectedFile?.path === path ? null : prev.selectedFile
        }))
        
        return true
      }
      return false
    } catch (error) {
      console.error('Failed to delete file:', error)
      return false
    }
  }, [])

  if (state.loading) {
    return (
      <div className="app-loading">
        <div className="loading-spinner" />
        <p>Loading repository structure...</p>
      </div>
    )
  }

  if (state.error) {
    return (
      <div className="app-error">
        <h2>Error loading application</h2>
        <p>{state.error}</p>
        <button onClick={loadData}>Retry</button>
      </div>
    )
  }

  return (
    <div className="app">
      <Header 
        onToggleSidebar={toggleSidebar}
        projectName={state.tree?.name || 'cc-atlas'}
      />
      
      <div className="app-body">
        <Sidebar
          tree={state.tree}
          memoryFiles={state.memoryFiles}
          recommendations={state.recommendations}
          selectedFile={state.selectedFile}
          onSelectFile={selectFile}
          collapsed={state.sidebarCollapsed}
        />
        
        <Editor
          file={state.selectedFile}
          onSave={updateFile}
          onDelete={deleteFile}
        />
      </div>
    </div>
  )
}

export default App