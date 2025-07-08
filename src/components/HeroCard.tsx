import React from 'react';
import { Hero } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Star, Target, Shield, Zap, Sword } from 'lucide-react';

interface HeroCardProps {
  hero: Hero;
}

const difficultyConfig = {
  Easy: { color: 'text-green-600 dark:text-green-400', stars: 1 },
  Medium: { color: 'text-yellow-600 dark:text-yellow-400', stars: 2 },
  Hard: { color: 'text-red-600 dark:text-red-400', stars: 3 }
};

const roleIcons = {
  Support: Shield,
  Mid: Target,
  Carry: Sword,
  Initiator: Zap
};

export const HeroCard: React.FC<HeroCardProps> = ({ hero }) => {
  const { t } = useLanguage();
  const difficultySettings = difficultyConfig[hero.difficulty];
  const RoleIcon = roleIcons[hero.role as keyof typeof roleIcons] || Target;

  // Get localized hero data
  const getLocalizedText = (key: string, fallback: string) => {
    const localized = t(key);
    return localized !== key ? localized : fallback;
  };

  const heroName = getLocalizedText(`hero.${hero.id}.name`, hero.name);
  const heroDescription = getLocalizedText(`hero.${hero.id}.description`, hero.description);
  const heroRole = getLocalizedText(`role.${hero.role}`, hero.role);
  const heroDifficulty = getLocalizedText(`difficulty.${hero.difficulty}`, hero.difficulty);

  return (
    <div className="bg-white/80 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-lg p-4 lg:p-6 hover:border-gray-300 dark:hover:border-gray-600 transition-colors backdrop-blur-sm mx-4 lg:mx-0">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 space-y-2 sm:space-y-0">
        <div className="flex items-center space-x-3">
          <RoleIcon size={20} className="text-blue-600 dark:text-blue-400 lg:w-6 lg:h-6" />
          <div>
            <h3 className="text-lg lg:text-xl font-bold text-gray-800 dark:text-white">{heroName}</h3>
            <p className="text-sm lg:text-base text-gray-600 dark:text-gray-400">{heroRole}</p>
          </div>
        </div>
        <div className="flex items-center space-x-1">
          {[...Array(3)].map((_, i) => (
            <Star
              key={i}
              size={14}
              className={i < difficultySettings.stars ? difficultySettings.color : 'text-gray-400 dark:text-gray-600'}
              fill={i < difficultySettings.stars ? 'currentColor' : 'none'}
            />
          ))}
          <span className="ml-2 text-xs text-gray-500 dark:text-gray-400">({heroDifficulty})</span>
        </div>
      </div>
      
      <p className="text-sm lg:text-base text-gray-700 dark:text-gray-300 mb-4">{heroDescription}</p>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <h4 className="text-sm lg:text-base text-green-600 dark:text-green-400 font-semibold mb-2">{t('hero.strengths')}</h4>
          <ul className="space-y-1">
            {hero.strengths.map((strength, index) => {
              const localizedStrength = getLocalizedText(`hero.${hero.id}.strengths.${index}`, strength);
              return (
                <li key={index} className="text-sm text-gray-700 dark:text-gray-300">• {localizedStrength}</li>
              );
            })}
          </ul>
        </div>
        <div>
          <h4 className="text-sm lg:text-base text-red-600 dark:text-red-400 font-semibold mb-2">{t('hero.weaknesses')}</h4>
          <ul className="space-y-1">
            {hero.weaknesses.map((weakness, index) => {
              const localizedWeakness = getLocalizedText(`hero.${hero.id}.weaknesses.${index}`, weakness);
              return (
                <li key={index} className="text-sm text-gray-700 dark:text-gray-300">• {localizedWeakness}</li>
              );
            })}
          </ul>
        </div>
      </div>
    </div>
  );
};