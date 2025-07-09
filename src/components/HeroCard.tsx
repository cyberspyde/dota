import React from 'react';
import { Hero } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Star, Target, Shield, Zap, Sword, Plus, Minus } from 'lucide-react';

interface HeroCardProps {
  hero: Hero;
}

const difficultyConfig = {
  Easy: { color: '#00ff88', stars: 1 },
  Medium: { color: '#ffff00', stars: 2 },
  Hard: { color: '#ff4444', stars: 3 }
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
    <div className="hero-card card">
      <div className="hero-header">
        <div className="hero-info">
          <div style={{ position: 'relative' }}>
            <RoleIcon size={32} style={{ color: '#00d4ff' }} />
          </div>
          <div>
            <h3 className="hero-name">{heroName.toUpperCase()}</h3>
            <p className="hero-role">{heroRole.toUpperCase()}</p>
          </div>
        </div>
        
        <div className="hero-difficulty">
          <div className="stars">
            {[...Array(3)].map((_, i) => (
              <Star
                key={i}
                size={16}
                className={`star ${i < difficultySettings.stars ? 'active' : 'inactive'}`}
                fill={i < difficultySettings.stars ? 'currentColor' : 'none'}
                style={{ color: i < difficultySettings.stars ? difficultySettings.color : 'rgba(128, 128, 128, 0.5)' }}
              />
            ))}
          </div>
          <span style={{ 
            fontSize: '0.875rem',
            color: 'rgba(255, 255, 255, 0.6)',
            fontFamily: 'Fira Code, monospace'
          }}>
            [{heroDifficulty.toUpperCase()}]
          </span>
        </div>
      </div>
      
      <div className="hero-description">
        <p style={{ 
          color: 'rgba(255, 255, 255, 0.8)',
          fontSize: '0.875rem',
          fontFamily: 'Fira Code, monospace',
          lineHeight: '1.6'
        }}>
          {heroDescription}
        </p>
      </div>
      
      <div className="hero-stats">
        <div className="stat-section strengths">
          <h4>
            <Plus size={16} />
            {t('hero.strengths').toUpperCase()}
          </h4>
          <ul className="stat-list">
            {hero.strengths.map((strength, index) => {
              const localizedStrength = getLocalizedText(`hero.${hero.id}.strengths.${index}`, strength);
              return (
                <li key={index} className="stat-item">
                  <span className="stat-marker">+</span>
                  {localizedStrength}
                </li>
              );
            })}
          </ul>
        </div>
        
        <div className="stat-section weaknesses">
          <h4>
            <Minus size={16} />
            {t('hero.weaknesses').toUpperCase()}
          </h4>
          <ul className="stat-list">
            {hero.weaknesses.map((weakness, index) => {
              const localizedWeakness = getLocalizedText(`hero.${hero.id}.weaknesses.${index}`, weakness);
              return (
                <li key={index} className="stat-item">
                  <span className="stat-marker">-</span>
                  {localizedWeakness}
                </li>
              );
            })}
          </ul>
        </div>
      </div>
    </div>
  );
};