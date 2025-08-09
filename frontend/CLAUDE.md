# React Frontend Context

## Structure
- `App.tsx` - Main component, state management
- `components/` - TreeView, Editor components
- `api/client.ts` - Backend communication

## Component Patterns

### TreeView
- Recursive rendering for nested directories
- Icons indicate state: 📝 (has memory), 💡 (recommended)
- Badges: M (memory exists), R (recommended)
- Expand/collapse state managed per node

### Editor
- Simple textarea (no fancy markdown for MVP)
- Ctrl+S to save
- Orange dot for unsaved changes
- Shows file stats (lines, files, depth)

## State Management
- useState at App level
- Props drilling (no context/redux for MVP)
- Single source of truth: `memoryFiles` array

## Styling
- Dark theme matching VS Code
- CSS modules per component
- No CSS framework (keep it light)

## API Integration
```typescript
// Simple fetch wrappers
fetchTree(): Promise<DirectoryInfo>
updateMemoryFile(path, content): Promise<void>
```

## TypeScript
- Interfaces for all data structures
- Strict mode enabled
- Export types from App.tsx

## Development
```bash
npm run dev   # Vite dev server on :3000
npm run build # Production build
```