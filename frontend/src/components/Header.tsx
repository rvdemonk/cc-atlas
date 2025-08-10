import React from 'react'
import { HeaderProps } from '../types'
import { HiMenuAlt2, HiCode } from 'react-icons/hi'
import './Header.css'

export const Header: React.FC<HeaderProps> = ({ onToggleSidebar, projectName }) => {
  return (
    <header className="header">
      <button 
        className="header-menu-btn"
        onClick={onToggleSidebar}
        aria-label="Toggle sidebar"
      >
        <HiMenuAlt2 />
      </button>
      
      <div className="header-brand">
        <HiCode className="header-icon" />
        <h1 className="header-title">cc-atlas</h1>
        <span className="header-separator">/</span>
        <span className="header-project">{projectName}</span>
      </div>
      
      <div className="header-status">
        <span className="status-indicator" />
        <span className="status-text">Connected</span>
      </div>
    </header>
  )
}