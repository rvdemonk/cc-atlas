import React, { useState, useEffect, useRef, useCallback } from 'react'
import { EditorProps, EditorState } from '../types'
import { HiEye, HiCode, HiCheck, HiX, HiTrash } from 'react-icons/hi'
import { HiTable, HiPlus, HiMinus } from 'react-icons/hi'
import { useEditor, EditorContent } from '@tiptap/react'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { Table, TableRow, TableCell, TableHeader } from '@tiptap/extension-table'
import './Editor.css'


export const Editor: React.FC<EditorProps> = ({ file, onSave, onDelete }) => {
  const [state, setState] = useState<EditorState>({
    content: '',
    saving: false,
    lastSaved: null,
    hasChanges: false,
    mode: 'wysiwyg'
  })

  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle')
  const contentRef = useRef<string>('')
  const lastLoadedPath = useRef<string | null>(null)
  const [fileExists, setFileExists] = useState<boolean>(false)

  // Determine placeholder text based on file existence
  const placeholderText = !fileExists && file
    ? `Start typing to create CLAUDE.md in ${file.path.replace('/CLAUDE.md', '')}...`
    : 'Start typing to create memory file here...'

  // TipTap editor instance
  const editor = useEditor({
    extensions: [
      StarterKit,
      Placeholder.configure({
        placeholder: placeholderText,
      }),
      Table.configure({
        resizable: true,
      }),
      TableRow,
      TableHeader,
      TableCell
    ],
    content: '',
    editorProps: {
      attributes: {
        class: 'tiptap-editor',
        autocorrect: 'off',
        autocapitalize: 'off',
        spellcheck: 'false'
      }
    },
    onUpdate: ({ editor }) => {
      // Mark as having changes when editing in WYSIWYG
      setState(prev => ({ ...prev, hasChanges: true }))
      setSaveStatus('idle')
    }
  })

  // Update placeholder when fileExists changes
  useEffect(() => {
    if (editor) {
      const newPlaceholder = !fileExists && file
        ? `Start typing to create CLAUDE.md in ${file.path.replace('/CLAUDE.md', '')}...`
        : 'Enter markdown content...'

      // Update placeholder through the extension
      const placeholderExtension = editor.extensionManager.extensions.find(
        extension => extension.name === 'placeholder'
      )

      if (placeholderExtension) {
        placeholderExtension.options.placeholder = newPlaceholder
        // Force re-render of placeholder
        editor.view.dispatch(editor.state.tr)
      }
    }
  }, [fileExists, file, editor])

  // Update editor when switching to a different file OR when file.exists changes
  useEffect(() => {
    if (!file || !editor) return

    // Update fileExists whenever it changes (including after file creation)
    setFileExists(file.exists || false)

    // Only reload content when switching to a different file
    if (file.path !== lastLoadedPath.current) {
      console.log('Loading new file into editor:', file.path, 'exists:', file.exists)
      lastLoadedPath.current = file.path

      // Use HTML for WYSIWYG mode
      const htmlContent = file.content_html || ''
      const markdownContent = file.content || ''

      // Clear editor content for new/non-existent files or when content is empty
      if (!file.exists || (!htmlContent && !markdownContent)) {
        console.log('Clearing editor for new file')
        editor.commands.clearContent()
        contentRef.current = ''

        setState(prev => ({
          ...prev,
          content: '',
          hasChanges: false
        }))
      } else {
        // Load existing content
        if (htmlContent) {
          editor.commands.setContent(htmlContent)
        } else if (markdownContent) {
          // Fallback to markdown if no HTML
          editor.commands.setContent(markdownContent)
        }

        contentRef.current = markdownContent  // Store markdown for source mode

        setState(prev => ({
          ...prev,
          content: markdownContent,
          hasChanges: false
        }))
      }

      setSaveStatus('idle')
    }
  }, [file, editor])

  const handleSave = useCallback(async () => {
    if (!file || !state.hasChanges || !editor) return

    setState(prev => ({ ...prev, saving: true }))
    setSaveStatus('saving')

    // Get content based on current mode
    let content: string
    let isHtml = false

    if (state.mode === 'wysiwyg') {
      // Send HTML for backend conversion
      content = editor.getHTML()
      isHtml = true
    } else {
      // Send markdown directly from source mode
      content = state.content
      isHtml = false
    }

    try {
      const wasNewFile = !fileExists
      const result = await onSave(file.path, content, isHtml)

      if (result.success) {
        // Update local markdown content with the backend's converted version
        if (result.content) {
          contentRef.current = result.content
          // Only update state content, don't reload the editor
          setState(prev => ({
            ...prev,
            content: result.content!,
            saving: false,
            lastSaved: new Date(),
            hasChanges: false
          }))
          // Don't update the TipTap editor content here - it already has the right content
        } else {
          setState(prev => ({
            ...prev,
            saving: false,
            lastSaved: new Date(),
            hasChanges: false
          }))
        }

        // If this was a new file creation, update our local state
        if (wasNewFile) {
          setFileExists(true)
        }

        setSaveStatus('saved')
        setTimeout(() => setSaveStatus('idle'), 2000)
      } else {
        console.error('Save failed:', result.error)
        setState(prev => ({ ...prev, saving: false }))
        setSaveStatus('error')
        setTimeout(() => setSaveStatus('idle'), 3000)
      }
    } catch (error) {
      console.error('Save error:', error)
      setState(prev => ({ ...prev, saving: false }))
      setSaveStatus('error')
      setTimeout(() => setSaveStatus('idle'), 3000)
    }
  }, [file, fileExists, state.hasChanges, state.mode, state.content, editor, onSave])

  // Track autosave timer
  const autosaveTimerRef = useRef<NodeJS.Timeout | null>(null)

  // Schedule autosave when content changes
  useEffect(() => {
    // Clear any existing timer
    if (autosaveTimerRef.current) {
      clearTimeout(autosaveTimerRef.current)
      autosaveTimerRef.current = null
    }

    // Only schedule if we have changes, a file, and not currently saving
    if (!state.hasChanges || !file || state.saving) {
      return
    }

    // Set a new timer - capture current values in closure
    const currentFileExists = fileExists
    const currentMode = state.mode

    autosaveTimerRef.current = setTimeout(() => {
      // For new files, ensure we have content
      if (!currentFileExists) {
        const currentContent = currentMode === 'wysiwyg'
          ? editor?.getHTML() || ''
          : contentRef.current

        if (!currentContent || currentContent.trim().length === 0) {
          return
        }
      }

      handleSave()
    }, 500)

    // Cleanup on unmount or when dependencies change
    return () => {
      if (autosaveTimerRef.current) {
        clearTimeout(autosaveTimerRef.current)
        autosaveTimerRef.current = null
      }
    }
    // Note: Intentionally not including handleSave to avoid re-creating timer
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state.hasChanges, state.content, state.mode, file, fileExists, state.saving, editor])

  const toggleMode = useCallback(() => {
    if (!editor) return

    // Save current changes before switching modes
    if (state.hasChanges) {
      handleSave()
    }

    setState(prev => ({
      ...prev,
      mode: prev.mode === 'wysiwyg' ? 'source' : 'wysiwyg'
    }))
  }, [editor, state.hasChanges, handleSave])

  const handleDelete = useCallback(async () => {
    if (!file || !onDelete) return

    const confirmDelete = window.confirm(`Are you sure you want to delete ${file.path}?`)
    if (!confirmDelete) return

    try {
      const success = await onDelete(file.path)
      if (success) {
        // File deleted successfully - the parent component will handle cleanup
        console.log('File deleted successfully')
      }
    } catch (error) {
      console.error('Failed to delete file:', error)
      alert('Failed to delete file. Please try again.')
    }
  }, [file, onDelete])

  if (!file) {
    return (
      <div className="editor-empty">
        <div className="empty-state">
          <HiCode className="empty-icon" />
          <h2>No file selected</h2>
          <p>Select a CLAUDE.md file from the sidebar to start editing</p>
        </div>
      </div>
    )
  }

  return (
    <div className="editor">
      <div className="editor-header">
        <div className="editor-path">
          <span className="path-label">Editing:</span>
          <span className="path-value">{file.path}</span>
        </div>

        <div className="editor-actions">
          {saveStatus !== 'idle' && (
            <div className="save-status">
              {saveStatus === 'saving' && (
                <span className="status-saving">Saving...</span>
              )}
              {saveStatus === 'saved' && (
                <span className="status-saved">
                  <HiCheck /> Saved
                </span>
              )}
              {saveStatus === 'error' && (
                <span className="status-error">
                  <HiX /> Error
                </span>
              )}
            </div>
          )}

          {state.mode === 'wysiwyg' && editor && (
            <div className="table-controls">
              <button
                className="action-btn"
                onClick={() => editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()}
                title="Insert table"
              >
                <HiTable /> <HiPlus />
              </button>
              {editor.can().addColumnAfter() && (
                <>
                  <button
                    className="action-btn"
                    onClick={() => editor.chain().focus().addColumnAfter().run()}
                    title="Add column"
                  >
                    Col <HiPlus />
                  </button>
                  <button
                    className="action-btn"
                    onClick={() => editor.chain().focus().deleteColumn().run()}
                    title="Delete column"
                  >
                    Col <HiMinus />
                  </button>
                </>
              )}
              {editor.can().addRowAfter() && (
                <>
                  <button
                    className="action-btn"
                    onClick={() => editor.chain().focus().addRowAfter().run()}
                    title="Add row"
                  >
                    Row <HiPlus />
                  </button>
                  <button
                    className="action-btn"
                    onClick={() => editor.chain().focus().deleteRow().run()}
                    title="Delete row"
                  >
                    Row <HiMinus />
                  </button>
                  <button
                    className="action-btn"
                    onClick={() => editor.chain().focus().deleteTable().run()}
                    title="Delete table"
                  >
                    <HiTable /> <HiMinus />
                  </button>
                </>
              )}
            </div>
          )}

          <button
            className={`action-btn ${state.mode === 'wysiwyg' ? 'active' : ''}`}
            onClick={toggleMode}
            title={state.mode === 'wysiwyg' ? 'Switch to source' : 'Switch to WYSIWYG'}
          >
            {state.mode === 'wysiwyg' ? <HiCode /> : <HiEye />}
          </button>

          {onDelete && fileExists && (
            <button
              className="action-btn action-delete"
              onClick={handleDelete}
              title="Delete file"
            >
              <HiTrash /> Delete
            </button>
          )}
        </div>
      </div>

      <div className="editor-content">
        <div className="editor-content-inner">
          {state.mode === 'wysiwyg' ? (
            <EditorContent editor={editor} />
          ) : (
            <textarea
              className="source-editor"
              value={state.content}
              onChange={(e) => {
                const newContent = e.target.value
                contentRef.current = newContent
                setState(prev => ({ ...prev, content: newContent, hasChanges: true }))
                setSaveStatus('idle')
                if (editor) {
                  editor.commands.setContent(newContent)
                }
              }}
              placeholder={placeholderText}
            />
          )}
        </div>
      </div>
    </div>
  )
}
