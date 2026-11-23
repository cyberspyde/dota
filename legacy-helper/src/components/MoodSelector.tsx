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
    className: 'aggressive',
  },
  defensive: {
    icon: Shield,
    className: 'defensive',
  },
  experimental: {
    icon: Zap,
    className: 'experimental',
  },
  creative: {
    icon: Sparkles,
    className: 'creative',
  },
  chaos: {
    icon: Skull,
    className: 'chaos',
  }
};

export const MoodSelector: React.FC<MoodSelectorProps> = ({ selectedMood, onMoodSelect }) => {
  const { t } = useLanguage();

  return (
    <div className="mood-grid">
      {Object.entries(moodConfig).map(([mood, config]) => {
        const Icon = config.icon;
        const isSelected = selectedMood === mood;
        
        return (
          <button
            key={mood}
            onClick={() => onMoodSelect(mood as Mood)}
            className={`mood-card ${config.className} ${isSelected ? 'selected' : ''}`}
          >
            <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '1rem' }}>
              <Icon size={32} />
              <div className="mood-title">
                {t(`mood.${mood}`).toUpperCase()}
              </div>
              <div className="mood-desc">
                {t(`mood.${mood}.desc`)}
              </div>
            </div>
          </button>
        );
      })}
    </div>
  );
};