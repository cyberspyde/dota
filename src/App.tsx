import React, { useState, useEffect, useMemo } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
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
import { Admin } from './components/Admin';
import { Gamepad2, Crown, RefreshCw, Search, Zap, Activity, ChevronLeft, MoreHorizontal, SlidersHorizontal } from 'lucide-react';

const PAGE_SIZE = 12;

function MainApp() {
  const [selectedMood, setSelectedMood] = useState<Mood | null>(null);
  const [currentHero, setCurrentHero] = useState<string | null>(null);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [heroes, setHeroes] = useState<Hero[]>([]);
  const [searchHeroes, setSearchHeroes] = useState<Hero[]>([]);
  const [totalHeroes, setTotalHeroes] = useState(0);
  const [offset, setOffset] = useState(0);
  const [builds, setBuilds] = useState<Build[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { t } = useLanguage();

  const heroAverageScores = useMemo(() => {
    if (!builds.length) return {};
    const scores: { [heroId: string]: { totalScore: number; count: number } } = {};
    
    for (const build of builds) {
      if (!scores[build.heroId]) {
        scores[build.heroId] = { totalScore: 0, count: 0 };
      }
      if (build.score !== null && build.score !== undefined) {
        scores[build.heroId].totalScore += build.score;
        scores[build.heroId].count++;
      }
    }
    
    const averages: { [heroId: string]: number } = {};
    for (const heroId in scores) {
      if (scores[heroId].count > 0) {
        averages[heroId] = scores[heroId].totalScore / scores[heroId].count;
      }
    }
    return averages;
  }, [builds]);

  useEffect(() => {
    loadInitialData();
  }, []);

  // Load all heroes for search
  const loadAllHeroes = async () => {
    try {
      const heroesResult = await apiService.getHeroes(1000, 0); // Get all heroes
      return heroesResult.heroes;
    } catch (err) {
      console.error('Failed to load all heroes:', err);
      return [];
    }
  };

  const loadInitialData = async () => {
    try {
      setLoading(true);
      setError(null);
      const [buildsData, heroesResult] = await Promise.all([
        apiService.getBuilds(),
        apiService.getHeroes(PAGE_SIZE, 0)
      ]);
      setBuilds(buildsData);
      setHeroes(heroesResult.heroes);
      setTotalHeroes(heroesResult.total);
      setOffset(PAGE_SIZE);
    } catch (err) {
      console.error('Failed to load data:', err);
      setError('Failed to load data. Please check your internet connection and try again.');
    } finally {
      setLoading(false);
    }
  };
  
  const handleMoodSelect = async (mood: Mood) => {
    if (selectedMood === mood) {
      // Deselect mood and load all heroes
      setSelectedMood(null);
      setHeroes([]);
      setOffset(0);
      await loadInitialData();
      return;
    }

    setSelectedMood(mood);
    setCurrentHero(null);
    setLoading(true);
    
    try {
      const result = await apiService.getHeroes(PAGE_SIZE, 0, mood);
      setHeroes(result.heroes);
      setTotalHeroes(result.total);
      setOffset(PAGE_SIZE);
    } catch (err) {
      console.error('Failed to filter heroes by mood:', err);
      setError('Failed to filter heroes. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleLoadMore = async () => {
    if (loadingMore || heroes.length >= totalHeroes) return;
    
    setLoadingMore(true);
    try {
      const result = await apiService.getHeroes(PAGE_SIZE, offset, selectedMood || undefined);
      setHeroes(prev => [...prev, ...result.heroes]);
      setOffset(prev => prev + PAGE_SIZE);
      setTotalHeroes(result.total);
    } catch (err) {
      console.error('Failed to load more heroes:', err);
    } finally {
      setLoadingMore(false);
    }
  }

  const handleHeroSelect = (hero: Hero) => {
    setCurrentHero(hero.id);
    
    // Find all builds for this hero
    const heroBuilds = builds.filter(b => b.heroId === hero.id);
    
    if (heroBuilds.length > 0) {
      // If there's a build for the current mood, keep it
      if (!selectedMood || !heroBuilds.some(b => b.mood === selectedMood)) {
        // Otherwise, select the first available mood for this hero
        setSelectedMood(heroBuilds[0].mood);
      }
    }

    // Add hero to main list if not already there
    if (!heroes.some(h => h.id === hero.id)) {
      setHeroes(prev => [...prev, hero]);
    }
  };

  const handleReroll = () => {
    if (heroes.length > 0) {
      const hero = heroes[Math.floor(Math.random() * heroes.length)];
      handleHeroSelect(hero); // Use the same logic for reroll
    }
  };
  
  const handleBack = () => {
    setCurrentHero(null);
  };

  const selectedHero = currentHero 
    ? (heroes.find(h => h.id === currentHero) || searchHeroes.find(h => h.id === currentHero))
    : null;
  const selectedBuild = selectedHero && selectedMood ? builds.find(b => b.heroId === selectedHero.id && b.mood === selectedMood) : null;

  if (loading && heroes.length === 0) {
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
          <button onClick={loadInitialData} className="btn btn-primary">
            <Zap size={16} />
            RETRY CONNECTION
          </button>
        </div>
      </div>
    );
  }

  if (selectedHero) {
    return (
      <div style={{ minHeight: '100vh', paddingTop: '5rem' }}>
        <ConnectionStatus />
        <div className="container">
          <button onClick={handleBack} className="btn btn-secondary mb-8">
            <ChevronLeft size={16} />
            BACK TO HERO GRID
          </button>
          <HeroCard hero={selectedHero} />
          {selectedBuild && <BuildGuide build={selectedBuild} />}
          {!selectedBuild && (
            <div className="card mt-8 p-8 text-center">
              <p>No build available for this hero and mood combination.</p>
            </div>
          )}
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
              onClick={async () => {
                const allHeroes = await loadAllHeroes();
                setSearchHeroes(allHeroes);
                setIsSearchOpen(true);
              }}
              className="btn btn-secondary"
              title={t('hero.search.button') || 'Search Heroes'}
            >
              <Search size={16} />
              <span>SEARCH</span>
            </button>
            <Link to="/admin" className="btn btn-secondary" title="Admin Dashboard">
              <SlidersHorizontal size={16} />
              <span>ADMIN</span>
            </Link>
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
          heroes={searchHeroes}
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

        {!selectedMood ? (
          <div className="welcome-screen">
            <div className="welcome-icon">🤔</div>
            <div className="welcome-message">
              Please select a mood to display corresponding heroes.
            </div>
          </div>
        ) : (
          <>
            <div className="hero-grid">
              {heroes.map(hero => {
                const buildScore = builds.find(b => b.heroId === hero.id && b.mood === selectedMood)?.score;
                return (
                  <div key={hero.id} onClick={() => handleHeroSelect(hero)} style={{ cursor: 'pointer' }}>
                    <HeroCard hero={hero} score={buildScore} />
                  </div>
                );
              })}
            </div>
            {loadingMore && <div className="loading-text text-center my-4">LOADING...</div>}
            {!loading && heroes.length < totalHeroes && (
              <div style={{ display: 'flex', justifyContent: 'center', marginTop: '2rem' }}>
                <button onClick={handleLoadMore} className="btn btn-primary" disabled={loadingMore}>
                  <MoreHorizontal size={16} />
                  {loadingMore ? 'LOADING...' : 'LOAD MORE HEROES'}
                </button>
              </div>
            )}
            {heroes.length === 0 && !loading && (
              <div className="welcome-screen">
                <div className="welcome-icon">🤔</div>
                <div className="welcome-message">
                  {`NO HEROES FOUND FOR '${(selectedMood || '').toUpperCase()}' PROTOCOL. TRY ANOTHER.`}
                </div>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <LanguageProvider>
        <Routes>
          <Route path="/" element={<MainApp />} />
          <Route path="/admin" element={<Admin />} />
        </Routes>
      </LanguageProvider>
    </ThemeProvider>
  );
}

export default App;