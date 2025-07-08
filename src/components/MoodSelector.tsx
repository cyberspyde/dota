import React from 'react';
import { Mood } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Sword, Shield, Zap, Sparkles, Skull } from 'lucide-react';

interface MoodSelectorProps {
  selectedMood: Mood | null;
  onMoodSelect: (mood: Mood) => void;
}

const moodConfig = {
  aggressive: {
    icon: Sword,
    color: 'border-red-500 bg-red-500/10 dark:bg-red-500/20 text-red-600 dark:text-red-400'
  },
  defensive: {
    icon: Shield,
    color: 'border-blue-500 bg-blue-500/10 dark:bg-blue-500/20 text-blue-600 dark:text-blue-400'
  },
  experimental: {
    icon: Zap,
    color: 'border-yellow-500 bg-yellow-500/10 dark:bg-yellow-500/20 text-yellow-600 dark:text-yellow-400'
  },
  creative: {
    icon: Sparkles,
    color: 'border-purple-500 bg-purple-500/10 dark:bg-purple-500/20 text-purple-600 dark:text-purple-400'
  },
  chaos: {
    icon: Skull,
    color: 'border-orange-500 bg-orange-500/10 dark:bg-orange-500/20 text-orange-600 dark:text-orange-400'
  }
};

export const MoodSelector: React.FC<MoodSelectorProps> = ({ selectedMood, onMoodSelect }) => {
  const { t } = useLanguage();

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-3 lg:gap-4 mb-8 px-4 lg:px-0">
      {Object.entries(moodConfig).map(([mood, config]) => {
        const Icon = config.icon;
        const isSelected = selectedMood === mood;
        
        return (
          <button
            key={mood}
            onClick={() => onMoodSelect(mood as Mood)}
            className={`p-3 lg:p-4 rounded-lg border-2 transition-all duration-200 hover:scale-105 active:scale-95 ${
              isSelected 
                ? config.color 
                : 'border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-gray-800/50 text-gray-600 dark:text-gray-300 hover:border-gray-400 dark:hover:border-gray-500'
            }`}
          >
            <div className="flex flex-col items-center space-y-2">
              <Icon size={20} className="lg:w-6 lg:h-6" />
              <div className="text-sm lg:text-base font-semibold">{t(`mood.${mood}`)}</div>
              <div className="text-xs lg:text-sm text-gray-500 dark:text-gray-400 text-center leading-tight">{t(`mood.${mood}.desc`)}</div>
            </div>
          </button>
        );
      })}
    </div>
  );
};