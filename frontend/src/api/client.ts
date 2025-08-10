import { DirectoryInfo, MemoryFile } from '../types'

const API_BASE = '/api'

export async function fetchTree(): Promise<DirectoryInfo> {
  const response = await fetch(`${API_BASE}/tree`)
  if (!response.ok) {
    throw new Error('Failed to fetch tree')
  }
  return response.json()
}

export async function fetchMemoryFiles(): Promise<MemoryFile[]> {
  const response = await fetch(`${API_BASE}/memory-files`)
  if (!response.ok) {
    throw new Error('Failed to fetch memory files')
  }
  return response.json()
}

export async function updateMemoryFile(
  path: string, 
  content: string, 
  isHtml: boolean = false
): Promise<{ content: string }> {
  const body = isHtml 
    ? { content_html: content }
    : { content };
  
  console.log('Saving to:', path, 'isHtml:', isHtml, 'content length:', content.length)
    
  const response = await fetch(`${API_BASE}/memory-files/${path}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  })
  
  if (!response.ok) {
    console.error('Save failed with status:', response.status)
    const text = await response.text()
    console.error('Response:', text)
    throw new Error(`Failed to update memory file: ${response.status}`)
  }
  
  // Check if response has content
  const text = await response.text()
  console.log('Response text:', text)
  
  if (!text) {
    console.warn('Empty response from server, returning empty content')
    return { content: '' }
  }
  
  try {
    const result = JSON.parse(text)
    console.log('Save successful, returned content length:', result.content?.length)
    return result
  } catch (e) {
    console.error('Failed to parse response as JSON:', e)
    console.error('Response was:', text)
    throw new Error('Invalid response from server')
  }
}

export async function createMemoryFile(
  path: string, 
  content: string, 
  isHtml: boolean = false
): Promise<{ content: string; created: boolean }> {
  const body = isHtml 
    ? { content_html: content }
    : { content };
  
  console.log('Creating file at:', path, 'isHtml:', isHtml, 'content length:', content.length)
    
  const response = await fetch(`${API_BASE}/memory-files/${path}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  })
  
  if (!response.ok) {
    // If we get 409 Conflict, the file already exists - treat as success
    if (response.status === 409) {
      console.log('File already exists, switching to update mode')
      // Return a success response so the Editor knows to switch to update mode
      return { content, created: false }
    }
    
    console.error('Create failed with status:', response.status)
    const text = await response.text()
    console.error('Response:', text)
    throw new Error(`Failed to create memory file: ${response.status}`)
  }
  
  const result = await response.json()
  console.log('Create successful, returned content length:', result.content?.length)
  return { ...result, created: true }
}

export async function deleteMemoryFile(path: string): Promise<{ deleted: boolean }> {
  console.log('Deleting file at:', path)
  
  const response = await fetch(`${API_BASE}/memory-files/${path}`, {
    method: 'DELETE',
  })
  
  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('File not found')
    }
    console.error('Delete failed with status:', response.status)
    const text = await response.text()
    console.error('Response:', text)
    throw new Error(`Failed to delete memory file: ${response.status}`)
  }
  
  const result = await response.json()
  console.log('Delete successful:', result)
  return result
}

export async function fetchRecommendations(): Promise<string[]> {
  const response = await fetch(`${API_BASE}/recommendations`)
  if (!response.ok) {
    throw new Error('Failed to fetch recommendations')
  }
  return response.json()
}