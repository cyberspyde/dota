import React, { useState, useMemo } from 'react';
import { Hero, Build } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Search, X, Star, Target, Shield, Zap, Sword, Filter } from 'lucide-react';

interface HeroSearchProps {
  onHeroSelect: (hero: Hero) => void;
  isOpen: boolean;
  onClose: () => void;
  heroes: Hero[];
  builds: Build[];
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
  }, [searchQuery, selectedRole, selectedDifficulty, heroes]);

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
    <div style={{
      position: 'fixed',
      inset: 0,
      background: 'rgba(0, 0, 0, 0.8)',
      backdropFilter: 'blur(4px)',
      zIndex: 50,
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '1rem'
    }}>
      <div className="card" style={{
        width: '100%',
        maxWidth: '72rem',
        maxHeight: '90vh',
        overflow: 'hidden',
        display: 'flex',
        flexDirection: 'column'
      }}>
        <div style={{
          padding: '1.5rem',
          borderBottom: '1px solid rgba(0, 212, 255, 0.3)'
        }}>
          <div style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            marginBottom: '1.5rem'
          }}>
            <h2 style={{
              fontSize: '1.5rem',
              fontWeight: 'bold',
              color: '#00d4ff',
              textShadow: '0 0 20px rgba(0, 212, 255, 0.5)',
              fontFamily: 'Fira Code, monospace'
            }}>
              {t('hero.search.title')?.toUpperCase() || 'HERO DATABASE'}
            </h2>
            <button onClick={onClose} className="btn btn-secondary" style={{ padding: '0.5rem' }}>
              <X size={20} />
            </button>
          </div>

          <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <div style={{ position: 'relative' }}>
              <Search size={20} style={{
                position: 'absolute',
                left: '0.75rem',
                top: '50%',
                transform: 'translateY(-50%)',
                color: '#00d4ff'
              }} />
              <input
                type="text"
                placeholder={t('hero.search.placeholder')?.toUpperCase() || 'SEARCH HEROES...'}
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                style={{
                  width: '100%',
                  paddingLeft: '2.75rem',
                  paddingRight: '1rem',
                  paddingTop: '0.75rem',
                  paddingBottom: '0.75rem',
                  background: 'rgba(26, 26, 46, 0.8)',
                  border: '1px solid rgba(0, 212, 255, 0.5)',
                  color: 'white',
                  fontFamily: 'Fira Code, monospace',
                  fontSize: '0.875rem',
                  borderRadius: '4px',
                  outline: 'none',
                  transition: 'all 0.3s ease'
                }}
                onFocus={(e) => {
                  e.target.style.borderColor = '#00d4ff';
                  e.target.style.boxShadow = '0 0 20px rgba(0, 212, 255, 0.5)';
                }}
                onBlur={(e) => {
                  e.target.style.borderColor = 'rgba(0, 212, 255, 0.5)';
                  e.target.style.boxShadow = 'none';
                }}
              />
            </div>

            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                <Filter size={16} style={{ color: '#00d4ff' }} />
                <select
                  value={selectedRole}
                  onChange={(e) => setSelectedRole(e.target.value)}
                  style={{
                    background: 'rgba(26, 26, 46, 0.8)',
                    border: '1px solid rgba(0, 212, 255, 0.5)',
                    color: 'white',
                    fontFamily: 'Fira Code, monospace',
                    fontSize: '0.875rem',
                    padding: '0.5rem',
                    borderRadius: '4px',
                    outline: 'none'
                  }}
                >
                  <option value="">{t('hero.search.allRoles')?.toUpperCase() || 'ALL ROLES'}</option>
                  {roles.map(role => (
                    <option key={role} value={role}>{role.toUpperCase()}</option>
                  ))}
                </select>
              </div>

              <select
                value={selectedDifficulty}
                onChange={(e) => setSelectedDifficulty(e.target.value)}
                style={{
                  background: 'rgba(26, 26, 46, 0.8)',
                  border: '1px solid rgba(0, 212, 255, 0.5)',
                  color: 'white',
                  fontFamily: 'Fira Code, monospace',
                  fontSize: '0.875rem',
                  padding: '0.5rem',
                  borderRadius: '4px',
                  outline: 'none'
                }}
              >
                <option value="">{t('hero.search.allDifficulties')?.toUpperCase() || 'ALL DIFFICULTIES'}</option>
                {difficulties.map(difficulty => (
                  <option key={difficulty} value={difficulty}>{difficulty.toUpperCase()}</option>
                ))}
              </select>
            </div>
          </div>
        </div>

        <div style={{
          padding: '1.5rem',
          maxHeight: '60vh',
          overflowY: 'auto',
          flex: 1
        }}>
          <div style={{
            fontSize: '0.875rem',
            color: '#00d4ff',
            fontFamily: 'Fira Code, monospace',
            marginBottom: '1rem'
          }}>
            {filteredHeroes.length} {t('hero.search.heroesFound')?.toUpperCase() || 'HEROES FOUND'}
          </div>

          <div style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(250px, 1fr))',
            gap: '1rem'
          }}>
            {filteredHeroes.map(hero => {
              const RoleIcon = roleIcons[hero.role as keyof typeof roleIcons] || Target;
              const difficultySettings = difficultyConfig[hero.difficulty];
              const hasBuild = builds.some(build => build.heroId === hero.id);

              return (
                <div
                  key={hero.id}
                  onClick={() => handleHeroClick(hero)}
                  style={{
                    background: 'rgba(5, 5, 8, 0.5)',
                    border: '1px solid rgba(0, 212, 255, 0.3)',
                    padding: '1rem',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    transition: 'all 0.3s ease'
                  }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.borderColor = 'rgba(0, 212, 255, 0.6)';
                    e.currentTarget.style.background = 'rgba(5, 5, 8, 0.7)';
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.borderColor = 'rgba(0, 212, 255, 0.3)';
                    e.currentTarget.style.background = 'rgba(5, 5, 8, 0.5)';
                  }}
                >
                  <div style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    marginBottom: '0.75rem'
                  }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
                      <RoleIcon size={20} style={{ color: '#00d4ff' }} />
                      <h3 style={{
                        fontWeight: 'bold',
                        color: 'white',
                        fontSize: '0.875rem',
                        fontFamily: 'Fira Code, monospace'
                      }}>
                        {hero.name.toUpperCase()}
                      </h3>
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
                      {[...Array(3)].map((_, i) => (
                        <Star
                          key={i}
                          size={10}
                          style={{
                            color: i < difficultySettings.stars ? difficultySettings.color : 'rgba(128, 128, 128, 0.5)'
                          }}
                          fill={i < difficultySettings.stars ? 'currentColor' : 'none'}
                        />
                      ))}
                    </div>
                  </div>
                  
                  <div style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    fontSize: '0.75rem',
                    fontFamily: 'Fira Code, monospace'
                  }}>
                    <span style={{ color: '#00d4ff' }}>{hero.role.toUpperCase()}</span>
                    {hasBuild && (
                      <span style={{
                        background: 'rgba(0, 255, 136, 0.2)',
                        color: '#00ff88',
                        padding: '0.25rem 0.5rem',
                        borderRadius: '4px',
                        fontSize: '0.75rem',
                        fontWeight: 'bold',
                        border: '1px solid rgba(0, 255, 136, 0.5)'
                      }}>
                        BUILD
                      </span>
                    )}
                  </div>
                </div>
              );
            })}
          </div>

          {filteredHeroes.length === 0 && (
            <div style={{
              textAlign: 'center',
              padding: '3rem 0',
              color: 'rgba(255, 255, 255, 0.6)',
              fontFamily: 'Fira Code, monospace'
            }}>
              {t('hero.search.noResults')?.toUpperCase() || 'NO HEROES FOUND MATCHING CRITERIA'}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};