import React from 'react';
import { useLanguage, Language } from '../contexts/LanguageContext';
import { Globe } from 'lucide-react';

export const LanguageSelector: React.FC = () => {
  const { language, setLanguage, t } = useLanguage();

  const languages: { code: Language; name: string; flag: string }[] = [
    { code: 'en', name: 'English', flag: '🇺🇸' },
    { code: 'ru', name: 'Русский', flag: '🇷🇺' },
    { code: 'uz', name: 'O\'zbek', flag: '🇺🇿' }
  ];

  return (
    <div className="relative group">
      <button className="flex items-center space-x-1 lg:space-x-2 px-2 lg:px-3 py-2 rounded-lg bg-white/10 dark:bg-gray-800/50 border border-gray-300 dark:border-gray-700 hover:bg-white/20 dark:hover:bg-gray-700/50 transition-colors">
        <Globe size={16} className="text-blue-400 lg:w-[18px] lg:h-[18px]" />
        <span className="text-xs lg:text-sm font-medium text-gray-800 dark:text-gray-200">
          {languages.find(l => l.code === language)?.flag}
        </span>
      </button>
      
      <div className="absolute top-full right-0 mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-lg shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-50 min-w-[120px] lg:min-w-[140px]">
        {languages.map(lang => (
          <button
            key={lang.code}
            onClick={() => setLanguage(lang.code)}
            className={`w-full text-left px-2 lg:px-3 py-2 text-xs lg:text-sm hover:bg-gray-100 dark:hover:bg-gray-700 first:rounded-t-lg last:rounded-b-lg transition-colors ${
              language === lang.code 
                ? 'bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400' 
                : 'text-gray-700 dark:text-gray-300'
            }`}
          >
            <span className="mr-2">{lang.flag}</span>
            {lang.name}
          </button>
        ))}
      </div>
    </div>
  );
};