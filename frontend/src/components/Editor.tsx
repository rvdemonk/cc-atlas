import React, { useState, useEffect } from 'react'
import { MemoryFile } from '../App'
import './Editor.css'

interface EditorProps {
  file: MemoryFile
  onSave: (content: string) => void
}

const Editor: React.FC<EditorProps> = ({ file, onSave }) => {
  const [content, setContent] = useState(file.content)
  const [hasChanges, setHasChanges] = useState(false)

  useEffect(() => {
    setContent(file.content)
    setHasChanges(false)
  }, [file])

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setContent(e.target.value)
    setHasChanges(true)
  }

  const handleSave = () => {
    onSave(content)
    setHasChanges(false)
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.ctrlKey && e.key === 's') {
      e.preventDefault()
      handleSave()
    }
  }

  return (
    <div className="editor">
      <div className="editor-header">
        <div className="editor-title">
          <span className="editor-path">{file.relative_path}</span>
          {hasChanges && <span className="unsaved">●</span>}
        </div>
        <div className="editor-actions">
          <button 
            onClick={handleSave} 
            disabled={!hasChanges}
            className="save-button"
          >
            Save (Ctrl+S)
          </button>
        </div>
      </div>
      
      <div className="editor-stats">
        <span>📁 {file.stats.file_count} files</span>
        <span>📝 {file.stats.total_lines} lines</span>
        <span>📊 Depth: {file.stats.depth}</span>
      </div>
      
      <textarea
        className="editor-textarea"
        value={content}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        spellCheck={false}
      />
    </div>
  )
}

export default Editor