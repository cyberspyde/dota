import React, { useState, useMemo } from 'react';
import { Hero, Build } from '../services/api';
import { useLanguage } from '../contexts/LanguageContext';
import { Search, X, Star, Target, Shield, Zap, Sword } from 'lucide-react';

interface HeroSearchProps {
  onHeroSelect: (hero: Hero) => void;
  isOpen: boolean;
  onClose: () => void;
  heroes: Hero[];
  builds: Build[];
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

export const HeroSearch: React.FC<HeroSearchProps> = ({ onHeroSelect, isOpen, onClose, heroes, builds }) => {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedRole, setSelectedRole] = useState<string>('');
  const [selectedDifficulty, setSelectedDifficulty] = useState<string>('');
  const { t } = useLanguage();

  const filteredHeroes = useMemo(() => {
    return heroes.filter(hero => {
      const matchesSearch = hero.name.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesRole = selectedRole === '' || hero.role === selectedRole;
      const matchesDifficulty = selectedDifficulty === '' || hero.difficulty === selectedDifficulty;
      
      return matchesSearch && matchesRole && matchesDifficulty;
    });
  }, [searchQuery, selectedRole, selectedDifficulty]);

  const handleHeroClick = (hero: Hero) => {
    onHeroSelect(hero);
    onClose();
    setSearchQuery('');
    setSelectedRole('');
    setSelectedDifficulty('');
  };

  const roles = [...new Set(heroes.map(hero => hero.role))];
  const difficulties = ['Easy', 'Medium', 'Hard'];

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-white dark:bg-gray-800 rounded-lg w-full max-w-4xl max-h-[90vh] overflow-hidden">
        <div className="p-4 lg:p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl lg:text-2xl font-bold text-gray-800 dark:text-white">
              {t('hero.search.title') || 'Hero Search'}
            </h2>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
            >
              <X size={20} className="text-gray-600 dark:text-gray-400" />
            </button>
          </div>

          <div className="space-y-4">
            <div className="relative">
              <Search size={20} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" />
              <input
                type="text"
                placeholder={t('hero.search.placeholder') || 'Search heroes...'}
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-10 pr-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>

            <div className="flex flex-wrap gap-2">
              <select
                value={selectedRole}
                onChange={(e) => setSelectedRole(e.target.value)}
                className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
              >
                <option value="">{t('hero.search.allRoles') || 'All Roles'}</option>
                {roles.map(role => (
                  <option key={role} value={role}>{role}</option>
                ))}
              </select>

              <select
                value={selectedDifficulty}
                onChange={(e) => setSelectedDifficulty(e.target.value)}
                className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
              >
                <option value="">{t('hero.search.allDifficulties') || 'All Difficulties'}</option>
                {difficulties.map(difficulty => (
                  <option key={difficulty} value={difficulty}>{difficulty}</option>
                ))}
              </select>
            </div>
          </div>
        </div>

        <div className="p-4 lg:p-6 max-h-[60vh] overflow-y-auto">
          <div className="text-sm text-gray-600 dark:text-gray-400 mb-4">
            {filteredHeroes.length} {t('hero.search.heroesFound') || 'heroes found'}
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            {filteredHeroes.map(hero => {
              const RoleIcon = roleIcons[hero.role as keyof typeof roleIcons] || Target;
              const difficultySettings = difficultyConfig[hero.difficulty];
              const hasBuild = builds.some(build => build.heroId === hero.id);

              return (
                <div
                  key={hero.id}
                  onClick={() => handleHeroClick(hero)}
                  className="bg-gray-50 dark:bg-gray-900/50 p-3 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700/50 cursor-pointer transition-colors border-2 border-transparent hover:border-blue-500/30"
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center space-x-2">
                      <RoleIcon size={16} className="text-blue-600 dark:text-blue-400" />
                      <h3 className="font-semibold text-gray-800 dark:text-white text-sm">{hero.name}</h3>
                    </div>
                    <div className="flex items-center space-x-1">
                      {[...Array(3)].map((_, i) => (
                        <Star
                          key={i}
                          size={12}
                          className={i < difficultySettings.stars ? difficultySettings.color : 'text-gray-400 dark:text-gray-600'}
                          fill={i < difficultySettings.stars ? 'currentColor' : 'none'}
                        />
                      ))}
                    </div>
                  </div>
                  
                  <div className="flex items-center justify-between text-xs">
                    <span className="text-gray-600 dark:text-gray-400">{hero.role}</span>
                    {hasBuild && (
                      <span className="bg-green-500/20 text-green-600 dark:text-green-400 px-2 py-1 rounded-full text-xs font-medium">
                        {t('hero.search.hasBuild') || 'Has Build'}
                      </span>
                    )}
                  </div>
                </div>
              );
            })}
          </div>

          {filteredHeroes.length === 0 && (
            <div className="text-center py-8 text-gray-500 dark:text-gray-400">
              {t('hero.search.noResults') || 'No heroes found matching your search criteria.'}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};