import { useState, useEffect, useCallback } from 'react'
import './App.css'
import { Sidebar } from './components/Sidebar'
import { Editor } from './components/Editor'
import { Header } from './components/Header'
import * as api from './api/client'
import { deleteMemoryFile, fetchDocFile } from './api/client'
import { MemoryFile, AppState, DocFile, EditableFile, DocsNode } from './types'

type ViewMode = 'memory' | 'docs'

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

  const [viewMode, setViewMode] = useState<ViewMode>('memory')
  const [docsTree, setDocsTree] = useState<DocsNode | null>(null)
  const [selectedDocFile, setSelectedDocFile] = useState<DocFile | null>(null)

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

  const loadDocsTree = async () => {
    try {
      const tree = await api.fetchDocsTree()
      setDocsTree(tree)
    } catch (error) {
      console.error('Failed to load docs tree:', error)
      setDocsTree(null)
    }
  }

  // Load docs tree when switching to docs view
  useEffect(() => {
    if (viewMode === 'docs' && !docsTree) {
      loadDocsTree()
    }
  }, [viewMode, docsTree])

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

  const handleDocFileSelect = useCallback(async (path: string) => {
    try {
      const docFile = await fetchDocFile(path)
      setSelectedDocFile(docFile)
    } catch (err) {
      console.error('Failed to load doc file:', err)
    }
  }, [])

  const handleCreateDoc = useCallback(async (folderPath: string) => {
    const name = window.prompt('Document name (without .md):')
    if (!name) return

    const docPath = folderPath === '.' ? `${name}.md` : `${folderPath}/${name}.md`

    try {
      // Create the file with empty content
      await api.createDocFile(docPath, '', false)

      // Create a DocFile object and select it
      const newDoc: DocFile = {
        path: docPath,
        content: '',
        content_html: '',
        exists: true
      }
      setSelectedDocFile(newDoc)

      // Refresh docs tree to show new file
      await loadDocsTree()
    } catch (err) {
      if (err instanceof Error && err.message.includes('already exists')) {
        alert('A file with that name already exists')
      } else {
        console.error('Failed to create doc:', err)
        alert('Failed to create document')
      }
    }
  }, [])

  const handleSave = useCallback(async (
    path: string,
    content: string,
    isHtml: boolean = false
  ) => {
    if (viewMode === 'memory') {
      // Use existing memory file save logic
      return updateFile(path, content, isHtml)
    } else {
      // Doc file save logic
      try {
        const fileExists = selectedDocFile?.exists === true

        if (fileExists) {
          // Update existing doc
          const result = await api.updateDocFile(path, content, isHtml)
          setSelectedDocFile(prev => prev ? { ...prev, content: result.content, exists: true } : null)
          return { success: true, content: result.content }
        } else {
          // Create new doc
          if (!content || content.trim().length === 0) {
            return { success: true, content: '' }
          }
          const result = await api.createDocFile(path, content, isHtml)
          // Reload docs tree after creation
          // TODO: implement tree refresh
          setSelectedDocFile(prev => prev ? { ...prev, content: result.content, exists: true } : null)
          return { success: true, content: result.content }
        }
      } catch (error) {
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to save'
        }
      }
    }
  }, [viewMode, selectedDocFile, updateFile])

  // Helper to get the active editable file
  const getEditableFile = (): EditableFile | null => {
    if (viewMode === 'memory' && state.selectedFile) {
      return { type: 'memory', ...state.selectedFile }
    }
    if (viewMode === 'docs' && selectedDocFile) {
      return { type: 'doc', ...selectedDocFile }
    }
    return null
  }

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
          viewMode={viewMode}
          onViewModeChange={setViewMode}
          docsTree={docsTree}
          selectedDocFile={selectedDocFile}
          onDocFileSelect={handleDocFileSelect}
          onCreateDoc={handleCreateDoc}
        />

        <Editor
          file={getEditableFile()}
          onSave={handleSave}
          onDelete={viewMode === 'memory' ? deleteFile : undefined}
        />
      </div>
    </div>
  )
}

export default App