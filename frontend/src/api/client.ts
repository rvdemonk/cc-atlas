import { DirectoryInfo, MemoryFile } from '../App'

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

export async function updateMemoryFile(path: string, content: string): Promise<void> {
  const response = await fetch(`${API_BASE}/memory-files/${path}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ content }),
  })
  
  if (!response.ok) {
    throw new Error('Failed to update memory file')
  }
}

export async function fetchRecommendations(): Promise<string[]> {
  const response = await fetch(`${API_BASE}/recommendations`)
  if (!response.ok) {
    throw new Error('Failed to fetch recommendations')
  }
  return response.json()
}