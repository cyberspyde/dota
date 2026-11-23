import React, { useState } from 'react';
import { useLanguage, Language } from '../contexts/LanguageContext';
import { Globe } from 'lucide-react';

export const LanguageSelector: React.FC = () => {
  const { language, setLanguage } = useLanguage();
  const [isOpen, setIsOpen] = useState(false);

  const languages: { code: Language; name: string; flag: string }[] = [
    { code: 'en', name: 'ENGLISH', flag: '🇺🇸' },
    { code: 'ru', name: 'РУССКИЙ', flag: '🇷🇺' },
    { code: 'uz', name: 'O\'ZBEK', flag: '🇺🇿' }
  ];

  const currentLanguage = languages.find(l => l.code === language);

  return (
    <div style={{ position: 'relative' }}>
      <button 
        className="btn btn-secondary"
        onClick={() => setIsOpen(!isOpen)}
        onBlur={() => setTimeout(() => setIsOpen(false), 200)}
      >
        <Globe size={16} />
        <span>{currentLanguage?.flag}</span>
      </button>
      
      {isOpen && (
        <div style={{
          position: 'absolute',
          top: '100%',
          right: 0,
          marginTop: '0.5rem',
          minWidth: '140px',
          zIndex: 50
        }}>
          <div className="card" style={{ padding: 0 }}>
            {languages.map(lang => (
              <button
                key={lang.code}
                onClick={() => {
                  setLanguage(lang.code);
                  setIsOpen(false);
                }}
                style={{
                  width: '100%',
                  textAlign: 'left',
                  padding: '0.75rem 1rem',
                  fontSize: '0.875rem',
                  fontFamily: 'Fira Code, monospace',
                  background: language === lang.code ? 'rgba(0, 212, 255, 0.2)' : 'transparent',
                  color: language === lang.code ? '#00d4ff' : 'rgba(255, 255, 255, 0.8)',
                  border: 'none',
                  cursor: 'pointer',
                  transition: 'all 0.3s ease',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '0.5rem'
                }}
                onMouseEnter={(e) => {
                  if (language !== lang.code) {
                    e.currentTarget.style.background = 'rgba(0, 212, 255, 0.1)';
                    e.currentTarget.style.color = 'white';
                  }
                }}
                onMouseLeave={(e) => {
                  if (language !== lang.code) {
                    e.currentTarget.style.background = 'transparent';
                    e.currentTarget.style.color = 'rgba(255, 255, 255, 0.8)';
                  }
                }}
              >
                <span>{lang.flag}</span>
                {lang.name}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};