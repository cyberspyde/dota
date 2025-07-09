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
import { Gamepad2, Crown, RefreshCw, Search } from 'lucide-react';

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
      setError('Failed to load data. Please make sure the server is running.');
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
      <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-indigo-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 text-gray-900 dark:text-white transition-colors duration-300">
        <div className="container mx-auto px-4 py-8 flex items-center justify-center min-h-screen">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <p className="text-lg text-gray-600 dark:text-gray-400">Loading Dota 2 data...</p>
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    const isDevelopment = process.env.NODE_ENV === 'development';
    
    return (
      <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-indigo-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 text-gray-900 dark:text-white transition-colors duration-300">
        <div className="container mx-auto px-4 py-8 flex items-center justify-center min-h-screen">
          <div className="text-center">
            <div className="text-6xl mb-4">⚠️</div>
            <h2 className="text-2xl font-bold mb-4 text-red-600 dark:text-red-400">Connection Error</h2>
            <p className="text-lg text-gray-600 dark:text-gray-400 mb-6">{error}</p>
            <button
              onClick={loadData}
              className="bg-blue-500 hover:bg-blue-600 text-white px-6 py-3 rounded-lg font-medium transition-colors"
            >
              Try Again
            </button>
            {isDevelopment ? (
              <div className="mt-4 text-sm text-gray-500 dark:text-gray-400">
                Make sure to run: <code className="bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded">npm run dev:server</code>
              </div>
            ) : (
              <div className="mt-4 text-sm text-gray-500 dark:text-gray-400">
                <p>This might be a temporary issue. Please try again in a few moments.</p>
                <p className="mt-2">If the problem persists, check the Netlify function logs.</p>
              </div>
            )}
          </div>
        </div>
      </div>
    );
  }
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-indigo-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 text-gray-900 dark:text-white transition-colors duration-300">
      <div className="container mx-auto px-4 py-8">
        <header className="text-center mb-12">
          <div className="flex items-center justify-center space-x-2 mb-8">
            <button
              onClick={() => setIsSearchOpen(true)}
              className="flex items-center space-x-1 lg:space-x-2 px-2 lg:px-3 py-2 rounded-lg bg-white/10 dark:bg-gray-800/50 border border-gray-300 dark:border-gray-700 hover:bg-white/20 dark:hover:bg-gray-700/50 transition-colors"
              title={t('hero.search.button') || 'Search Heroes'}
            >
              <Search size={16} className="text-blue-600 dark:text-blue-400 lg:w-[18px] lg:h-[18px]" />
              <span className="text-xs lg:text-sm font-medium text-gray-800 dark:text-gray-200 hidden sm:inline">
                {t('hero.search.button') || 'Search'}
              </span>
            </button>
            <LanguageSelector />
            <ThemeToggle />
          </div>
          
          <div className="flex items-center justify-center space-x-3 mb-6">
            <Gamepad2 size={40} className="text-blue-600 dark:text-blue-400 lg:w-12 lg:h-12" />
            <h1 className="text-2xl sm:text-3xl lg:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 dark:from-blue-400 dark:to-purple-400 bg-clip-text text-transparent">
              {t('app.title')}
            </h1>
            <Crown size={40} className="text-blue-600 dark:text-blue-400 lg:w-12 lg:h-12" />
          </div>
        </header>

        <div className="max-w-6xl mx-auto">
          <HeroSearch 
            isOpen={isSearchOpen}
            onClose={() => setIsSearchOpen(false)}
            onHeroSelect={handleHeroSelect}
            heroes={heroes}
            builds={builds}
          />
          
          <div className="mb-8">
            <h2 className="text-xl lg:text-2xl font-bold mb-4 text-center text-gray-800 dark:text-white">{t('mood.title')}</h2>
            <MoodSelector selectedMood={selectedMood} onMoodSelect={handleMoodSelect} />
          </div>

          {selectedHero && (
            <div className="space-y-8">
              <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between space-y-4 sm:space-y-0">
                <h2 className="text-xl lg:text-2xl font-bold text-gray-800 dark:text-white">
                  {selectedMood ? t('hero.recommendation') : t('hero.selected') || 'Selected Hero'}
                </h2>
                <div className="flex items-center space-x-2">
                  {selectedMood && (
                    <button
                      onClick={handleReroll}
                      className="flex items-center space-x-2 bg-blue-500/20 hover:bg-blue-500/30 text-blue-600 dark:text-blue-400 px-3 py-2 lg:px-4 lg:py-2 rounded-lg transition-colors border border-blue-300 dark:border-blue-600 text-sm lg:text-base"
                    >
                      <RefreshCw size={16} className="lg:w-[18px] lg:h-[18px]" />
                      <span>{t('hero.reroll')}</span>
                    </button>
                  )}
                  <button
                    onClick={() => setIsSearchOpen(true)}
                    className="flex items-center space-x-2 bg-green-500/20 hover:bg-green-500/30 text-green-600 dark:text-green-400 px-3 py-2 lg:px-4 lg:py-2 rounded-lg transition-colors border border-green-300 dark:border-green-600 text-sm lg:text-base"
                  >
                    <Search size={16} className="lg:w-[18px] lg:h-[18px]" />
                    <span>{t('hero.search.change') || 'Change Hero'}</span>
                  </button>
                </div>
              </div>
              
              <HeroCard hero={selectedHero} />

              {selectedBuild && (
                <div>
                  <h2 className="text-xl lg:text-2xl font-bold mb-4 text-gray-800 dark:text-white">{t('build.title')}</h2>
                  <BuildGuide build={selectedBuild} />
                </div>
              )}
              
              {currentHero && availableBuilds.length > 0 && !selectedMood && (
                <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
                  <h3 className="text-lg font-semibold text-blue-800 dark:text-blue-200 mb-3">
                    {t('build.selectMood') || 'Select a mood to see build guide'}
                  </h3>
                  <div className="text-sm text-blue-700 dark:text-blue-300 mb-3">
                    {t('build.availableFor') || 'Available builds for this hero:'}
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {availableBuilds.map(build => (
                      <button
                        key={build.mood}
                        onClick={() => setSelectedMood(build.mood)}
                        className="px-3 py-2 bg-blue-500/20 hover:bg-blue-500/30 text-blue-600 dark:text-blue-400 rounded-lg border border-blue-300 dark:border-blue-600 text-sm font-medium transition-colors"
                      >
                        {t(`mood.${build.mood}`) || build.mood}
                      </button>
                    ))}
                  </div>
                </div>
              )}
              
              {currentHero && availableBuilds.length === 0 && (
                <div className="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4 text-center">
                  <p className="text-yellow-800 dark:text-yellow-200">
                    {t('build.noBuildForHero') || 'No build guides available for this hero yet.'}
                  </p>
                </div>
              )}
            </div>
          )}

          {!selectedMood && !currentHero && (
            <div className="text-center py-8 lg:py-12 px-4">
              <div className="text-4xl lg:text-6xl mb-4">{t('welcome.title')}</div>
              <p className="text-base lg:text-xl text-gray-600 dark:text-gray-400">
                {t('welcome.message')}
              </p>
            </div>
          )}
        </div>
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