import { DocsNode, DocFile } from '../types'

const API_BASE = '/api'

export async function fetchDocsTree(): Promise<DocsNode | null> {
  const response = await fetch(`${API_BASE}/docs/tree`)
  if (!response.ok) {
    throw new Error('Failed to fetch docs tree')
  }
  return response.json()
}

export async function fetchDocFile(path: string): Promise<DocFile> {
  const response = await fetch(`${API_BASE}/docs/files/${path}`)
  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('Doc file not found')
    }
    throw new Error('Failed to fetch doc file')
  }
  return response.json()
}

export async function updateDocFile(
  path: string,
  content: string,
  isHtml: boolean = false
): Promise<{ content: string }> {
  const body = isHtml
    ? { content_html: content }
    : { content };

  console.log('Saving doc to:', path, 'isHtml:', isHtml, 'content length:', content.length)

  const response = await fetch(`${API_BASE}/docs/files/${path}`, {
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
    throw new Error(`Failed to update doc file: ${response.status}`)
  }

  const result = await response.json()
  console.log('Save successful, returned content length:', result.content?.length)
  return result
}
