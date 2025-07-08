import React from 'react';
import { useTheme } from '../contexts/ThemeContext';
import { useLanguage } from '../contexts/LanguageContext';
import { Sun, Moon } from 'lucide-react';

export const ThemeToggle: React.FC = () => {
  const { theme, toggleTheme } = useTheme();
  const { t } = useLanguage();

  return (
    <button
      onClick={toggleTheme}
      className="flex items-center space-x-1 lg:space-x-2 px-2 lg:px-3 py-2 rounded-lg bg-white/10 dark:bg-gray-800/50 border border-gray-300 dark:border-gray-700 hover:bg-white/20 dark:hover:bg-gray-700/50 transition-colors"
      title={`${t('settings.theme')}: ${theme === 'light' ? t('theme.light') : t('theme.dark')}`}
    >
      {theme === 'light' ? (
        <Moon size={16} className="text-blue-600 lg:w-[18px] lg:h-[18px]" />
      ) : (
        <Sun size={16} className="text-yellow-400 lg:w-[18px] lg:h-[18px]" />
      )}
      <span className="text-xs lg:text-sm font-medium text-gray-800 dark:text-gray-200 hidden sm:inline">
        {theme === 'light' ? t('theme.light') : t('theme.dark')}
      </span>
    </button>
  );
};