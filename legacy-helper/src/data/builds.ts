import { Build } from '../types';

export const builds: Build[] = [
  // STRENGTH HEROES
  
  // Undying builds
  {
    heroId: 'undying',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana sustain for spells' },
      { id: 'arcane', name: 'Arcane Boots', cost: 1300, phase: 'Early', priority: 'Core', description: 'Team mana support' },
      { id: 'mek', name: 'Mekansm', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Team healing and armor' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced Tombstone' },
      { id: 'shivas', name: "Shiva's Guard", cost: 4850, phase: 'Late', priority: 'Luxury', description: 'AOE slow and armor' }
    ],
    playstyle: {
      dos: [
        'Place Tombstone in strategic locations',
        'Use Decay to steal strength early',
        'Group with team for teamfights',
        'Build aura items for team support'
      ],
      donts: [
        "Don't place Tombstone in easily accessible spots",
        "Don't waste Decay on creeps",
        "Don't neglect positioning in fights",
        "Don't build carry items"
      ],
      tips: [
        'Decay enemies before fights to reduce their HP',
        'Use Soul Rip to heal allies or damage enemies',
        'Place Tombstone behind your team',
        'Coordinate with team for maximum Tombstone value'
      ]
    },
    gameplan: {
      early: 'Harass with Decay, steal strength, protect your carry',
      mid: 'Group with team, use Tombstone in teamfights, build support items',
      late: 'Focus on team utility, use enhanced Tombstone strategically'
    }
  },

  // AGILITY HEROES
  
  // Weaver builds
  {
    heroId: 'weaver',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'wraith-band', name: 'Wraith Band', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats' },
      { id: 'power-treads', name: 'Power Treads', cost: 1400, phase: 'Early', priority: 'Core', description: 'Attack speed and stats' },
      { id: 'maelstrom', name: 'Maelstrom', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Farming and damage' },
      { id: 'desolator', name: 'Desolator', cost: 3500, phase: 'Mid', priority: 'Core', description: 'Armor reduction' },
      { id: 'butterfly', name: 'Butterfly', cost: 5525, phase: 'Late', priority: 'Luxury', description: 'Evasion and attack speed' }
    ],
    playstyle: {
      dos: [
        'Use Shukuchi for mobility and damage',
        'Time Lapse to escape dangerous situations',
        'Split push with Geminate Attack',
        'Build damage items for scaling'
      ],
      donts: [
        "Don't waste Time Lapse on minor damage",
        "Don't get caught without escape options",
        "Don't fight without proper items",
        "Don't neglect farming"
      ],
      tips: [
        'Use Shukuchi to scout and escape',
        'Time Lapse can reset negative effects',
        'Geminate Attack works on buildings',
        'Position aggressively but always have escape plan'
      ]
    },
    gameplan: {
      early: 'Farm safely, use Shukuchi for harassment and escape',
      mid: 'Split push, join fights when advantageous, build damage items',
      late: 'Become a major damage dealer, use mobility to control fights'
    }
  },

  // INTELLIGENCE HEROES
  
  // Lina builds
  {
    heroId: 'lina',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Early mana and damage' },
      { id: 'power-treads', name: 'Power Treads', cost: 1400, phase: 'Early', priority: 'Core', description: 'Attack speed and stats' },
      { id: 'maelstrom', name: 'Maelstrom', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Farming and chain lightning' },
      { id: 'bloodthorn', name: 'Bloodthorn', cost: 6800, phase: 'Late', priority: 'Core', description: 'Silence and critical hits' },
      { id: 'satanic', name: 'Satanic', cost: 5050, phase: 'Late', priority: 'Luxury', description: 'Lifesteal and survivability' }
    ],
    playstyle: {
      dos: [
        'Stack Fiery Soul for attack speed',
        'Use Dragon Slave for farming and harass',
        'Build hybrid damage items',
        'Position aggressively in fights'
      ],
      donts: [
        "Don't waste Laguna Blade on tanks",
        "Don't neglect positioning",
        "Don't build pure support items",
        "Don't forget to stack Fiery Soul"
      ],
      tips: [
        'Use Dragon Slave to stack Fiery Soul',
        'Laguna Blade pierces magic immunity',
        'Light Strike Array has long range',
        'Build both magical and physical damage'
      ]
    },
    gameplan: {
      early: 'Harass with Dragon Slave, stack Fiery Soul, farm efficiently',
      mid: 'Build damage items, join fights, use Laguna Blade on priority targets',
      late: 'Become a major damage dealer, use Bloodthorn for control'
    }
  },

  // Storm Spirit builds
  {
    heroId: 'storm-spirit',
    mood: 'creative',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Early mana and damage' },
      { id: 'power-treads', name: 'Power Treads', cost: 1400, phase: 'Early', priority: 'Core', description: 'Attack speed and stats' },
      { id: 'orchid', name: 'Orchid Malevolence', cost: 3475, phase: 'Mid', priority: 'Core', description: 'Silence and damage amplification' },
      { id: 'bloodstone', name: 'Bloodstone', cost: 4600, phase: 'Mid', priority: 'Core', description: 'Mana sustain and survivability' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Late', priority: 'Luxury', description: 'Enhanced Ball Lightning' }
    ],
    playstyle: {
      dos: [
        'Use Ball Lightning for mobility and damage',
        'Manage mana carefully',
        'Gank with Electric Vortex',
        'Build mana and damage items'
      ],
      donts: [
        "Don't waste mana on unnecessary Ball Lightning",
        "Don't get caught without mana",
        "Don't neglect farming",
        "Don't fight without proper items"
      ],
      tips: [
        'Ball Lightning damage scales with distance',
        'Electric Vortex can interrupt channeling',
        'Overload provides bonus damage',
        'Always keep mana for escape'
      ]
    },
    gameplan: {
      early: 'Farm safely, manage mana, use Static Remnant for harass',
      mid: 'Gank with Orchid, control fights with Electric Vortex, build mana items',
      late: 'Use Ball Lightning for mobility and damage, control teamfights'
    }
  },

  // Rubick builds
  {
    heroId: 'rubick',
    mood: 'creative',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Early mana and damage' },
      { id: 'arcane', name: 'Arcane Boots', cost: 1300, phase: 'Early', priority: 'Core', description: 'Team mana support' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Positioning and initiation' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced Spell Steal' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction and spell lifesteal' }
    ],
    playstyle: {
      dos: [
        'Steal powerful enemy spells',
        'Use Telekinesis for positioning',
        'Build utility and positioning items',
        'Stay alive to steal more spells'
      ],
      donts: [
        "Don't steal weak spells",
        "Don't waste Telekinesis",
        "Don't build carry items",
        "Don't die early in fights"
      ],
      tips: [
        'Steal spells that synergize with your team',
        'Telekinesis can interrupt channeling',
        'Fade Bolt reduces enemy damage',
        'Position to steal the most impactful spells'
      ]
    },
    gameplan: {
      early: 'Support your team, steal useful spells, build positioning items',
      mid: 'Use stolen spells effectively, control fights with Telekinesis',
      late: 'Become a major teamfight controller with multiple stolen spells'
    }
  },

  // STRENGTH HEROES
  
  // Mars builds
  {
    heroId: 'mars',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bracer', name: 'Bracer', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats and survivability' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Mobility and armor' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Initiation and positioning' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced Arena of Blood' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Luxury', description: 'Team armor and attack speed' }
    ],
    playstyle: {
      dos: [
        'Use Spear of Mars for initiation',
        'Create Arena of Blood for teamfights',
        'Build tanky and utility items',
        'Position to protect your team'
      ],
      donts: [
        "Don't waste Arena of Blood",
        "Don't initiate without team follow-up",
        "Don't build pure damage items",
        "Don't neglect positioning"
      ],
      tips: [
        'Spear of Mars can stun multiple enemies',
        'Arena of Blood blocks projectiles',
        'God\'s Rebuke pushes enemies back',
        'Use Bulwark to tank damage'
      ]
    },
    gameplan: {
      early: 'Harass with Spear of Mars, build tanky items, protect your carry',
      mid: 'Initiate fights with Blink and Spear, create Arena for teamfights',
      late: 'Become a major teamfight controller, use enhanced Arena strategically'
    }
  },

  // AGILITY HEROES
  
  // Monkey King builds
  {
    heroId: 'monkey-king',
    mood: 'creative',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'wraith-band', name: 'Wraith Band', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats' },
      { id: 'power-treads', name: 'Power Treads', cost: 1400, phase: 'Early', priority: 'Core', description: 'Attack speed and stats' },
      { id: 'diffusal', name: 'Diffusal Blade', cost: 2500, phase: 'Mid', priority: 'Core', description: 'Mana burn and agility' },
      { id: 'basher', name: 'Skull Basher', cost: 2875, phase: 'Mid', priority: 'Core', description: 'Bash and damage' },
      { id: 'abyssal', name: 'Abyssal Blade', cost: 6400, phase: 'Late', priority: 'Luxury', description: 'Stun and damage' }
    ],
    playstyle: {
      dos: [
        'Use Tree Dance for mobility and vision',
        'Stack Jingu Mastery for bonus damage',
        'Use Wukong\'s Command in teamfights',
        'Build damage and control items'
      ],
      donts: [
        "Don't waste Tree Dance charges",
        "Don't fight without Jingu Mastery stacks",
        "Don't neglect farming",
        "Don't use Wukong's Command poorly"
      ],
      tips: [
        'Tree Dance provides vision and escape',
        'Jingu Mastery gives lifesteal and damage',
        'Boundless Strike can stun multiple enemies',
        'Wukong\'s Command creates illusions'
      ]
    },
    gameplan: {
      early: 'Harass with Jingu Mastery, use Tree Dance for mobility, farm efficiently',
      mid: 'Build damage items, join fights with Wukong\'s Command, control with Boundless Strike',
      late: 'Become a major damage dealer, use Abyssal for control'
    }
  },

  // INTELLIGENCE HEROES
  
  // Grimstroke builds
  {
    heroId: 'grimstroke',
    mood: 'experimental',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Early mana and damage' },
      { id: 'arcane', name: 'Arcane Boots', cost: 1300, phase: 'Early', priority: 'Core', description: 'Team mana support' },
      { id: 'glimmer', name: 'Glimmer Cape', cost: 1950, phase: 'Mid', priority: 'Core', description: 'Save allies and escape' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced Soulbind' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction and spell lifesteal' }
    ],
    playstyle: {
      dos: [
        'Use Soulbind to link enemies',
        'Save allies with Stroke of Fate',
        'Use Phantom\'s Embrace for silence',
        'Build utility and save items'
      ],
      donts: [
        "Don't waste Soulbind on single targets",
        "Don't neglect positioning",
        "Don't build carry items",
        "Don't forget to save allies"
      ],
      tips: [
        'Soulbind can link multiple enemies',
        'Stroke of Fate can save allies from afar',
        'Phantom\'s Embrace provides vision',
        'Ink Swell can save allies or damage enemies'
      ]
    },
    gameplan: {
      early: 'Support your team, use Stroke of Fate for harass and save',
      mid: 'Use Soulbind in teamfights, save allies with Glimmer, build utility items',
      late: 'Become a major teamfight controller with enhanced Soulbind'
    }
  }
];
