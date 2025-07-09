import React, { useState } from 'react';
import { Mood, Hero, Build } from './types';
import { apiService } from './services/api';
import { MoodSelector } from './components/MoodSelector';
import { HeroCard } from './components/HeroCard';
import { BuildGuide } from './components/BuildGuide';
import { HeroSearch } from './components/HeroSearch';
import { LanguageSelector } from './components/LanguageSelector';
import { ThemeToggle } from './components/ThemeToggle';
import { LanguageProvider, useLanguage } from './contexts/LanguageContext';
import { ThemeProvider } from './contexts/ThemeContext';
import { ConnectionStatus } from './components/ConnectionStatus';
import { Gamepad2, Crown, RefreshCw, Search, Zap, Activity } from 'lucide-react';

function AppContent() {
  const [selectedMood, setSelectedMood] = useState<Mood | null>(null);
  const [currentHero, setCurrentHero] = useState<string | null>(null);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [heroes, setHeroes] = useState<Hero[]>([]);
  const [builds, setBuilds] = useState<Build[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { t } = useLanguage();

  // Load data on component mount
  React.useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const [heroesData, buildsData] = await Promise.all([
        apiService.getHeroes(),
        apiService.getBuilds()
      ]);
      
      setHeroes(heroesData);
      setBuilds(buildsData);
    } catch (err) {
      console.error('Failed to load data:', err);
      setError('Failed to load data. Please check your internet connection and try again.');
    } finally {
      setLoading(false);
    }
  };

  const getRandomHero = (mood: Mood) => {
    const moodHeroes = heroes.filter(hero => hero.moods.includes(mood));
    if (moodHeroes.length === 0) return null;
    return moodHeroes[Math.floor(Math.random() * moodHeroes.length)];
  };

  const handleMoodSelect = (mood: Mood) => {
    setSelectedMood(mood);
    const hero = getRandomHero(mood);
    setCurrentHero(hero?.id || null);
  };

  const handleHeroSelect = (hero: Hero) => {
    setCurrentHero(hero.id);
    
    // Get available builds for this hero
    const heroBuildMoods = builds.filter(b => b.heroId === hero.id).map(b => b.mood);
    
    // If hero has builds, automatically select the first mood to show the build
    if (heroBuildMoods.length > 0) {
      // If current mood has a build for this hero, keep it
      if (selectedMood && heroBuildMoods.includes(selectedMood)) {
        // Keep current mood
        return;
      }
      // Otherwise, auto-select the first available mood
      setSelectedMood(heroBuildMoods[0]);
    } else {
      // No builds available, clear mood selection
      setSelectedMood(null);
    }
  };

  const handleReroll = () => {
    if (selectedMood) {
      const hero = getRandomHero(selectedMood);
      setCurrentHero(hero?.id || null);
    }
  };

  const selectedHero = currentHero ? heroes.find(h => h.id === currentHero) : null;
  
  // Get available builds for current hero
  const availableBuilds = currentHero ? builds.filter(b => b.heroId === currentHero) : [];
  const selectedBuild = selectedMood ? availableBuilds.find(b => b.mood === selectedMood) : null;

  if (loading) {
    return (
      <div className="loading-screen">
        <div>
          <div className="loading-spinner">
            <div className="spinner spinner-1"></div>
            <div className="spinner spinner-2"></div>
          </div>
          <p className="loading-text">
            <Activity className="inline mr-2" size={20} />
            Initializing Dota 2 Neural Network...
          </p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error-screen">
        <div className="error-card card">
          <div className="error-icon">⚠️</div>
          <h2 className="error-title">CONNECTION ERROR</h2>
          <p className="error-message">{error}</p>
          <button onClick={loadData} className="btn btn-primary">
            <Zap size={16} />
            RETRY CONNECTION
          </button>
        </div>
      </div>
    );
  }

  return (
    <div style={{ minHeight: '100vh', paddingTop: '5rem' }}>
      <ConnectionStatus />
      
      <div className="container">
        <header className="header">
          <div className="header-controls">
            <button
              onClick={() => setIsSearchOpen(true)}
              className="btn btn-secondary"
              title={t('hero.search.button') || 'Search Heroes'}
            >
              <Search size={16} />
              <span>SEARCH</span>
            </button>
            <LanguageSelector />
            <ThemeToggle />
          </div>
          
          <div className="header-title">
            <Gamepad2 size={48} style={{ color: '#00ff88' }} />
            <h1 className="app-title">{t('app.title')}</h1>
            <Crown size={48} style={{ color: '#ffff00' }} />
          </div>
          
          <div className="subtitle">
            {">> NEURAL NETWORK ACTIVE // HERO ANALYSIS PROTOCOL ONLINE <<"}
          </div>
        </header>

        <HeroSearch 
          isOpen={isSearchOpen}
          onClose={() => setIsSearchOpen(false)}
          onHeroSelect={handleHeroSelect}
          heroes={heroes}
          builds={builds}
        />
        
        <div style={{ marginBottom: '3rem' }}>
          <h2 style={{ 
            fontSize: '2rem', 
            fontWeight: 'bold', 
            marginBottom: '1.5rem', 
            textAlign: 'center',
            color: '#00d4ff',
            textShadow: '0 0 20px rgba(0, 212, 255, 0.5)'
          }}>
            {t('mood.title')}
          </h2>
          <MoodSelector selectedMood={selectedMood} onMoodSelect={handleMoodSelect} />
        </div>

        {selectedHero && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '2rem' }}>
            <div style={{ 
              display: 'flex', 
              flexWrap: 'wrap',
              justifyContent: 'space-between', 
              alignItems: 'center',
              gap: '1rem'
            }}>
              <h2 style={{ 
                fontSize: '2rem', 
                fontWeight: 'bold',
                color: '#00ff88',
                textShadow: '0 0 20px rgba(0, 255, 136, 0.5)'
              }}>
                {selectedMood ? t('hero.recommendation') : t('hero.selected') || 'SELECTED HERO'}
              </h2>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
                {selectedMood && (
                  <button onClick={handleReroll} className="btn btn-primary">
                    <RefreshCw size={16} />
                    REROLL
                  </button>
                )}
                <button onClick={() => setIsSearchOpen(true)} className="btn btn-secondary">
                  <Search size={16} />
                  CHANGE
                </button>
              </div>
            </div>
            
            <HeroCard hero={selectedHero} />

            {selectedBuild && (
              <div>
                <h2 style={{ 
                  fontSize: '2rem', 
                  fontWeight: 'bold', 
                  marginBottom: '1.5rem',
                  color: '#8b5cf6',
                  textShadow: '0 0 20px rgba(139, 92, 246, 0.5)'
                }}>
                  {t('build.title')}
                </h2>
                <BuildGuide build={selectedBuild} />
              </div>
            )}
            
            {currentHero && availableBuilds.length > 0 && !selectedMood && (
              <div className="card" style={{ padding: '1.5rem', borderColor: 'rgba(255, 255, 0, 0.5)' }}>
                <h3 style={{ 
                  fontSize: '1.25rem', 
                  fontWeight: 'bold',
                  color: '#ffff00',
                  marginBottom: '1rem'
                }}>
                  {t('build.selectMood') || 'SELECT COMBAT PROTOCOL'}
                </h3>
                <div style={{ 
                  fontSize: '0.875rem',
                  color: 'rgba(255, 255, 0, 0.8)',
                  marginBottom: '1rem',
                  fontFamily: 'Fira Code, monospace'
                }}>
                  {t('build.availableFor') || 'Available protocols for this hero:'}
                </div>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem' }}>
                  {availableBuilds.map(build => (
                    <button
                      key={build.mood}
                      onClick={() => setSelectedMood(build.mood)}
                      className="btn btn-primary"
                      style={{ fontSize: '0.875rem', padding: '0.5rem 1rem' }}
                    >
                      {t(`mood.${build.mood}`) || build.mood}
                    </button>
                  ))}
                </div>
              </div>
            )}
            
            {currentHero && availableBuilds.length === 0 && (
              <div className="card" style={{ padding: '1.5rem', borderColor: 'rgba(255, 102, 0, 0.5)' }}>
                <p style={{ 
                  color: '#ff6600',
                  fontFamily: 'Fira Code, monospace',
                  textAlign: 'center'
                }}>
                  {t('build.noBuildForHero') || 'NO BUILD PROTOCOLS AVAILABLE FOR THIS HERO'}
                </p>
              </div>
            )}
          </div>
        )}

        {!selectedMood && !currentHero && (
          <div className="welcome-screen">
            <div className="welcome-icon">🎮</div>
            <div className="welcome-message">
              {t('welcome.message') || 'SELECT YOUR COMBAT MOOD TO BEGIN HERO ANALYSIS'}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <LanguageProvider>
        <AppContent />
      </LanguageProvider>
    </ThemeProvider>
  );
}

export default App;