import { Hero, Build } from '../src/types';

// ==============================================
// HERO TEMPLATE
// ==============================================
export const heroTemplate: Hero = {
  id: 'hero-id',                    // Unique identifier (lowercase, use dashes)
  name: 'Hero Name',                // Display name
  role: 'Carry',                    // Role: 'Carry' | 'Support' | 'Mid' | 'Initiator'
  difficulty: 'Medium',             // Difficulty: 'Easy' | 'Medium' | 'Hard'
  moods: ['aggressive', 'experimental'], // Available moods for this hero
  description: 'Hero description goes here', // Short description
  strengths: [                      // 3-5 strengths
    'First strength',
    'Second strength',
    'Third strength'
  ],
  weaknesses: [                     // 3-5 weaknesses
    'First weakness',
    'Second weakness',
    'Third weakness'
  ]
};

// ==============================================
// BUILD TEMPLATE
// ==============================================
export const buildTemplate: Build = {
  heroId: 'hero-id',                // Must match a hero's id
  mood: 'aggressive',               // Mood: 'aggressive' | 'defensive' | 'experimental' | 'creative' | 'chaos'
  items: [                          // 4-8 items in order
    {
      id: 'boots',
      name: 'Boots of Speed',
      cost: 500,
      phase: 'Early',               // Phase: 'Early' | 'Mid' | 'Late'
      priority: 'Core',             // Priority: 'Core' | 'Situational' | 'Luxury'
      description: 'Basic mobility'
    },
    {
      id: 'wraith-band',
      name: 'Wraith Band',
      cost: 505,
      phase: 'Early',
      priority: 'Core',
      description: 'Early stats'
    },
    // Add more items...
  ],
  playstyle: {
    dos: [                          // 4-6 do's
      'First thing to do',
      'Second thing to do',
      'Third thing to do'
    ],
    donts: [                        // 4-6 don'ts
      'First thing not to do',
      'Second thing not to do',
      'Third thing not to do'
    ],
    tips: [                         // 4-6 tips
      'First pro tip',
      'Second pro tip',
      'Third pro tip'
    ]
  },
  gameplan: {
    early: 'Early game strategy and goals',
    mid: 'Mid game strategy and goals',
    late: 'Late game strategy and goals'
  }
};

// ==============================================
// REAL EXAMPLES
// ==============================================

// Example: Pudge Hero
export const pudgeHero: Hero = {
  id: 'pudge',
  name: 'Pudge',
  role: 'Support',
  difficulty: 'Easy',
  moods: ['aggressive', 'chaos', 'experimental'],
  description: 'A tanky support who excels at picking off enemies with hooks',
  strengths: [
    'High HP pool',
    'Game-changing hooks',
    'Strong initiation',
    'Grows stronger with kills'
  ],
  weaknesses: [
    'Low mobility',
    'Mana dependent',
    'Skill shot reliant',
    'Vulnerable to kiting'
  ]
};

// Example: Pudge Build
export const pudgeBuild: Build = {
  heroId: 'pudge',
  mood: 'aggressive',
  items: [
    { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
    { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana sustain for hooks' },
    { id: 'tranquil', name: 'Tranquil Boots', cost: 925, phase: 'Early', priority: 'Core', description: 'Health regen and movement' },
    { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Initiation and positioning' },
    { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced Dismember' },
    { id: 'heart', name: 'Heart of Tarrasque', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Massive HP and regen' }
  ],
  playstyle: {
    dos: [
      'Practice hook accuracy in practice mode',
      'Use Rot to slow enemies and farm',
      'Position behind trees for surprise hooks',
      'Build HP items to increase Flesh Heap stacks',
      'Use Blink Dagger for better positioning'
    ],
    donts: [
      "Don't waste hooks on creeps",
      "Don't hook enemies toward their team",
      "Don't neglect mana management",
      "Don't chase without backup",
      "Don't ignore last-hitting in lane"
    ],
    tips: [
      'Hook has longer range than it appears',
      'Rot damages both you and enemies',
      'Dismember heals you while channeling',
      'Each kill increases your HP permanently',
      'Use trees and fog to hide hook animations'
    ]
  },
  gameplan: {
    early: 'Lane with your carry, harass enemies with Rot, practice hooks, get Soul Ring for mana.',
    mid: 'Get Blink Dagger, start making plays around the map, hook key targets in team fights.',
    late: 'Become an unkillable tank, initiate fights with Blink + Hook, protect your carries.'
  }
};

// ==============================================
// AVAILABLE VALUES REFERENCE
// ==============================================

/*
ROLES:
- 'Carry'
- 'Support' 
- 'Mid'
- 'Initiator'

DIFFICULTIES:
- 'Easy'
- 'Medium'
- 'Hard'

MOODS:
- 'aggressive'
- 'defensive'
- 'experimental'
- 'creative'
- 'chaos'

ITEM PHASES:
- 'Early'
- 'Mid'
- 'Late'

ITEM PRIORITIES:
- 'Core'
- 'Situational'
- 'Luxury'
*/