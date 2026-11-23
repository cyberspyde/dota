import React from 'react';
import { useTheme } from '../contexts/ThemeContext';
import { Sun, Moon, Monitor } from 'lucide-react';

export const ThemeToggle: React.FC = () => {
  const { theme, toggleTheme } = useTheme();

  const getIcon = () => {
    switch (theme) {
      case 'light':
        return <Sun size={16} style={{ color: '#ffff00' }} />;
      case 'dark':
        return <Moon size={16} style={{ color: '#8b5cf6' }} />;
      default:
        return <Monitor size={16} style={{ color: '#8b5cf6' }} />;
    }
  };

  return (
    <button
      onClick={toggleTheme}
      className="btn btn-secondary"
      title={`THEME: ${theme === 'light' ? 'LIGHT' : 'DARK'}`}
    >
      {getIcon()}
      <span>
        {theme === 'light' ? 'LIGHT' : 'DARK'}
      </span>
    </button>
  );
};