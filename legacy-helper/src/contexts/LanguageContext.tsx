import React, { createContext, useContext, useState, ReactNode } from 'react';

export type Language = 'en' | 'ru' | 'uz';

interface LanguageContextType {
  language: Language;
  setLanguage: (lang: Language) => void;
  t: (key: string) => string;
}

const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

export const useLanguage = () => {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error('useLanguage must be used within a LanguageProvider');
  }
  return context;
};

interface LanguageProviderProps {
  children: ReactNode;
}

export const LanguageProvider: React.FC<LanguageProviderProps> = ({ children }) => {
  const [language, setLanguage] = useState<Language>('en');

  const t = (key: string): string => {
    return translations[language][key] || key;
  };

  return (
    <LanguageContext.Provider value={{ language, setLanguage, t }}>
      {children}
    </LanguageContext.Provider>
  );
};

const translations = {
  en: {
    // Header
    'app.title': 'CYBERSPYDE Plays Dota',
    'app.subtitle': 'Choose your mood and get a perfect hero recommendation with detailed build guides and gameplay strategies',
    
    // Mood Selection
    'mood.title': "What's your mood today?",
    'mood.aggressive': 'Aggressive',
    'mood.aggressive.desc': 'High-risk, high-reward playstyle',
    'mood.defensive': 'Defensive',
    'mood.defensive.desc': 'Safe, supportive gameplay',
    'mood.experimental': 'Experimental',
    'mood.experimental.desc': 'Try new builds and strategies',
    'mood.creative': 'Creative',
    'mood.creative.desc': 'Unique and unconventional approaches',
    'mood.chaos': 'Chaos',
    'mood.chaos.desc': 'Pure mayhem and unpredictability',
    
    // Hero Section
    'hero.recommendation': 'Your Hero Recommendation',
    'hero.selected': 'Selected Hero',
    'hero.reroll': 'Reroll',
    'hero.strengths': 'Strengths',
    'hero.weaknesses': 'Weaknesses',
    'hero.search.title': 'Hero Search',
    'hero.search.placeholder': 'Search heroes...',
    'hero.search.button': 'Search',
    'hero.search.change': 'Change Hero',
    'hero.search.allRoles': 'All Roles',
    'hero.search.allDifficulties': 'All Difficulties',
    'hero.search.heroesFound': 'heroes found',
    'hero.search.hasBuild': 'Has Build',
    'hero.search.noResults': 'No heroes found matching your search criteria.',
    
    // Build Guide
    'build.title': 'Build Guide',
    'build.items': 'Item Build',
    'build.playstyle': 'Playstyle',
    'build.gameplan': 'Game Plan',
    'build.progression': 'Item Progression',
    'build.dos': "Do's",
    'build.donts': "Don'ts",
    'build.tips': 'Pro Tips',
    'build.early': 'Early Game',
    'build.mid': 'Mid Game',
    'build.late': 'Late Game',
    'build.notAvailable': 'Build guide not available for this hero and mood combination yet.',
    'build.selectMood': 'Select a mood to see build guide',
    'build.availableFor': 'Available builds for this hero:',
    'build.noBuildForHero': 'No build guides available for this hero yet.',
    
    // Game Phases
    'phase.Early': 'Early',
    'phase.Mid': 'Mid',
    'phase.Late': 'Late',
    
    // Item Priority
    'priority.Core': 'Core',
    'priority.Situational': 'Situational',
    'priority.Luxury': 'Luxury',
    
    // Welcome Message
    'welcome.title': '🎮',
    'welcome.message': 'Select a mood above to get started with your personalized Dota 2 recommendation!',
    
    // Settings
    'settings.language': 'Language',
    'settings.theme': 'Theme',
    'theme.light': 'Light',
    'theme.dark': 'Dark',
    
    // Roles
    'role.Carry': 'Carry',
    'role.Support': 'Support',
    'role.Mid': 'Mid',
    'role.Initiator': 'Initiator',
    
    // Difficulties
    'difficulty.Easy': 'Easy',
    'difficulty.Medium': 'Medium',
    'difficulty.Hard': 'Hard'
  },
  ru: {
    // Header
    'app.title': 'CYBERSPYDE Играет в Dota',
    'app.subtitle': 'Выберите ваше настроение и получите идеальную рекомендацию героя с детальными гайдами по сборке и игровым стратегиям',
    
    // Mood Selection
    'mood.title': 'Какое у вас настроение сегодня?',
    'mood.aggressive': 'Агрессивный',
    'mood.aggressive.desc': 'Высокий риск, высокая награда',
    'mood.defensive': 'Защитный',
    'mood.defensive.desc': 'Безопасная, поддерживающая игра',
    'mood.experimental': 'Экспериментальный',
    'mood.experimental.desc': 'Попробуйте новые сборки и стратегии',
    'mood.creative': 'Творческий',
    'mood.creative.desc': 'Уникальные и нестандартные подходы',
    'mood.chaos': 'Хаос',
    'mood.chaos.desc': 'Чистый хаос и непредсказуемость',
    
    // Hero Section
    'hero.recommendation': 'Ваша рекомендация героя',
    'hero.selected': 'Выбранный герой',
    'hero.reroll': 'Перебросить',
    'hero.strengths': 'Сильные стороны',
    'hero.weaknesses': 'Слабые стороны',
    'hero.search.title': 'Поиск героев',
    'hero.search.placeholder': 'Поиск героев...',
    'hero.search.button': 'Поиск',
    'hero.search.change': 'Сменить героя',
    'hero.search.allRoles': 'Все роли',
    'hero.search.allDifficulties': 'Все сложности',
    'hero.search.heroesFound': 'героев найдено',
    'hero.search.hasBuild': 'Есть сборка',
    'hero.search.noResults': 'Герои не найдены по вашим критериям поиска.',
    
    // Build Guide
    'build.title': 'Гайд по сборке',
    'build.items': 'Сборка предметов',
    'build.playstyle': 'Стиль игры',
    'build.gameplan': 'План игры',
    'build.progression': 'Прогрессия предметов',
    'build.dos': 'Что делать',
    'build.donts': 'Чего не делать',
    'build.tips': 'Про советы',
    'build.early': 'Ранняя игра',
    'build.mid': 'Средняя игра',
    'build.late': 'Поздняя игра',
    'build.notAvailable': 'Гайд по сборке для этого героя и настроения еще недоступен.',
    'build.selectMood': 'Выберите настроение, чтобы увидеть гайд по сборке',
    'build.availableFor': 'Доступные сборки для этого героя:',
    'build.noBuildForHero': 'Гайды по сборке для этого героя еще недоступны.',
    
    // Game Phases
    'phase.Early': 'Рано',
    'phase.Mid': 'Середина',
    'phase.Late': 'Поздно',
    
    // Item Priority
    'priority.Core': 'Основной',
    'priority.Situational': 'Ситуационный',
    'priority.Luxury': 'Роскошь',
    
    // Welcome Message
    'welcome.title': '🎮',
    'welcome.message': 'Выберите настроение выше, чтобы начать с персональной рекомендации Dota 2!',
    
    // Settings
    'settings.language': 'Язык',
    'settings.theme': 'Тема',
    'theme.light': 'Светлая',
    'theme.dark': 'Тёмная',
    
    // Roles
    'role.Carry': 'Керри',
    'role.Support': 'Поддержка',
    'role.Mid': 'Мид',
    'role.Initiator': 'Инициатор',
    
    // Difficulties
    'difficulty.Easy': 'Легко',
    'difficulty.Medium': 'Средне',
    'difficulty.Hard': 'Сложно'
  },
  uz: {
    // Header
    'app.title': 'CYBERSPYDE Dota O\'ynaydi',
    'app.subtitle': 'Kayfiyatingizni tanlang va batafsil qurilish qo\'llanmalari va o\'yin strategiyalari bilan mukammal qahramon tavsiyasini oling',
    
    // Mood Selection
    'mood.title': 'Bugun kayfiyatingiz qanday?',
    'mood.aggressive': 'Tajovuzkor',
    'mood.aggressive.desc': 'Yuqori xavf, yuqori mukofot o\'yin uslubi',
    'mood.defensive': 'Himoyaviy',
    'mood.defensive.desc': 'Xavfsiz, qo\'llab-quvvatlovchi o\'yin',
    'mood.experimental': 'Eksperimental',
    'mood.experimental.desc': 'Yangi qurilishlar va strategiyalarni sinab ko\'ring',
    'mood.creative': 'Ijodiy',
    'mood.creative.desc': 'Noyob va an\'anaviy bo\'lmagan yondashuvlar',
    'mood.chaos': 'Xaos',
    'mood.chaos.desc': 'Sof tartibsizlik va oldindan aytib bo\'lmaslik',
    
    // Hero Section
    'hero.recommendation': 'Sizning qahramon tavsiyangiz',
    'hero.selected': 'Tanlangan qahramon',
    'hero.reroll': 'Qayta tashlash',
    'hero.strengths': 'Kuchli tomonlar',
    'hero.weaknesses': 'Zaif tomonlar',
    'hero.search.title': 'Qahramon qidirish',
    'hero.search.placeholder': 'Qahramonlarni qidirish...',
    'hero.search.button': 'Qidiruv',
    'hero.search.change': 'Qahramonni o\'zgartirish',
    'hero.search.allRoles': 'Barcha rollar',
    'hero.search.allDifficulties': 'Barcha qiyinchilik darajalari',
    'hero.search.heroesFound': 'qahramon topildi',
    'hero.search.hasBuild': 'Qurilish bor',
    'hero.search.noResults': 'Qidiruv mezonlaringizga mos qahramonlar topilmadi.',
    
    // Build Guide
    'build.title': 'Qurilish qo\'llanmasi',
    'build.items': 'Buyum qurilishi',
    'build.playstyle': 'O\'yin uslubi',
    'build.gameplan': 'O\'yin rejasi',
    'build.progression': 'Buyum progressiyasi',
    'build.dos': 'Nima qilish kerak',
    'build.donts': 'Nima qilmaslik kerak',
    'build.tips': 'Professional maslahatlar',
    'build.early': 'Erta o\'yin',
    'build.mid': 'O\'rta o\'yin',
    'build.late': 'Kech o\'yin',
    'build.notAvailable': 'Ushbu qahramon va kayfiyat uchun qurilish qo\'llanmasi hali mavjud emas.',
    'build.selectMood': 'Qurilish qo\'llanmasini ko\'rish uchun kayfiyatni tanlang',
    'build.availableFor': 'Ushbu qahramon uchun mavjud qurilishlar:',
    'build.noBuildForHero': 'Ushbu qahramon uchun qurilish qo\'llanmalari hali mavjud emas.',
    
    // Game Phases
    'phase.Early': 'Erta',
    'phase.Mid': 'O\'rta',
    'phase.Late': 'Kech',
    
    // Item Priority
    'priority.Core': 'Asosiy',
    'priority.Situational': 'Vaziyatga qarab',
    'priority.Luxury': 'Hashamat',
    
    // Welcome Message
    'welcome.title': '🎮',
    'welcome.message': 'Shaxsiy Dota 2 tavsiyangizni boshlash uchun yuqorida kayfiyat tanlang!',
    
    // Settings
    'settings.language': 'Til',
    'settings.theme': 'Mavzu',
    'theme.light': 'Yorug\'',
    'theme.dark': 'Qorong\'u',
    
    // Roles
    'role.Carry': 'Kerri',
    'role.Support': 'Qo\'llab-quvvatlash',
    'role.Mid': 'Mid',
    'role.Initiator': 'Boshlang\'ich',
    
    // Difficulties
    'difficulty.Easy': 'Oson',
    'difficulty.Medium': 'O\'rta',
    'difficulty.Hard': 'Qiyin'
  }
};