export interface Hero {
  id: string;
  name: string;
  role: string;
  difficulty: 'Easy' | 'Medium' | 'Hard';
  moods: Mood[];
  description: string;
  strengths: string[];
  weaknesses: string[];
}

export interface Item {
  id: string | number;
  name: string;
  cost: number;
  phase: 'Early' | 'Mid' | 'Late';
  priority: 'Core' | 'Situational' | 'Luxury';
  description: string;
}

export interface Build {
  id?: number;
  heroId: string;
  mood: Mood;
  score?: number;
  items: Item[];
  playstyle: {
    dos: string[];
    donts: string[];
    tips: string[];
  };
  gameplan: {
    early: string;
    mid: string;
    late: string;
  };
}

export type Mood = 'aggressive' | 'defensive' | 'experimental' | 'creative' | 'chaos';