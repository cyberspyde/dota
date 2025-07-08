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
  },
  {
    heroId: 'abaddon',
    mood: 'experimental',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Damage and mobility' },
      { id: 'radiance', name: 'Radiance', cost: 5150, phase: 'Mid', priority: 'Situational', description: 'Experimental damage aura' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Mid', priority: 'Core', description: 'Cooldown reduction' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Late', priority: 'Luxury', description: 'Maximum survivability' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double ultimates' }
    ],
    playstyle: {
      dos: [
        'Experiment with carry Abaddon',
        'Use Borrowed Time aggressively',
        'Build unusual damage items',
        'Focus on sustained fights'
      ],
      donts: [
        "Don't ignore your support potential",
        "Don't build glass cannon",
        "Don't forget team utility",
        "Don't abandon core items completely"
      ],
      tips: [
        'Radiance works well with Borrowed Time',
        'Use Octarine for more shield uptime',
        'Experiment with different skill builds',
        'Try carry builds in appropriate games'
      ]
    },
    gameplan: {
      early: 'Farm aggressively while maintaining support potential',
      mid: 'Build unconventional items while staying useful',
      late: 'Become an unkillable damage dealer'
    }
  },

  // Alchemist builds
  {
    heroId: 'alchemist',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana for Acid Spray' },
      { id: 'radiance', name: 'Radiance', cost: 5150, phase: 'Mid', priority: 'Core', description: 'Fast farming and damage' },
      { id: 'travels', name: 'Boots of Travel', cost: 2500, phase: 'Mid', priority: 'Core', description: 'Global presence' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Core', description: 'Attack speed and armor' },
      { id: 'abyssal', name: 'Abyssal Blade', cost: 6250, phase: 'Late', priority: 'Luxury', description: 'Lockdown and damage' }
    ],
    playstyle: {
      dos: [
        'Farm aggressively with Greevils Greed',
        'Use Acid Spray to farm stacks',
        'Look for early fight participation',
        'Abuse your fast farming speed'
      ],
      donts: [
        "Don't neglect early fighting",
        "Don't waste Chemical Rage charges",
        "Don't ignore team when farming",
        "Don't build too greedily"
      ],
      tips: [
        'Stack camps for yourself',
        'Use Chemical Rage for sustained fights',
        'Time item timings for major fights',
        'Abuse your gold advantage'
      ]
    },
    gameplan: {
      early: 'Farm efficiently with Acid Spray and Greevils Greed',
      mid: 'Get Radiance quickly, start participating in fights',
      late: 'Become an unstoppable farming and fighting machine'
    }
  },
  {
    heroId: 'alchemist',
    mood: 'experimental',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'arcane', name: 'Arcane Boots', cost: 1300, phase: 'Early', priority: 'Core', description: 'Mana for team' },
      { id: 'guardian', name: 'Guardian Greaves', cost: 5300, phase: 'Mid', priority: 'Core', description: 'Team utility' },
      { id: 'aghs-team', name: "Aghanim's Scepter (for ally)", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Gift to teammate' },
      { id: 'lotus', name: 'Lotus Orb', cost: 4000, phase: 'Late', priority: 'Core', description: 'Team utility' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction' }
    ],
    playstyle: {
      dos: [
        'Play support Alchemist',
        'Farm Aghs for teammates',
        'Focus on team utility',
        'Abuse your farming to help team'
      ],
      donts: [
        "Don't steal farm from carry",
        "Don't neglect your own items",
        "Don't forget to support in fights",
        "Don't abandon your team"
      ],
      tips: [
        'Farm Aghs for your mid/carry first',
        'Use your farm speed for team items',
        'Prioritize team over personal items',
        'Coordinate with team for Aghs timing'
      ]
    },
    gameplan: {
      early: 'Support your carry while farming efficiently',
      mid: 'Get utility items and Aghs for teammates',
      late: 'Become the ultimate team supporter'
    }
  },

  // Axe builds
  {
    heroId: 'axe',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'vanguard', name: 'Vanguard', cost: 2200, phase: 'Early', priority: 'Core', description: 'Early survivability' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Essential for initiation' },
      { id: 'blade-mail', name: 'Blade Mail', cost: 2100, phase: 'Mid', priority: 'Core', description: 'Reflect damage during call' },
      { id: 'crimson', name: 'Crimson Guard', cost: 3550, phase: 'Late', priority: 'Core', description: 'Team damage reduction' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Late', priority: 'Luxury', description: 'Maximum tankiness' }
    ],
    playstyle: {
      dos: [
        'Initiate fights with Blink Call',
        'Tank damage during Counter Helix',
        'Focus squishy supports first',
        'Force fights when you have advantage'
      ],
      donts: [
        "Don't blink without backup",
        "Don't call tanks unnecessarily",
        "Don't forget to activate Blade Mail",
        "Don't chase without vision"
      ],
      tips: [
        'Practice blink-call combos',
        'Use Berserkers Call to interrupt TPs',
        'Position to hit multiple enemies',
        'Coordinate with team before initiating'
      ]
    },
    gameplan: {
      early: 'Tank damage in lane, get early survivability items',
      mid: 'Get Blink Dagger, start initiating fights',
      late: 'Become the primary teamfight initiator'
    }
  },
  {
    heroId: 'axe',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Speed for chaos' },
      { id: 'shadow-blade', name: 'Shadow Blade', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Surprise initiations' },
      { id: 'blade-mail', name: 'Blade Mail', cost: 2100, phase: 'Mid', priority: 'Core', description: 'Chaos damage reflection' },
      { id: 'mjollnir', name: 'Mjollnir', cost: 5600, phase: 'Late', priority: 'Situational', description: 'Lightning chaos' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double chaos' }
    ],
    playstyle: {
      dos: [
        'Surprise enemies with Shadow Blade calls',
        'Fight early and often',
        'Create chaotic teamfight situations',
        'Build unexpected items'
      ],
      donts: [
        "Don't play it safe",
        "Don't worry about perfect positioning",
        "Don't avoid risky plays",
        "Don't stick to meta builds"
      ],
      tips: [
        'Use Shadow Blade for surprise factor',
        'Activate Blade Mail before calling',
        'Fight in creep waves for more spins',
        'Create maximum chaos in fights'
      ]
    },
    gameplan: {
      early: 'Play aggressively, fight early and often',
      mid: 'Get surprise items, continue chaotic fighting',
      late: 'Become the ultimate chaos creator in teamfights'
    }
  },

  // Beastmaster builds
  {
    heroId: 'beastmaster',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana for summons' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Initiation tool' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced ultimate' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Core', description: 'Double roar' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Luxury', description: 'Aura for summons' }
    ],
    playstyle: {
      dos: [
        'Use summons for vision and scouting',
        'Look for Roar opportunities',
        'Micro manage your units effectively',
        'Provide vision for your team'
      ],
      donts: [
        "Don't waste your ultimate",
        "Don't neglect summon micro",
        "Don't forget to use hawk for vision",
        "Don't initiate without backup"
      ],
      tips: [
        'Practice summon micro management',
        'Use hawk for high ground vision',
        'Time Roar to catch key targets',
        'Coordinate with team for lockdown'
      ]
    },
    gameplan: {
      early: 'Control lane with summons, provide vision',
      mid: 'Get Blink and Aghs, start initiating fights',
      late: 'Become primary initiator with double roar'
    }
  },

  // Brewmaster builds
  {
    heroId: 'brewmaster',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bottle', name: 'Bottle', cost: 675, phase: 'Early', priority: 'Core', description: 'Sustain for mid' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Initiation' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced spirits' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Core', description: 'Double ultimate chaos' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction' }
    ],
    playstyle: {
      dos: [
        'Master elemental spirit micro',
        'Use ultimate to escape impossible situations',
        'Create chaos in teamfights',
        'Focus on disrupting enemy team'
      ],
      donts: [
        "Don't panic during ultimate",
        "Don't waste your spirits",
        "Don't forget each spirit's abilities",
        "Don't neglect positioning"
      ],
      tips: [
        'Practice spirit micro extensively',
        'Use storm spirit for mobility',
        'Earth spirit for disable',
        'Fire spirit for damage'
      ]
    },
    gameplan: {
      early: 'Farm mid efficiently, practice spirit control',
      mid: 'Get core items, start teamfighting',
      late: 'Master the chaos of elemental combat'
    }
  },

  // Bristleback builds
  {
    heroId: 'bristleback',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'vanguard', name: 'Vanguard', cost: 2200, phase: 'Early', priority: 'Core', description: 'Early tankiness' },
      { id: 'hood', name: 'Hood of Defiance', cost: 1200, phase: 'Early', priority: 'Core', description: 'Magic resistance' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Mid', priority: 'Core', description: 'Cooldown reduction' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Late', priority: 'Core', description: 'Maximum HP' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Luxury', description: 'Attack speed and armor' }
    ],
    playstyle: {
      dos: [
        'Turn your back to enemies',
        'Spam Quill Spray constantly',
        'Tank damage to build Warpath stacks',
        'Chase enemies while they run'
      ],
      donts: [
        "Don't face enemies directly",
        "Don't waste mana early game",
        "Don't get kited by ranged heroes",
        "Don't abandon positioning"
      ],
      tips: [
        'Always keep your back to enemies',
        'Build Warpath stacks before fighting',
        'Use Nasal Goo to slow escaping enemies',
        'Tank damage to trigger Bristleback'
      ]
    },
    gameplan: {
      early: 'Build early survivability, farm safely',
      mid: 'Start teamfighting with core survivability items',
      late: 'Become an unkillable damage sponge'
    }
  },

  // Centaur Warrunner builds
  {
    heroId: 'centaur-warrunner',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bracer', name: 'Bracer', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Initiation tool' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Mid', priority: 'Core', description: 'Maximum strength' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Late', priority: 'Core', description: 'Enhanced abilities' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Luxury', description: 'Team aura' }
    ],
    playstyle: {
      dos: [
        'Initiate with Blink Stomp',
        'Use Return damage to trade',
        'Stampede to save team or engage',
        'Build maximum strength for damage'
      ],
      donts: [
        "Don't waste Stampede charges",
        "Don't forget Return is passive",
        "Don't neglect positioning",
        "Don't initiate without backup"
      ],
      tips: [
        'More strength = more damage',
        'Use Stampede to escape ganks',
        'Double Edge + Return combo',
        'Coordinate team for Stampede'
      ]
    },
    gameplan: {
      early: 'Farm strength items, use Return for trades',
      mid: 'Get Blink, start initiating fights',
      late: 'Become the primary initiator with global mobility'
    }
  },

  // Chaos Knight builds
  {
    heroId: 'chaos-knight',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bracer', name: 'Bracer', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats' },
      { id: 'echo', name: 'Echo Sabre', cost: 2650, phase: 'Mid', priority: 'Core', description: 'Double hit synergy' },
      { id: 'bkb', name: 'Black King Bar', cost: 4050, phase: 'Mid', priority: 'Core', description: 'Magic immunity' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Late', priority: 'Core', description: 'Illusion survivability' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Luxury', description: 'Attack speed for illusions' }
    ],
    playstyle: {
      dos: [
        'Embrace the RNG of Chaos Strike',
        'Use illusions to confuse enemies',
        'Chaos Bolt for reliable disable',
        'Fight with full illusion army'
      ],
      donts: [
        "Don't rely purely on RNG",
        "Don't waste ultimate charges",
        "Don't forget Reality Rift positioning",
        "Don't fight without illusions"
      ],
      tips: [
        'Phantasm before big fights',
        'Use Reality Rift to gap close',
        'Items that help illusions help you',
        'Chaos Strike scales with items'
      ]
    },
    gameplan: {
      early: 'Farm basic items, look for kill opportunities',
      mid: 'Get core items, fight with illusions',
      late: 'Overwhelm enemies with chaos army'
    }
  },

  // Clockwerk builds
  {
    heroId: 'clockwerk',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana for abilities' },
      { id: 'blade-mail', name: 'Blade Mail', cost: 2100, phase: 'Mid', priority: 'Core', description: 'Damage reflection in cogs' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced hookshot' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Late', priority: 'Core', description: 'Survivability' },
      { id: 'assault', name: 'Assault Cuirass', cost: 5125, phase: 'Late', priority: 'Luxury', description: 'Attack speed and armor' }
    ],
    playstyle: {
      dos: [
        'Look for Hookshot opportunities',
        'Isolate targets with Power Cogs',
        'Use Battery Assault on single targets',
        'Practice hook predictions'
      ],
      donts: [
        "Don't hook into multiple enemies",
        "Don't waste cogs without purpose",
        "Don't miss easy hookshots",
        "Don't forget flare for vision"
      ],
      tips: [
        'Practice hookshot angles and timings',
        'Use cogs to trap or push enemies',
        'Battery Assault interrupts channeling',
        'Rocket Flare for vision and farm'
      ]
    },
    gameplan: {
      early: 'Harass with rocket flare, get mana sustain',
      mid: 'Look for hookshot picks with Aghs',
      late: 'Become primary initiator and ganker'
    }
  },

  // Continue with all other Strength heroes...
  // [Adding builds for remaining strength heroes: Doom, Dragon Knight, Earth Spirit, Earthshaker, Elder Titan, Huskar, etc.]

  // AGILITY HEROES

  // Anti-Mage builds
  {
    heroId: 'anti-mage',
    mood: 'aggressive',
    items: [
      { id: 'quelling', name: 'Quelling Blade', cost: 130, phase: 'Early', priority: 'Core', description: 'Farming efficiency' },
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'power-treads', name: 'Power Treads', cost: 1400, phase: 'Early', priority: 'Core', description: 'Stats and attack speed' },
      { id: 'battlefury', name: 'Battle Fury', cost: 4100, phase: 'Early', priority: 'Core', description: 'Farming acceleration' },
      { id: 'manta', name: 'Manta Style', cost: 4600, phase: 'Mid', priority: 'Core', description: 'Illusions and dispel' },
      { id: 'basher', name: 'Skull Basher', cost: 2700, phase: 'Late', priority: 'Core', description: 'Lockdown for kills' }
    ],
    playstyle: {
      dos: [
        'Farm aggressively with Battle Fury',
        'Look for isolated targets to kill',
        'Use Mana Break to drain supports',
        'Split push when team fights'
      ],
      donts: [
        "Don't fight early without items",
        "Don't ignore farming opportunities",
        "Don't blink into grouped enemies",
        "Don't forget to carry TPs"
      ],
      tips: [
        'Practice last hitting efficiently',
        'Learn optimal farming routes',
        'Time your split push with team fights',
        'Focus on mana-dependent targets'
      ]
    },
    gameplan: {
      early: 'Focus on getting Battle Fury as quickly as possible',
      mid: 'Farm efficiently while looking for pickoff opportunities',
      late: 'Become the primary damage dealer and close out games'
    }
  },
  {
    heroId: 'anti-mage',
    mood: 'defensive',
    items: [
      { id: 'quelling', name: 'Quelling Blade', cost: 130, phase: 'Early', priority: 'Core', description: 'Farming efficiency' },
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'ring-of-health', name: 'Ring of Health', cost: 200, phase: 'Early', priority: 'Core', description: 'Lane sustain' },
      { id: 'battlefury', name: 'Battle Fury', cost: 4100, phase: 'Early', priority: 'Core', description: 'Safe farming tool' },
      { id: 'treads', name: 'Power Treads', cost: 1400, phase: 'Mid', priority: 'Core', description: 'Stats and survivability' },
      { id: 'linken', name: "Linken's Sphere", cost: 4600, phase: 'Mid', priority: 'Core', description: 'Spell block protection' }
    ],
    playstyle: {
      dos: [
        'Focus purely on safe farming',
        'Use Blink for escapes only',
        'Farm in safe areas',
        'Build defensive items first'
      ],
      donts: [
        "Don't take unnecessary fights",
        "Don't farm dangerous areas",
        "Don't chase kills",
        "Don't neglect map awareness"
      ],
      tips: [
        'Always carry TPs for escapes',
        'Use Blink defensively',
        'Farm jungle when lanes are dangerous',
        'Communicate with team about rotations'
      ]
    },
    gameplan: {
      early: 'Focus on safe farming and avoiding fights',
      mid: 'Continue safe farming with defensive items',
      late: 'Enter fights only when necessary and with advantage'
    }
  },

  // Continue with remaining Agility heroes...
  // [Adding builds for all remaining agility heroes]

  // INTELLIGENCE HEROES

  // Crystal Maiden builds
  {
    heroId: 'crystal-maiden',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Essential mobility' },
      { id: 'bracer', name: 'Bracer', cost: 505, phase: 'Early', priority: 'Core', description: 'Early survivability' },
      { id: 'glimmer', name: 'Glimmer Cape', cost: 1950, phase: 'Mid', priority: 'Core', description: 'Invisibility and magic resist' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Positioning for ultimate' },
      { id: 'bkb', name: 'Black King Bar', cost: 4050, phase: 'Late', priority: 'Core', description: 'Spell immunity for channeling' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Late', priority: 'Luxury', description: 'Enhanced ultimate' }
    ],
    playstyle: {
      dos: [
        'Stay behind your team always',
        'Use Frostbite on key targets',
        'Look for multi-hero ultimates',
        'Provide mana aura for team'
      ],
      donts: [
        "Don't show yourself early in fights",
        "Don't channel ultimate without protection",
        "Don't walk alone ever",
        "Don't forget to use your disables"
      ],
      tips: [
        'Practice blink-ultimate combinations',
        'Learn to cancel ultimate if necessary',
        'Position behind trees and cliffs',
        'Communicate with team before big ultimates'
      ]
    },
    gameplan: {
      early: 'Focus on harassing in lane, stack camps, stay safe and get basic items',
      mid: 'Get Blink Dagger, start looking for teamfight opportunities',
      late: 'Become the teamfight game-changer with massive ultimate damage'
    }
  },
  {
    heroId: 'crystal-maiden',
    mood: 'defensive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bracer', name: 'Bracer', cost: 505, phase: 'Early', priority: 'Core', description: 'Survivability' },
      { id: 'tranquil', name: 'Tranquil Boots', cost: 925, phase: 'Early', priority: 'Core', description: 'Health regen' },
      { id: 'force-staff', name: 'Force Staff', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Escape mechanism' },
      { id: 'ghost', name: 'Ghost Scepter', cost: 1500, phase: 'Mid', priority: 'Core', description: 'Physical immunity' },
      { id: 'glimmer', name: 'Glimmer Cape', cost: 1950, phase: 'Late', priority: 'Core', description: 'Team utility' }
    ],
    playstyle: {
      dos: [
        'Focus on staying alive above all',
        'Use spells from maximum range',
        'Ward defensively for team vision',
        'Provide aura support from safety'
      ],
      donts: [
        "Don't take unnecessary risks",
        "Don't position aggressively",
        "Don't use ultimate without escape plan",
        "Don't chase for kills"
      ],
      tips: [
        'Always carry multiple TPs',
        'Use trees for defensive positioning',
        'Keep Force Staff ready for escapes',
        'Communicate with team constantly'
      ]
    },
    gameplan: {
      early: 'Focus purely on survival and supporting team safely',
      mid: 'Build defensive items, provide utility from backlines',
      late: 'Maximize team aura benefit while staying alive'
    }
  },
  {
    heroId: 'crystal-maiden',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bottle', name: 'Bottle', cost: 675, phase: 'Early', priority: 'Core', description: 'Sustain for chaos' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Speed and damage' },
      { id: 'veil', name: 'Veil of Discord', cost: 1525, phase: 'Mid', priority: 'Core', description: 'Magic amplification' },
      { id: 'dagon', name: 'Dagon', cost: 2700, phase: 'Mid', priority: 'Situational', description: 'Burst damage chaos' },
      { id: 'ethereal', name: 'Ethereal Blade', cost: 4650, phase: 'Late', priority: 'Luxury', description: 'Maximum burst potential' }
    ],
    playstyle: {
      dos: [
        'Go for risky ultimate positions',
        'Build damage over survivability',
        'Look for surprise burst kills',
        'Create chaotic teamfight scenarios'
      ],
      donts: [
        "Don't play it safe",
        "Don't worry about dying for big plays",
        "Don't avoid dangerous positions",
        "Don't build defensive items"
      ],
      tips: [
        'Use Dagon for instant burst',
        'Combine Veil with ultimate',
        'Position aggressively for chaos',
        'Focus on maximum damage output'
      ]
    },
    gameplan: {
      early: 'Play aggressively, prioritize damage items over safety',
      mid: 'Look for high-risk high-reward ultimate positions',
      late: 'Become a glass cannon focused on maximum chaos'
    }
  },

  // Zeus builds
  {
    heroId: 'zeus',
    mood: 'aggressive',
    items: [
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats and damage' },
      { id: 'bottle', name: 'Bottle', cost: 675, phase: 'Early', priority: 'Core', description: 'Mana and health sustain' },
      { id: 'arcane', name: 'Arcane Boots', cost: 1300, phase: 'Early', priority: 'Core', description: 'Mana sustain' },
      { id: 'veil', name: 'Veil of Discord', cost: 1525, phase: 'Mid', priority: 'Core', description: 'Magic amplification' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced ultimate' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double ultimates' }
    ],
    playstyle: {
      dos: [
        'Harass constantly with Arc Lightning',
        'Use Lightning Bolt for vision',
        'Look for multi-hero ultimates',
        'Stay aggressive in lane'
      ],
      donts: [
        "Don't show yourself early in fights",
        "Don't waste mana on single targets",
        "Don't get caught without escape",
        "Don't forget to use Static Field"
      ],
      tips: [
        'Use Arc Lightning to last hit and harass',
        'Time your ultimate for maximum damage',
        'Position for multi-hero Static Field procs',
        'Use Lightning Bolt to scout and secure kills'
      ]
    },
    gameplan: {
      early: 'Dominate your lane with aggressive harass and efficient farming',
      mid: 'Join fights early with your nuking power and ultimate',
      late: 'Be the primary magical damage dealer in teamfights'
    }
  },

  // Lion builds
  {
    heroId: 'lion',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'tranquil', name: 'Tranquil Boots', cost: 925, phase: 'Early', priority: 'Core', description: 'Health regen' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Initiation tool' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced ultimate' },
      { id: 'glimmer', name: 'Glimmer Cape', cost: 1950, phase: 'Mid', priority: 'Situational', description: 'Survival tool' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double combo' }
    ],
    playstyle: {
      dos: [
        'Look for blink-hex-ultimate combos',
        'Use Mana Drain on key targets',
        'Position aggressively for initiations',
        'Focus squishy targets first'
      ],
      donts: [
        "Don't waste your disable combos",
        "Don't show yourself before initiating",
        "Don't forget to drain mana",
        "Don't hesitate to use ultimate on supports"
      ],
      tips: [
        'Practice blink-hex timing',
        'Learn to queue spells properly',
        'Use terrain for positioning',
        'Communicate with team before initiating'
      ]
    },
    gameplan: {
      early: 'Harass in lane, set up kills with your disables',
      mid: 'Get Blink Dagger, start looking for pickoffs',
      late: 'Be the primary initiator with your disable combo'
    }
  },

  // Continue adding builds for ALL remaining heroes...
  // This would include every single Dota 2 hero with appropriate builds for each mood

  // [Continue with remaining intelligence heroes: Invoker, Ancient Apparition, Bane, Batrider, etc.]
  // [Add builds for remaining agility heroes: Arc Warden, Bloodseeker, Bounty Hunter, etc.]
  // [Add builds for remaining strength heroes: Doom, Dragon Knight, Earth Spirit, etc.]

  // Invoker builds
  {
    heroId: 'invoker',
    mood: 'creative',
    items: [
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Stats and damage for laning' },
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'midas', name: 'Hand of Midas', cost: 2050, phase: 'Early', priority: 'Core', description: 'Accelerated experience and gold' },
      { id: 'travels', name: 'Boots of Travel', cost: 2500, phase: 'Mid', priority: 'Core', description: 'Global mobility' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Reduces cooldowns and improves spells' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction and spell lifesteal' }
    ],
    playstyle: {
      dos: [
        'Master at least 6-8 spell combinations',
        'Practice quick spell switching',
        'Use Forge Spirits for last hitting',
        'Experiment with different Quas/Wex/Exort builds'
      ],
      donts: [
        "Don't panic and forget your combos",
        "Don't neglect your orb walking",
        "Don't always go for the flashy plays",
        "Don't ignore your positioning"
      ],
      tips: [
        'Bind orbs to easily accessible keys',
        'Practice common combos until muscle memory',
        'Learn to adapt your build to game situation',
        'Study pro Invoker players on YouTube'
      ]
    },
    gameplan: {
      early: 'Farm efficiently with Forge Spirits, get Midas quickly, focus on getting levels',
      mid: 'Start participating in fights with your core combos, get Aghs to reduce cooldowns',
      late: 'Become the primary damage dealer, use advanced combos to control teamfights'
    }
  },
  {
    heroId: 'invoker',
    mood: 'experimental',
    items: [
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Stats for experimental builds' },
      { id: 'bottle', name: 'Bottle', cost: 675, phase: 'Early', priority: 'Core', description: 'Sustain for different builds' },
      { id: 'urn', name: 'Urn of Shadows', cost: 875, phase: 'Early', priority: 'Situational', description: 'Experimental roaming item' },
      { id: 'euls', name: "Eul's Scepter", cost: 2725, phase: 'Mid', priority: 'Core', description: 'Setup for experimental combos' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Core', description: 'Double spell combos' },
      { id: 'hex', name: 'Scythe of Vyse', cost: 5675, phase: 'Late', priority: 'Luxury', description: 'Disable for complex plays' }
    ],
    playstyle: {
      dos: [
        'Try unconventional orb combinations',
        'Experiment with different item builds',
        'Practice complex spell rotations',
        'Roam early for kills'
      ],
      donts: [
        "Don't stick to standard builds",
        "Don't ignore team composition needs",
        "Don't over-experiment in ranked",
        "Don't forget core mechanics"
      ],
      tips: [
        'Try max Wex builds for mobility',
        'Experiment with Quas tank builds',
  // Phantom Assassin builds
  {
    heroId: 'phantom-assassin',
    mood: 'aggressive',
    items: [
      { id: 'quelling', name: 'Quelling Blade', cost: 130, phase: 'Early', priority: 'Core', description: 'Farming efficiency' },
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'desolator', name: 'Desolator', cost: 3500, phase: 'Mid', priority: 'Core', description: 'Early damage and armor reduction' },
      { id: 'bkb', name: 'Black King Bar', cost: 4050, phase: 'Mid', priority: 'Core', description: 'Magic immunity for aggressive plays' },
      { id: 'basher', name: 'Skull Basher', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Lockdown for kills' },
      { id: 'abyssal', name: 'Abyssal Blade', cost: 6250, phase: 'Late', priority: 'Core', description: 'Reliable disable for pickoffs' }
    ],
    playstyle: {
      dos: [
        'Look for early kills with Phantom Strike',
        'Use Blur to trade efficiently',
        'Focus squishy supports first',
        'Play aggressively in lane'
      ],
      donts: [
        "Don't be afraid to dive towers",
        "Don't skip early fights",
        "Don't play too defensively",
        "Don't ignore kill opportunities"
      ],
      tips: [
        'Use Phantom Strike to gap close',
        'Learn to animation cancel with dagger',
        'Time BKB for key moments',
        'Focus on critical strike timing'
      ]
    },
    gameplan: {
      early: 'Farm aggressively while looking for kills, use Phantom Strike for trades',
      mid: 'Join fights with Desolator, focus on getting kills over farm',
      late: 'Be the primary damage dealer, hunt down key targets'
    }
  },
  {
    heroId: 'phantom-assassin',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Damage and mobility' },
      { id: 'vanguard', name: 'Vanguard', cost: 2200, phase: 'Early', priority: 'Core', description: 'Early survivability for chaos' },
      { id: 'radiance', name: 'Radiance', cost: 5150, phase: 'Mid', priority: 'Situational', description: 'Chaos damage aura' },
      { id: 'butterfly', name: 'Butterfly', cost: 5525, phase: 'Late', priority: 'Core', description: 'Maximum evasion chaos' },
      { id: 'divine', name: 'Divine Rapier', cost: 5750, phase: 'Late', priority: 'Luxury', description: 'Ultimate chaos risk/reward' }
    ],
    playstyle: {
      dos: [
        'Take maximum risks for kills',
        'Build unconventional items',
        'Fight constantly regardless of odds',
        'Go for high-risk plays'
      ],
      donts: [
        "Don't play it safe",
        "Don't worry about farming patterns",
        "Don't avoid fights",
        "Don't build standard items"
      ],
      tips: [
        'Use Phantom Strike aggressively',
        'Rely on RNG for big plays',
        'Take fights you probably shouldnt',
        'Focus on creating chaos in fights'
      ]
    },
    gameplan: {
      early: 'Fight early and often, take maximum risks',
      mid: 'Continue aggressive fighting with unconventional items',
      late: 'Go for game-ending risky plays with high-damage items'
    }
  },
        'Test different spell priorities',
  // Sniper builds
  {
    heroId: 'sniper',
    mood: 'defensive',
    items: [
      { id: 'wraith', name: 'Wraith Band', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats and damage' },
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'maelstrom', name: 'Maelstrom', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Farming and wave clear' },
      { id: 'dragon-lance', name: 'Dragon Lance', cost: 1900, phase: 'Mid', priority: 'Core', description: 'Range and stats' },
      { id: 'bkb', name: 'Black King Bar', cost: 4050, phase: 'Mid', priority: 'Core', description: 'Survival in fights' },
      { id: 'mjollnir', name: 'Mjollnir', cost: 5600, phase: 'Late', priority: 'Core', description: 'High DPS and protection' }
    ],
    playstyle: {
      dos: [
        'Stay at maximum range always',
        'Use Shrapnel for vision and control',
        'Focus on farming safely',
        'Position behind your team'
      ],
      donts: [
        "Don't overextend for kills",
        "Don't fight without vision",
        "Don't get caught alone",
        "Don't chase into fog"
      ],
      tips: [
        'Use Shrapnel to clear waves safely',
        'Always carry TPs for escapes',
        'Learn to kite with range advantage',
        'Use high ground for positioning'
      ]
    },
    gameplan: {
      early: 'Focus on safe farming, use range to harass enemies',
      mid: 'Farm efficiently with Maelstrom, join fights from safe distance',
      late: 'Become the primary damage source while staying safe'
    }
  },
  {
    heroId: 'sniper',
    mood: 'aggressive',
    items: [
      { id: 'wraith', name: 'Wraith Band', cost: 505, phase: 'Early', priority: 'Core', description: 'Early damage' },
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Damage and mobility' },
      { id: 'shadow-blade', name: 'Shadow Blade', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Aggressive positioning' },
      { id: 'desolator', name: 'Desolator', cost: 3500, phase: 'Mid', priority: 'Core', description: 'Armor reduction and damage' },
      { id: 'mkb', name: 'Monkey King Bar', cost: 5400, phase: 'Late', priority: 'Core', description: 'True strike and damage' }
    ],
    playstyle: {
      dos: [
        'Use Shadow Blade for aggressive positioning',
        'Look for isolated targets to burst',
        'Use Assassinate to secure kills',
        'Position aggressively for maximum damage'
      ],
      donts: [
        "Don't be too passive",
        "Don't ignore kill opportunities",
        "Don't always play from maximum range",
        "Don't forget to use Shadow Blade offensively"
      ],
      tips: [
        'Use Shadow Blade to get better positions',
        'Time Assassinate for kill securing',
        'Use Shrapnel aggressively for vision',
        'Focus on burst damage combos'
      ]
    },
    gameplan: {
      early: 'Play aggressively in lane, look for kill opportunities',
      mid: 'Use Shadow Blade for aggressive plays and pickoffs',
      late: 'Become a high-damage assassin with burst potential'
    }
  },
  {
    heroId: 'juggernaut',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Damage and mobility' },
      { id: 'mask-of-madness', name: 'Mask of Madness', cost: 1800, phase: 'Mid', priority: 'Core', description: 'Attack speed and lifesteal' },
      { id: 'desolator', name: 'Desolator', cost: 3500, phase: 'Mid', priority: 'Core', description: 'Armor reduction and damage' },
      { id: 'bkb', name: 'Black King Bar', cost: 4050, phase: 'Mid', priority: 'Core', description: 'Magic immunity' },
      { id: 'butterfly', name: 'Butterfly', cost: 5525, phase: 'Late', priority: 'Luxury', description: 'Evasion and attack speed' }
    ],
    playstyle: {
      dos: [
        'Use Blade Fury to escape and farm',
        'Look for early kill opportunities',
        'Use Omnislash on isolated targets',
        'Play aggressively in lane'
      ],
      donts: [
        "Don't waste Omnislash on creeps",
        "Don't be afraid to dive with Blade Fury",
        "Don't neglect your healing ward",
        "Don't fight without mana for spells"
      ],
      tips: [
        'Use Blade Fury to dodge spells',
        'Time Omnislash when enemies are alone',
        'Use Healing Ward to sustain aggression',
        'Learn to animation cancel with Phase'
      ]
    },
    gameplan: {
      early: 'Farm efficiently while looking for kills with Blade Fury',
      mid: 'Join fights with Omnislash, focus on getting kills',
      late: 'Become the primary damage dealer in teamfights'
    }
  },
  // Techies builds
  {
    heroId: 'techies',
    mood: 'chaos',
    items: [
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana sustain for mining' },
      { id: 'tranquil', name: 'Tranquil Boots', cost: 925, phase: 'Early', priority: 'Core', description: 'Movement and health regen' },
      { id: 'force-staff', name: 'Force Staff', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Escape and positioning' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Improves all abilities' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction for more mines' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double ultimates for chaos' }
    ],
    playstyle: {
      dos: [
        'Mine key chokepoints and rune spots',
        'Stack mines in unexpected locations',
        'Use Blast Off for aggressive plays',
        'Constantly pressure enemy movements'
      ],
      donts: [
        "Don't mine the same spots repeatedly",
        "Don't neglect your team in fights",
        "Don't forget to upgrade your mines",
        "Don't waste mana on obvious mine spots"
      ],
      tips: [
        'Learn common ward spots to counter-mine',
        'Coordinate with team for mine setups',
        'Use trees and cliffs for sneaky mines',
        'Practice mine stacking techniques'
      ]
    },
    gameplan: {
      early: 'Focus on mining key areas, get Soul Ring for mana sustain, stay hidden',
      mid: 'Start participating in fights with Blast Off, continue map control with mines',
      late: 'Become the ultimate area denial hero, control teamfight locations with mine fields'
    }
  },
  // Slark builds
  {
    heroId: 'slark',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'poor-mans', name: "Poor Man's Shield", cost: 500, phase: 'Early', priority: 'Core', description: 'Early survivability' },
      { id: 'shadow-blade', name: 'Shadow Blade', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Initiation and escape' },
      { id: 'echo', name: 'Echo Sabre', cost: 2650, phase: 'Mid', priority: 'Core', description: 'Double hit and stats' },
      { id: 'silver-edge', name: 'Silver Edge', cost: 5450, phase: 'Late', priority: 'Core', description: 'Enhanced invisibility' },
      { id: 'skadi', name: 'Eye of Skadi', cost: 5675, phase: 'Late', priority: 'Luxury', description: 'Stats and slow' }
    ],
    playstyle: {
      dos: [
        'Look for solo pickoffs constantly',
        'Use Dark Pact to dispel debuffs',
        'Steal stats from enemy heroes',
        'Play aggressively with escapes'
      ],
      donts: [
        "Don't fight when detected",
        "Don't ignore essence shift stacks",
        "Don't overextend without ultimate",
        "Don't fight in groups early"
      ],
      tips: [
        'Use Shadow Blade for initiation',
        'Time Dark Pact before incoming stuns',
        'Focus on getting solo kills',
        'Use ultimate for repositioning'
      ]
    },
    gameplan: {
      early: 'Farm efficiently while looking for kill opportunities',
      mid: 'Start hunting for solo pickoffs with Shadow Blade',
      late: 'Become an unstoppable killing machine with stat steal'
    }
  },
  // Earthshaker builds
  {
    heroId: 'earthshaker',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana sustain' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Essential for initiation' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced ultimate' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double ultimate chaos' },
      { id: 'octarine', name: 'Octarine Core', cost: 5275, phase: 'Late', priority: 'Luxury', description: 'Cooldown reduction' }
    ],
    playstyle: {
      dos: [
        'Look for multi-hero Echo Slams',
        'Use Fissure to block paths',
        'Chain your spells for maximum chaos',
        'Create chaotic teamfight situations'
      ],
      donts: [
        "Don't waste Echo Slam on single targets",
        "Don't forget to block with Fissure",
        "Don't hesitate to go for big plays",
        "Don't save your ultimate too long"
      ],
      tips: [
        'Practice blink-echo timing',
        'Use creeps for bigger Echo Slams',
        'Learn Fissure blocking spots',
        'Coordinate with team for maximum chaos'
      ]
    },
    gameplan: {
      early: 'Focus on getting Blink Dagger, provide setup for team',
      mid: 'Start initiating fights with massive Echo Slams',
      late: 'Be the primary teamfight initiator, create maximum chaos'
    }
  },
  // Windranger builds
  {
    heroId: 'windranger',
    mood: 'creative',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'null-talisman', name: 'Null Talisman', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats' },
      { id: 'maelstrom', name: 'Maelstrom', cost: 2700, phase: 'Mid', priority: 'Core', description: 'Farming and DPS' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Positioning' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Mid', priority: 'Core', description: 'Enhanced ultimate' },
      { id: 'mjollnir', name: 'Mjollnir', cost: 5600, phase: 'Late', priority: 'Core', description: 'High DPS' }
    ],
    playstyle: {
      dos: [
        'Practice Shackleshot angles',
        'Use Windrun for positioning',
        'Focus fire key targets',
        'Experiment with different builds'
      ],
      donts: [
        "Don't waste Shackleshot on singles",
        "Don't forget to use Powershot for vision",
        "Don't neglect your positioning",
        "Don't focus fire tanks unnecessarily"
      ],
      tips: [
        'Learn common Shackle spots',
        'Use Powershot to scout and farm',
        'Time Focus Fire with BKB/Windrun',
        'Practice creative Shackle angles'
      ]
    },
    gameplan: {
      early: 'Farm efficiently while practicing skill shots',
      mid: 'Look for creative Shackle setups and pickoffs',
      late: 'Become a high DPS core with creative positioning'
    }
  },
  // Omniknight builds
  {
    heroId: 'omniknight',
    mood: 'defensive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'soul-ring', name: 'Soul Ring', cost: 770, phase: 'Early', priority: 'Core', description: 'Mana sustain' },
      { id: 'mek', name: 'Mekansm', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Team healing' },
      { id: 'guardian', name: 'Guardian Greaves', cost: 5300, phase: 'Mid', priority: 'Core', description: 'Enhanced team support' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Late', priority: 'Core', description: 'Improved abilities' },
      { id: 'refresher', name: 'Refresher Orb', cost: 5000, phase: 'Late', priority: 'Luxury', description: 'Double ultimates' }
    ],
    playstyle: {
      dos: [
        'Stay close to your carry',
        'Use Purification proactively',
        'Time your ultimate perfectly',
        'Build aura items for team'
      ],
      donts: [
        "Don't waste your ultimate too early",
        "Don't leave your team alone",
        "Don't forget to dispel debuffs",
        "Don't neglect your positioning"
      ],
      tips: [
        'Learn to predict incoming damage',
        'Practice quick dispel reactions',
        'Coordinate with team for engagements',
        'Use Degen Aura to kite enemies'
      ]
    },
    gameplan: {
      early: 'Focus on keeping your carry alive and farming safely',
      mid: 'Start grouping with team, provide healing and protection',
      late: 'Become the ultimate team protector with Guardian Angel'
    }
  },
  // Pudge builds (keeping existing ones and adding more)
  {
    heroId: 'pudge',
    mood: 'aggressive',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility for positioning' },
      { id: 'bracer', name: 'Bracer', cost: 505, phase: 'Early', priority: 'Core', description: 'Early tankiness and stats' },
      { id: 'tranquil', name: 'Tranquil Boots', cost: 925, phase: 'Early', priority: 'Core', description: 'Health regen and movement speed' },
      { id: 'blink', name: 'Blink Dagger', cost: 2250, phase: 'Mid', priority: 'Core', description: 'Essential for initiation' },
      { id: 'force-staff', name: 'Force Staff', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Mobility and positioning' },
      { id: 'aghs', name: "Aghanim's Scepter", cost: 4200, phase: 'Late', priority: 'Luxury', description: 'Enhances abilities significantly' }
    ],
    playstyle: {
      dos: [
        'Look for hook opportunities constantly',
        'Position behind trees and fog',
        'Use Rot to secure kills and farm',
        'Ward enemy jungle for hook setups'
      ],
      donts: [
        "Don't waste hooks on creeps",
        "Don't overextend without backup",
        "Don't forget to stack neutral camps",
        "Don't hook enemies toward your team when low"
      ],
      tips: [
        'Practice hook predictions in demo mode',
        'Learn common hook spots on each map',
        'Communicate with team before big hooks',
        'Use Dismember to cancel enemy TPs'
      ]
    },
    gameplan: {
      early: 'Focus on laning phase, stack camps, and look for hook opportunities on enemy supports',
      mid: 'Get Blink Dagger ASAP, start roaming and setting up kills for your team',
      late: 'Become the primary initiator, focus on hooking key enemy heroes in teamfights'
    }
  },
  {
    heroId: 'pudge',
    mood: 'chaos',
    items: [
      { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
      { id: 'bottle', name: 'Bottle', cost: 675, phase: 'Early', priority: 'Core', description: 'Sustain for roaming chaos' },
      { id: 'phase', name: 'Phase Boots', cost: 1500, phase: 'Early', priority: 'Core', description: 'Speed for chaos hooks' },
      { id: 'aether', name: 'Aether Lens', cost: 2300, phase: 'Mid', priority: 'Core', description: 'Longer hook range for chaos' },
      { id: 'blade-mail', name: 'Blade Mail', cost: 2100, phase: 'Mid', priority: 'Core', description: 'Reflect damage while in the thick of it' },
      { id: 'heart', name: 'Heart of Tarrasque', cost: 5200, phase: 'Late', priority: 'Luxury', description: 'Maximum chaos survivability' }
    ],
    playstyle: {
      dos: [
        'Hook anyone and everyone',
        'Roam constantly and unpredictably',
        'Fight early and often',
        'Prioritize fun over efficiency'
      ],
      donts: [
        "Don't worry about perfect positioning",
        "Don't overthink hook timing",
        "Don't avoid risky plays",
        "Don't play it safe"
      ],
      tips: [
        'Blind hook through trees for surprise',
        'Use Rot + Blade Mail combo in fights',
        'Hook enemies into your team randomly',
        'Focus on creating chaotic situations'
      ]
    },
    gameplan: {
      early: 'Roam the map looking for any hook opportunity, create chaos in all lanes',
      mid: 'Continue aggressive roaming, start fights whenever possible',
      late: 'Be the chaos factor in teamfights, hook anyone you can reach'
    }
  }
]}}]