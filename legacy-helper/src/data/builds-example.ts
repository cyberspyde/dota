import { Build } from '../types';

export const builds: Build[] = [
  // STRENGTH HEROES
  
  // Abaddon builds
  {
    heroId: 'abaddon',
    mood: 'defensive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana sustain for spells' },
      { id: 'tranquil', name: 'Tranquil Boots', cost: 925, phase: 'Early', priority: 'Core', description: 'Health regen and movement' },
      { id: 'mek', name: 'Mekansm', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Team healing' },
      { id: 'pipe', name: 'Pipe of Insight', cost: 3100, phase: 'Mid', priority: 'Core', description: 'Magic resistance for team' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Late', priority: 'Luxury', description: 'Enhanced abilities' }
    ],
    playstyle: {
      dos: [
        'Shield allies with Aphotic Shield',
        'Use Borrowed Time to tank damage',
        'Build aura items for team',
        'Stay close to your carry'
      ],
      donts: [
        "Don't waste shields on yourself",
        "Don't trigger Borrowed Time too early",
        "Don't neglect mana management",
        "Don't abandon your team"
      ],
      tips: [
        'Pre-cast shield before fights',
        'Use Mist Coil to heal or nuke',
        'Time Borrowed Time for maximum impact',
        'Position to protect multiple allies'
      ]
    },
    gameplan: {
      early: 'Focus on protecting your carry in lane, use shield to block harassment',
      mid: 'Build team items, group with team for fights',
      late: 'Become the ultimate team protector with defensive items'
    }
  }
]