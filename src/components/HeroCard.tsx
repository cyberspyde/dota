import React from 'react';
import { Hero } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Star, Target, Shield, Zap, Sword } from 'lucide-react';

interface HeroCardProps {
  hero: Hero;
  score?: number;
}

const roleIcons = {
  Support: Shield,
  Mid: Target,
  Carry: Sword,
  Initiator: Zap
};

export const HeroCard: React.FC<HeroCardProps> = ({ hero, score }) => {
  const { t } = useLanguage();
  const RoleIcon = roleIcons[hero.role as keyof typeof roleIcons] || Target;

  const getLocalizedText = (key: string, fallback: string) => {
    const localized = t(key);
    return localized !== key ? localized : fallback;
  };

  const heroName = getLocalizedText(`hero.${hero.id}.name`, hero.name);

  const renderScore = () => {
    if (score === undefined || score === null) return null;
    return (
      <div className="hero-card-score">
        <Star size={12} style={{ color: '#ffff00' }}/>
        <span>{score.toFixed(1)}</span>
      </div>
    );
  };

  return (
    <div className="hero-card-grid-item card">
      <div className="hero-header">
        <div className="hero-info">
          <RoleIcon size={24} style={{ color: '#00d4ff' }} />
          <h3 className="hero-name">{heroName.toUpperCase()}</h3>
        </div>
        {renderScore()}
      </div>
    </div>
  );
};