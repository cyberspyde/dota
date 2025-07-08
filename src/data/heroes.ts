import { Hero } from '../types';

export const heroes: Hero[] = [
  // Strength Heroes
  {
    id: 'abaddon',
    name: 'Abaddon',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['defensive', 'experimental'],
    description: 'A versatile support with powerful healing and damage absorption',
    strengths: ['Strong heals', 'Damage absorption', 'Good survivability'],
    weaknesses: ['Limited offensive power', 'Mana dependent', 'Predictable']
  },
  {
    id: 'alchemist',
    name: 'Alchemist',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A versatile hero who can farm extremely fast or support teammates',
    strengths: ['Fast farming', 'Versatile roles', 'Strong teamfight'],
    weaknesses: ['Vulnerable early', 'Complex itemization', 'Positioning crucial']
  },
  {
    id: 'axe',
    name: 'Axe',
    role: 'Initiator',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'defensive'],
    description: 'A tanky initiator who forces fights and soaks damage',
    strengths: ['Strong initiation', 'Tanky', 'Forces fights'],
    weaknesses: ['Lacks disable', 'Kitable', 'Falls off late']
  },
  {
    id: 'beastmaster',
    name: 'Beastmaster',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A versatile initiator with summons and powerful disable',
    strengths: ['Strong disable', 'Good vision', 'Versatile'],
    weaknesses: ['Micro management', 'Mana issues', 'Needs levels']
  },
  {
    id: 'brewmaster',
    name: 'Brewmaster',
    role: 'Initiator',
    difficulty: 'Hard',
    moods: ['chaos', 'experimental', 'aggressive'],
    description: 'A complex initiator who splits into elemental spirits',
    strengths: ['Game-changing ultimate', 'Strong teamfight', 'Multiple abilities'],
    weaknesses: ['Very complex', 'Long cooldowns', 'Micro intensive']
  },
  {
    id: 'bristleback',
    name: 'Bristleback',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A tanky carry who gets stronger as he takes damage',
    strengths: ['Very tanky', 'Scales with damage taken', 'Good sustain'],
    weaknesses: ['Kitable', 'Predictable', 'Vulnerable to silence']
  },
  {
    id: 'centaur-warrunner',
    name: 'Centaur Warrunner',
    role: 'Initiator',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'chaos'],
    description: 'A tanky initiator with global mobility ultimate',
    strengths: ['Very tanky', 'Strong initiation', 'Global mobility'],
    weaknesses: ['Mana issues', 'Limited farming', 'Predictable']
  },
  {
    id: 'chaos-knight',
    name: 'Chaos Knight',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['chaos', 'aggressive', 'experimental'],
    description: 'An illusion-based carry with high damage potential',
    strengths: ['High damage', 'Strong illusions', 'Good disable'],
    weaknesses: ['RNG dependent', 'Mana issues', 'Slow farmer']
  },
  {
    id: 'clockwerk',
    name: 'Clockwerk',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A precise initiator with long-range hook and isolation',
    strengths: ['Long range initiation', 'Good isolation', 'Strong harass'],
    weaknesses: ['Skill shot dependent', 'Mana issues', 'Falls off late']
  },
  {
    id: 'doom',
    name: 'Doom',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A versatile carry who can silence and disable enemies completely',
    strengths: ['Powerful disable', 'Versatile builds', 'Strong scaling'],
    weaknesses: ['Slow early game', 'Vulnerable to kiting', 'Long cooldowns']
  },
  {
    id: 'dragon-knight',
    name: 'Dragon Knight',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['defensive', 'aggressive', 'experimental'],
    description: 'A tanky carry who transforms into a dragon',
    strengths: ['Very tanky', 'Good sustain', 'Versatile'],
    weaknesses: ['Low damage early', 'Mana issues', 'Predictable']
  },
  {
    id: 'earth-spirit',
    name: 'Earth Spirit',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A highly mobile support with stone-based abilities',
    strengths: ['High mobility', 'Versatile spells', 'Strong teamfight'],
    weaknesses: ['Very complex', 'Stone management', 'High skill floor']
  },
  {
    id: 'earthshaker',
    name: 'Earthshaker',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'defensive'],
    description: 'A powerful initiator with game-changing AOE abilities',
    strengths: ['Amazing teamfight', 'Multiple disables', 'High burst'],
    weaknesses: ['Mana dependent', 'Needs positioning', 'Long cooldowns']
  },
  {
    id: 'elder-titan',
    name: 'Elder Titan',
    role: 'Initiator',
    difficulty: 'Hard',
    moods: ['experimental', 'aggressive', 'creative'],
    description: 'A strategic initiator with armor reduction and long-range abilities',
    strengths: ['Armor reduction', 'Long range', 'Strong teamfight'],
    weaknesses: ['Complex mechanics', 'Vulnerable', 'Skill shot dependent']
  },
  {
    id: 'huskar',
    name: 'Huskar',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A berserker carry who gets stronger at low health',
    strengths: ['Stronger when low', 'Magic resistance', 'High DPS'],
    weaknesses: ['Vulnerable to pure damage', 'Predictable', 'All-in playstyle']
  },
  {
    id: 'io',
    name: 'Io',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'defensive'],
    description: 'A unique support who tethers to allies and provides global mobility',
    strengths: ['Global mobility', 'Strong support', 'Unique mechanics'],
    weaknesses: ['Very complex', 'Team dependent', 'Fragile']
  },
  {
    id: 'kunkka',
    name: 'Kunkka',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A versatile hero with powerful AOE abilities and cleave',
    strengths: ['High AOE damage', 'Good disable', 'Versatile'],
    weaknesses: ['Skill shot dependent', 'Mana issues', 'Positioning crucial']
  },
  {
    id: 'legion-commander',
    name: 'Legion Commander',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A tanky initiator who grows stronger with each duel won',
    strengths: ['Grows stronger', 'Strong initiation', 'Magic immunity'],
    weaknesses: ['Duel can backfire', 'Needs good timing', 'Vulnerable to kiting']
  },
  {
    id: 'lifestealer',
    name: 'Lifestealer',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A tanky carry with built-in magic immunity and lifesteal',
    strengths: ['Magic immunity', 'Built-in lifesteal', 'Good survivability'],
    weaknesses: ['Kitable', 'Needs attack speed', 'Vulnerable to pure damage']
  },
  {
    id: 'lycan',
    name: 'Lycan',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A werewolf carry with powerful summons and transformation',
    strengths: ['Strong push', 'High mobility', 'Good jungling'],
    weaknesses: ['Vulnerable to AOE', 'Micro management', 'Predictable']
  },
  {
    id: 'magnus',
    name: 'Magnus',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A powerful initiator with reverse polarity and positioning spells',
    strengths: ['Game-changing ultimate', 'Good positioning', 'Strong teamfight'],
    weaknesses: ['Skill shot dependent', 'Mana issues', 'Positioning crucial']
  },
  {
    id: 'mars',
    name: 'Mars',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'A warrior initiator with arena-controlling abilities',
    strengths: ['Strong teamfight', 'Good disable', 'Arena control'],
    weaknesses: ['Mana issues', 'Positioning crucial', 'Long cooldowns']
  },
  {
    id: 'night-stalker',
    name: 'Night Stalker',
    role: 'Initiator',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A hunter who becomes much stronger during nighttime',
    strengths: ['Strong at night', 'Good mobility', 'Vision control'],
    weaknesses: ['Weak during day', 'Time dependent', 'Falls off late']
  },
  {
    id: 'omniknight',
    name: 'Omniknight',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['defensive', 'experimental', 'creative'],
    description: 'A defensive support who excels at protecting allies',
    strengths: ['Strong heals', 'Magic immunity', 'Team protection'],
    weaknesses: ['Low damage', 'Mana issues', 'Passive playstyle']
  },
  {
    id: 'primal-beast',
    name: 'Primal Beast',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A massive beast with powerful charge and AOE abilities',
    strengths: ['Strong initiation', 'High damage', 'Good survivability'],
    weaknesses: ['Vulnerable to kiting', 'Mana issues', 'Predictable']
  },
  {
    id: 'pudge',
    name: 'Pudge',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A tanky support who excels at picking off enemies with hooks',
    strengths: ['High HP', 'Game-changing hooks', 'Strong initiation'],
    weaknesses: ['Low mobility', 'Mana dependent', 'Skill shot reliant']
  },
  {
    id: 'sand-king',
    name: 'Sand King',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A mobile initiator with powerful AOE ultimate',
    strengths: ['High mobility', 'Strong AOE', 'Good disable'],
    weaknesses: ['Channeling ultimate', 'Mana dependent', 'Positioning crucial']
  },
  {
    id: 'slardar',
    name: 'Slardar',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A tanky carry with armor reduction and bash',
    strengths: ['Armor reduction', 'Good disable', 'Tanky'],
    weaknesses: ['Needs items', 'Kitable', 'Predictable']
  },
  {
    id: 'spirit-breaker',
    name: 'Spirit Breaker',
    role: 'Initiator',
    difficulty: 'Easy',
    moods: ['chaos', 'aggressive', 'experimental'],
    description: 'A global ganker who charges across the map',
    strengths: ['Global presence', 'Strong disable', 'Good mobility'],
    weaknesses: ['Predictable', 'Vulnerable during charge', 'Falls off late']
  },
  {
    id: 'sven',
    name: 'Sven',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A powerful carry with cleave and massive damage steroid',
    strengths: ['High damage', 'AOE cleave', 'Good disable'],
    weaknesses: ['Kitable', 'Mana issues', 'Needs items']
  },
  {
    id: 'tidehunter',
    name: 'Tidehunter',
    role: 'Initiator',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'chaos'],
    description: 'A tanky initiator with game-changing ravage ultimate',
    strengths: ['Game-changing ultimate', 'Very tanky', 'Strong teamfight'],
    weaknesses: ['Long ultimate cooldown', 'Mana issues', 'Predictable']
  },
  {
    id: 'timbersaw',
    name: 'Timbersaw',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A mobile nuker who cuts through trees and enemies',
    strengths: ['High mobility', 'Pure damage', 'Good against strength'],
    weaknesses: ['Tree dependent', 'Mana hungry', 'Vulnerable to silence']
  },
  {
    id: 'tiny',
    name: 'Tiny',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A growing carry who becomes a massive damage dealer',
    strengths: ['High damage late', 'Good disable', 'Versatile'],
    weaknesses: ['Slow early', 'Attack speed issues', 'Mana problems']
  },
  {
    id: 'treant-protector',
    name: 'Treant Protector',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['defensive', 'experimental', 'creative'],
    description: 'A supportive tree who provides vision and healing',
    strengths: ['Global heal', 'Good vision', 'Strong disable'],
    weaknesses: ['Very slow', 'Mana issues', 'Low damage']
  },
  {
    id: 'tusk',
    name: 'Tusk',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'creative', 'chaos'],
    description: 'A mobile initiator with snowball and positioning abilities',
    strengths: ['High mobility', 'Good disable', 'Creative plays'],
    weaknesses: ['Falls off late', 'Mana issues', 'Positioning crucial']
  },
  {
    id: 'underlord',
    name: 'Underlord',
    role: 'Initiator',
    difficulty: 'Easy',
    moods: ['defensive', 'experimental', 'aggressive'],
    description: 'A tanky initiator with global teleportation',
    strengths: ['Very tanky', 'Global mobility', 'Good teamfight'],
    weaknesses: ['Low mobility', 'Mana issues', 'Predictable']
  },
  {
    id: 'undying',
    name: 'Undying',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'defensive'],
    description: 'A tanky support who grows stronger by stealing strength',
    strengths: ['Steals strength', 'Very tanky', 'Strong early game'],
    weaknesses: ['Falls off late', 'Mana issues', 'Predictable']
  },
  {
    id: 'wraith-king',
    name: 'Wraith King',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A simple carry with reincarnation and lifesteal aura',
    strengths: ['Reincarnation', 'Simple mechanics', 'Good aura'],
    weaknesses: ['Very predictable', 'Mana issues', 'Kitable']
  },

  // Agility Heroes
  {
    id: 'anti-mage',
    name: 'Anti-Mage',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'A mobile carry that excels at farming and split pushing',
    strengths: ['High mobility', 'Fast farming', 'Strong late game'],
    weaknesses: ['Weak early game', 'Needs items', 'Vulnerable to ganks']
  },
  {
    id: 'arc-warden',
    name: 'Arc Warden',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'chaos'],
    description: 'A unique carry who can create a perfect duplicate of himself',
    strengths: ['Double everything', 'High farming', 'Unique mechanics'],
    weaknesses: ['Very complex', 'Micro intensive', 'Needs high skill']
  },
  {
    id: 'bloodseeker',
    name: 'Bloodseeker',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A fast-paced carry that hunts wounded enemies across the map',
    strengths: ['High speed', 'Global presence', 'Strong against low HP'],
    weaknesses: ['Vulnerable to TP', 'Kitable', 'Weak to disables']
  },
  {
    id: 'bounty-hunter',
    name: 'Bounty Hunter',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'An invisible support who tracks enemies and provides gold',
    strengths: ['Invisibility', 'Gold generation', 'Good harassment'],
    weaknesses: ['Fragile', 'Vulnerable to detection', 'Falls off late']
  },
  {
    id: 'broodmother',
    name: 'Broodmother',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['experimental', 'chaos', 'aggressive'],
    description: 'A spider carry who dominates lanes with webs and spiderlings',
    strengths: ['Web mobility', 'Spiderling army', 'Lane domination'],
    weaknesses: ['AOE vulnerable', 'Micro intensive', 'Predictable']
  },
  {
    id: 'clinkz',
    name: 'Clinkz',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'An invisible archer with high damage and mobility',
    strengths: ['Invisibility', 'High damage', 'Good mobility'],
    weaknesses: ['Fragile', 'Vulnerable to detection', 'Needs farm']
  },
  {
    id: 'drow-ranger',
    name: 'Drow Ranger',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['defensive', 'aggressive', 'experimental'],
    description: 'A ranged carry with global aura and silence abilities',
    strengths: ['Long range', 'Global aura', 'Strong push'],
    weaknesses: ['No escape', 'Vulnerable to gap closers', 'Needs positioning']
  },
  {
    id: 'ember-spirit',
    name: 'Ember Spirit',
    role: 'Mid',
    difficulty: 'Hard',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A mobile mid hero with fire-based abilities and escape',
    strengths: ['High mobility', 'Good escape', 'Strong teamfight'],
    weaknesses: ['Fragile', 'Mana issues', 'Complex mechanics']
  },
  {
    id: 'faceless-void',
    name: 'Faceless Void',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A time-manipulating carry with game-changing ultimate',
    strengths: ['Game-changing ultimate', 'Good escape', 'Strong late game'],
    weaknesses: ['Ultimate can hurt team', 'Needs farm', 'Timing crucial']
  },
  {
    id: 'gyrocopter',
    name: 'Gyrocopter',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A flying carry with powerful AOE abilities',
    strengths: ['Strong AOE', 'Good farming', 'Versatile'],
    weaknesses: ['Mana issues', 'Needs positioning', 'Vulnerable early']
  },
  {
    id: 'hoodwink',
    name: 'Hoodwink',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A tricky support with tree-based abilities and long-range ultimate',
    strengths: ['Tree mechanics', 'Long range', 'Good disable'],
    weaknesses: ['Fragile', 'Tree dependent', 'Skill shot reliant']
  },
  {
    id: 'juggernaut',
    name: 'Juggernaut',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'A versatile carry with magic immunity and healing abilities',
    strengths: ['Magic immunity', 'Built-in heal', 'Strong at all stages'],
    weaknesses: ['Vulnerable during omnislash', 'Limited disable', 'Predictable']
  },
  {
    id: 'lone-druid',
    name: 'Lone Druid',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['experimental', 'creative', 'aggressive'],
    description: 'A bear-summoning carry with micro-intensive gameplay',
    strengths: ['Two units', 'Strong push', 'Versatile builds'],
    weaknesses: ['Micro intensive', 'Complex', 'Falls off without items']
  },
  {
    id: 'luna',
    name: 'Luna',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A lunar carry with global aura and bouncing attacks',
    strengths: ['Bouncing attacks', 'Global aura', 'Strong farming'],
    weaknesses: ['No escape', 'Fragile', 'Needs positioning']
  },
  {
    id: 'marci',
    name: 'Marci',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'A mobile support with powerful movement and buff abilities',
    strengths: ['High mobility', 'Strong buffs', 'Good initiation'],
    weaknesses: ['Fragile', 'Complex timing', 'Mana issues']
  },
  {
    id: 'medusa',
    name: 'Medusa',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['defensive', 'experimental', 'aggressive'],
    description: 'A tanky carry who becomes nearly unkillable late game',
    strengths: ['Very tanky late', 'Split shot', 'Strong teamfight'],
    weaknesses: ['Very slow start', 'Needs lots of farm', 'Vulnerable early']
  },
  {
    id: 'meepo',
    name: 'Meepo',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['chaos', 'experimental', 'creative'],
    description: 'A unique carry who clones himself multiple times',
    strengths: ['Multiple units', 'Fast farming', 'Overwhelming presence'],
    weaknesses: ['Very micro intensive', 'Dies if one dies', 'Extremely complex']
  },
  {
    id: 'mirana',
    name: 'Mirana',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A versatile hero with long-range abilities and invisibility',
    strengths: ['Long range', 'Versatile', 'Good roaming'],
    weaknesses: ['Skill shot dependent', 'Squishy', 'Needs positioning']
  },
  {
    id: 'monkey-king',
    name: 'Monkey King',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A versatile carry with tree mechanics and transformation',
    strengths: ['Tree mechanics', 'Versatile', 'Strong critical'],
    weaknesses: ['Tree dependent', 'Complex positioning', 'Needs practice']
  },
  {
    id: 'morphling',
    name: 'Morphling',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['experimental', 'creative', 'aggressive'],
    description: 'A shapeshifting carry with incredible versatility',
    strengths: ['Stat morphing', 'Replication', 'High skill ceiling'],
    weaknesses: ['Very complex', 'Fragile when morphed', 'High skill requirement']
  },
  {
    id: 'naga-siren',
    name: 'Naga Siren',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['experimental', 'defensive', 'creative'],
    description: 'An illusion-based carry with sleep ultimate',
    strengths: ['Illusion farming', 'Game-changing ultimate', 'Split push'],
    weaknesses: ['Micro intensive', 'Complex timing', 'Team coordination needed']
  },
  {
    id: 'nyx-assassin',
    name: 'Nyx Assassin',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'An invisible assassin with mana burn and reflect abilities',
    strengths: ['Invisibility', 'Mana burn', 'Spell reflection'],
    weaknesses: ['Fragile', 'Falls off late', 'Vulnerable to detection']
  },
  {
    id: 'pangolier',
    name: 'Pangolier',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'creative', 'chaos'],
    description: 'A mobile initiator who rolls around the battlefield',
    strengths: ['High mobility', 'Magic immunity', 'Good disable'],
    weaknesses: ['Complex mechanics', 'Positioning crucial', 'Falls off late']
  },
  {
    id: 'phantom-assassin',
    name: 'Phantom Assassin',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A deadly carry with high critical damage potential',
    strengths: ['High crit damage', 'Evasion', 'Good scaling'],
    weaknesses: ['RNG dependent', 'Vulnerable to MKB', 'Needs farm']
  },
  {
    id: 'phantom-lancer',
    name: 'Phantom Lancer',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['creative', 'experimental', 'chaos'],
    description: 'An illusion-based carry that confuses enemies with multiple copies',
    strengths: ['Hard to kill', 'Illusion army', 'Good split push'],
    weaknesses: ['Weak to AOE', 'Slow start', 'Micro intensive']
  },
  {
    id: 'razor',
    name: 'Razor',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'An electric carry who steals damage from enemies',
    strengths: ['Damage steal', 'Good farming', 'Strong teamfight'],
    weaknesses: ['Needs to stay close', 'Kitable', 'Predictable']
  },
  {
    id: 'riki',
    name: 'Riki',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A permanently invisible carry who backstabs enemies',
    strengths: ['Permanent invisibility', 'Backstab damage', 'Good ganking'],
    weaknesses: ['Vulnerable to detection', 'Needs farm', 'Predictable']
  },
  {
    id: 'shadow-fiend',
    name: 'Shadow Fiend',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'A farming mid hero with high damage potential and map presence',
    strengths: ['High damage', 'Fast farming', 'Strong teamfight'],
    weaknesses: ['No escape', 'Needs souls', 'Vulnerable to ganks']
  },
  {
    id: 'slark',
    name: 'Slark',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'An agile carry that grows stronger by killing enemies',
    strengths: ['High mobility', 'Steals stats', 'Good escape'],
    weaknesses: ['Weak to detection', 'Needs kills', 'Vulnerable to disables']
  },
  {
    id: 'sniper',
    name: 'Sniper',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['defensive', 'aggressive', 'experimental'],
    description: 'A long-range carry that excels at staying safe while dealing damage',
    strengths: ['Long range', 'High damage', 'Good harass'],
    weaknesses: ['No escape', 'Fragile', 'Positioning crucial']
  },
  {
    id: 'spectre',
    name: 'Spectre',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['defensive', 'experimental', 'aggressive'],
    description: 'A late-game carry with global presence and damage reflection',
    strengths: ['Global ultimate', 'Very strong late', 'Damage reflection'],
    weaknesses: ['Weak early game', 'Needs lots of farm', 'Slow farmer']
  },
  {
    id: 'templar-assassin',
    name: 'Templar Assassin',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'A mid assassin with refraction shields and trap mechanics',
    strengths: ['High damage', 'Refraction shields', 'Good burst'],
    weaknesses: ['Vulnerable to DOT', 'Needs positioning', 'Falls off late']
  },
  {
    id: 'terrorblade',
    name: 'Terrorblade',
    role: 'Carry',
    difficulty: 'Hard',
    moods: ['experimental', 'aggressive', 'creative'],
    description: 'A metamorphosis carry with illusions and health swap',
    strengths: ['Illusion army', 'Health swap', 'Strong push'],
    weaknesses: ['Fragile base form', 'Micro intensive', 'Complex mechanics']
  },
  {
    id: 'troll-warlord',
    name: 'Troll Warlord',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A berserker carry who switches between melee and ranged',
    strengths: ['High attack speed', 'Form switching', 'Strong late game'],
    weaknesses: ['Kitable', 'Needs items', 'Predictable']
  },
  {
    id: 'ursa',
    name: 'Ursa',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A fury-driven carry who stacks damage with each attack',
    strengths: ['High damage potential', 'Good Roshan killer', 'Strong early'],
    weaknesses: ['Kitable', 'No escape', 'Predictable']
  },
  {
    id: 'vengeful-spirit',
    name: 'Vengeful Spirit',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'A vengeful support with swap and aura abilities',
    strengths: ['Powerful swap', 'Good aura', 'Strong disable'],
    weaknesses: ['Fragile', 'Positioning crucial', 'Falls off late']
  },
  {
    id: 'venomancer',
    name: 'Venomancer',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A poisonous support with strong harass and team fight',
    strengths: ['Strong harass', 'Good teamfight', 'Ward vision'],
    weaknesses: ['Very fragile', 'No escape', 'Positioning crucial']
  },
  {
    id: 'viper',
    name: 'Viper',
    role: 'Carry',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'A poisonous carry with strong lane presence',
    strengths: ['Strong laning', 'Good harass', 'Poison damage'],
    weaknesses: ['Low mobility', 'Falls off late', 'Predictable']
  },
  {
    id: 'weaver',
    name: 'Weaver',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'A time-weaving carry with invisibility and time travel',
    strengths: ['Invisibility', 'Time travel', 'High mobility'],
    weaknesses: ['Fragile', 'Mana issues', 'Needs farm']
  },

  // Intelligence Heroes
  {
    id: 'ancient-apparition',
    name: 'Ancient Apparition',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'An ice mage with global ultimate and healing prevention',
    strengths: ['Global ultimate', 'Prevents healing', 'Long range'],
    weaknesses: ['Very fragile', 'Skill shot dependent', 'Positioning crucial']
  },
  {
    id: 'bane',
    name: 'Bane',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'defensive', 'creative'],
    description: 'A nightmare support with powerful single-target disables',
    strengths: ['Strong disables', 'Good harassment', 'Sleep mechanics'],
    weaknesses: ['Fragile', 'Single target focus', 'Positioning crucial']
  },
  {
    id: 'batrider',
    name: 'Batrider',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A mobile initiator who can drag enemies with lasso',
    strengths: ['BKB-piercing disable', 'High mobility', 'Good initiation'],
    weaknesses: ['Fragile', 'Mana hungry', 'Falls off late']
  },
  {
    id: 'chen',
    name: 'Chen',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['experimental', 'creative', 'defensive'],
    description: 'A holy support who converts creeps and provides team healing',
    strengths: ['Creep conversion', 'Global heal', 'Strong early push'],
    weaknesses: ['Very complex', 'Micro intensive', 'Falls off late']
  },
  {
    id: 'crystal-maiden',
    name: 'Crystal Maiden',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'chaos'],
    description: 'A fragile but powerful support with devastating ultimates',
    strengths: ['High magical damage', 'Mana aura', 'Strong teamfight'],
    weaknesses: ['Very fragile', 'Low mobility', 'Positioning crucial']
  },
  {
    id: 'dark-seer',
    name: 'Dark Seer',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['experimental', 'aggressive', 'creative'],
    description: 'A utility initiator with illusion wall and speed boosts',
    strengths: ['Illusion wall', 'Speed boost', 'Good farming'],
    weaknesses: ['Complex mechanics', 'Team dependent', 'Needs positioning']
  },
  {
    id: 'dark-willow',
    name: 'Dark Willow',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A tricky support with multiple disables and terrain abilities',
    strengths: ['Multiple disables', 'Terrain manipulation', 'High utility'],
    weaknesses: ['Very complex', 'Fragile', 'High skill requirement']
  },
  {
    id: 'dazzle',
    name: 'Dazzle',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['defensive', 'experimental', 'aggressive'],
    description: 'A grave-saving support with powerful healing abilities',
    strengths: ['Grave save', 'Strong heals', 'Armor manipulation'],
    weaknesses: ['Fragile', 'Mana dependent', 'Positioning crucial']
  },
  {
    id: 'death-prophet',
    name: 'Death Prophet',
    role: 'Mid',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A ghostly mid hero with spirits and strong push',
    strengths: ['Strong push', 'Good sustain', 'Exorcism spirits'],
    weaknesses: ['No escape', 'Vulnerable to silence', 'Ultimate timing crucial']
  },
  {
    id: 'disruptor',
    name: 'Disruptor',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'A storm support with field control and glimpse mechanics',
    strengths: ['Field control', 'Glimpse mechanics', 'Strong teamfight'],
    weaknesses: ['Fragile', 'Skill dependent', 'Positioning crucial']
  },
  {
    id: 'enchantress',
    name: 'Enchantress',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['experimental', 'creative', 'aggressive'],
    description: 'A nature support who charms creeps and has pure damage',
    strengths: ['Creep control', 'Pure damage', 'Strong harass'],
    weaknesses: ['Fragile', 'Vulnerable to burst', 'Micro needed']
  },
  {
    id: 'enigma',
    name: 'Enigma',
    role: 'Initiator',
    difficulty: 'Medium',
    moods: ['defensive', 'experimental', 'aggressive'],
    description: 'A jungle-farming initiator with black hole ultimate',
    strengths: ['Game-changing ultimate', 'Fast jungle', 'Good push'],
    weaknesses: ['Channeling ultimate', 'Vulnerable to interruption', 'Needs positioning']
  },
  {
    id: 'grimstroke',
    name: 'Grimstroke',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'An ink-wielding support with binding and phantom abilities',
    strengths: ['Soul bind', 'Long range', 'Good teamfight'],
    weaknesses: ['Fragile', 'Skill shot dependent', 'Positioning crucial']
  },
  {
    id: 'invoker',
    name: 'Invoker',
    role: 'Mid',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A versatile mage with countless spell combinations',
    strengths: ['Versatile spells', 'High damage', 'Great scaling'],
    weaknesses: ['Complex mechanics', 'Mana hungry', 'Needs levels']
  },
  {
    id: 'jakiro',
    name: 'Jakiro',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'A twin-headed dragon with ice and fire abilities',
    strengths: ['Dual damage types', 'Good disable', 'Strong push'],
    weaknesses: ['Slow projectiles', 'Fragile', 'Positioning important']
  },
  {
    id: 'keeper-of-the-light',
    name: 'Keeper of the Light',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['defensive', 'experimental', 'creative'],
    description: 'A light-wielding support with mana provision and global abilities',
    strengths: ['Mana provision', 'Good wave clear', 'Global abilities'],
    weaknesses: ['Channeling spells', 'Fragile', 'Positioning crucial']
  },
  {
    id: 'leshrac',
    name: 'Leshrac',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A lightning-wielding mid with strong push and teamfight',
    strengths: ['Strong push', 'High magical damage', 'Good farming'],
    weaknesses: ['Mana hungry', 'Fragile', 'Vulnerable to silence']
  },
  {
    id: 'lich',
    name: 'Lich',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['defensive', 'aggressive', 'experimental'],
    description: 'An ice support with frost armor and chain frost',
    strengths: ['Frost armor', 'Chain frost', 'Good harass'],
    weaknesses: ['Fragile', 'Limited mobility', 'Falls off late']
  },
  {
    id: 'lina',
    name: 'Lina',
    role: 'Mid',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A fire mage with high burst damage and attack speed',
    strengths: ['High burst', 'Long range', 'Good scaling'],
    weaknesses: ['Fragile', 'Skill shot dependent', 'Positioning crucial']
  },
  {
    id: 'lion',
    name: 'Lion',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'chaos'],
    description: 'A versatile support with multiple disables and high burst',
    strengths: ['Multiple disables', 'High burst', 'Mana drain'],
    weaknesses: ['Very fragile', 'Short range', 'Positioning crucial']
  },
  {
    id: 'muerta',
    name: 'Muerta',
    role: 'Carry',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'creative'],
    description: 'A ghostly gunslinger with dual-form combat abilities',
    strengths: ['Dual forms', 'High damage', 'Unique mechanics'],
    weaknesses: ['Complex mechanics', 'Positioning crucial', 'Form management']
  },
  {
    id: 'nature-prophet',
    name: "Nature's Prophet",
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['creative', 'experimental', 'defensive'],
    description: 'A global presence hero who excels at split pushing and farming',
    strengths: ['Global teleport', 'Fast farming', 'Good push'],
    weaknesses: ['Squishy', 'Needs farm', 'Vulnerable when alone']
  },
  {
    id: 'necrophos',
    name: 'Necrophos',
    role: 'Mid',
    difficulty: 'Easy',
    moods: ['defensive', 'aggressive', 'experimental'],
    description: 'A tanky support/core hybrid with strong healing and execution',
    strengths: ['Strong sustain', 'Execute ultimate', 'Tanky'],
    weaknesses: ['Low mobility', 'Mana dependent', 'Vulnerable to burst']
  },
  {
    id: 'ogre-magi',
    name: 'Ogre Magi',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A tanky support with multicast abilities and strong buffs',
    strengths: ['Very tanky', 'Multicast RNG', 'Strong buffs'],
    weaknesses: ['RNG dependent', 'Mana issues', 'Limited range']
  },
  {
    id: 'oracle',
    name: 'Oracle',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'defensive'],
    description: 'A complex support with fate-altering abilities',
    strengths: ['Fate manipulation', 'Strong save', 'Purge abilities'],
    weaknesses: ['Very complex', 'Can hurt allies', 'High skill requirement']
  },
  {
    id: 'outworld-destroyer',
    name: 'Outworld Destroyer',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'An astral mid hero with mana burn and sanity eclipse',
    strengths: ['Mana burn', 'High pure damage', 'Astral mechanics'],
    weaknesses: ['Vulnerable to gap closers', 'Mana dependent', 'Needs farm']
  },
  {
    id: 'puck',
    name: 'Puck',
    role: 'Mid',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A mobile trickster with phase shift and dream coil',
    strengths: ['High mobility', 'Phase shift', 'Good disable'],
    weaknesses: ['Fragile', 'Complex mechanics', 'Falls off late']
  },
  {
    id: 'pugna',
    name: 'Pugna',
    role: 'Mid',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A building-destroying mid with ward and life drain',
    strengths: ['Destroys buildings', 'Nether ward', 'Life drain'],
    weaknesses: ['Very fragile', 'Positioning crucial', 'Vulnerable to burst']
  },
  {
    id: 'queen-of-pain',
    name: 'Queen of Pain',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A mobile mid assassin with blink and sonic wave',
    strengths: ['High mobility', 'Good burst', 'Blink escape'],
    weaknesses: ['Fragile', 'Falls off late', 'Needs good positioning']
  },
  {
    id: 'rubick',
    name: 'Rubick',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['creative', 'experimental', 'chaos'],
    description: 'A spell-stealing support who can turn enemy abilities against them',
    strengths: ['Spell steal', 'Good disable', 'High utility'],
    weaknesses: ['Very fragile', 'Complex timing', 'Needs good reactions']
  },
  {
    id: 'shadow-demon',
    name: 'Shadow Demon',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['experimental', 'creative', 'aggressive'],
    description: 'A shadow support with illusion and disruption mechanics',
    strengths: ['Illusion creation', 'Disruption save', 'Soul catcher'],
    weaknesses: ['Very fragile', 'Complex mechanics', 'Positioning crucial']
  },
  {
    id: 'shadow-shaman',
    name: 'Shadow Shaman',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'defensive'],
    description: 'A shamanic support with hex and serpent wards',
    strengths: ['Multiple disables', 'Serpent wards', 'Good push'],
    weaknesses: ['Fragile', 'Channeling ultimate', 'Positioning crucial']
  },
  {
    id: 'silencer',
    name: 'Silencer',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'experimental'],
    description: 'An intelligence-stealing support with global silence',
    strengths: ['Global silence', 'Int steal', 'Good harass'],
    weaknesses: ['Fragile', 'Limited mobility', 'Needs positioning']
  },
  {
    id: 'skywrath-mage',
    name: 'Skywrath Mage',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A high-damage support with infinite mana and burst potential',
    strengths: ['High magical damage', 'Infinite mana', 'Long range'],
    weaknesses: ['Very fragile', 'No disable', 'Positioning crucial']
  },
  {
    id: 'storm-spirit',
    name: 'Storm Spirit',
    role: 'Mid',
    difficulty: 'Hard',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A highly mobile mid hero with infinite escape potential',
    strengths: ['High mobility', 'Good escape', 'Strong ganking'],
    weaknesses: ['Mana hungry', 'Needs levels', 'Vulnerable when low mana']
  },
  {
    id: 'techies',
    name: 'Techies',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['chaos', 'creative', 'experimental'],
    description: 'A unique hero that controls the map through mines and traps',
    strengths: ['Map control', 'Unique playstyle', 'Psychological warfare'],
    weaknesses: ['Team dependent', 'Complex positioning', 'Unconventional']
  },
  {
    id: 'tinker',
    name: 'Tinker',
    role: 'Mid',
    difficulty: 'Hard',
    moods: ['creative', 'chaos', 'experimental'],
    description: 'A mobile nuker who can reset cooldowns and control the map',
    strengths: ['High mobility', 'Constant nuking', 'Map control'],
    weaknesses: ['Very complex', 'Mana hungry', 'Needs levels and items']
  },
  {
    id: 'tormentor',
    name: 'Tormentor',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['aggressive', 'experimental', 'chaos'],
    description: 'A demonic support with reflection and torment abilities',
    strengths: ['Damage reflection', 'Good harass', 'Strong teamfight'],
    weaknesses: ['Fragile', 'Positioning crucial', 'Complex timing']
  },
  {
    id: 'visage',
    name: 'Visage',
    role: 'Support',
    difficulty: 'Hard',
    moods: ['experimental', 'creative', 'aggressive'],
    description: 'A gargoyle summoner with familiar micro and grave chill',
    strengths: ['Familiar summons', 'Damage reduction', 'Good nuke'],
    weaknesses: ['Micro intensive', 'Fragile', 'Complex mechanics']
  },
  {
    id: 'void-spirit',
    name: 'Void Spirit',
    role: 'Mid',
    difficulty: 'Hard',
    moods: ['aggressive', 'creative', 'experimental'],
    description: 'A void-traveling mid with portal mechanics and high mobility',
    strengths: ['High mobility', 'Portal mechanics', 'Good burst'],
    weaknesses: ['Complex mechanics', 'Fragile', 'High skill requirement']
  },
  {
    id: 'warlock',
    name: 'Warlock',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['defensive', 'aggressive', 'experimental'],
    description: 'A demonic support with golem summoning and team healing',
    strengths: ['Golem ultimate', 'Strong teamfight', 'Good heal'],
    weaknesses: ['Long cooldowns', 'Channeling ultimate', 'Positioning crucial']
  },
  {
    id: 'windranger',
    name: 'Windranger',
    role: 'Mid',
    difficulty: 'Medium',
    moods: ['creative', 'experimental', 'aggressive'],
    description: 'A versatile hero with escape, disable, and high DPS potential',
    strengths: ['Versatile', 'Good escape', 'High single target damage'],
    weaknesses: ['Skill shot dependent', 'Needs farm', 'Mana issues']
  },
  {
    id: 'winter-wyvern',
    name: 'Winter Wyvern',
    role: 'Support',
    difficulty: 'Medium',
    moods: ['defensive', 'experimental', 'creative'],
    description: 'An ice dragon support with curse and arctic burn',
    strengths: ['Winter curse', 'Percent damage', 'Flying vision'],
    weaknesses: ['Fragile', 'Complex curse timing', 'Positioning crucial']
  },
  {
    id: 'witch-doctor',
    name: 'Witch Doctor',
    role: 'Support',
    difficulty: 'Easy',
    moods: ['aggressive', 'defensive', 'chaos'],
    description: 'A healing support with powerful teamfight abilities',
    strengths: ['Strong heal', 'Good disable', 'High teamfight damage'],
    weaknesses: ['Channeling abilities', 'Fragile', 'Positioning crucial']
  },
  {
    id: 'zeus',
    name: 'Zeus',
    role: 'Mid',
    difficulty: 'Easy',
    moods: ['aggressive', 'chaos', 'experimental'],
    description: 'A magical nuker with global presence and high burst damage',
    strengths: ['High magical damage', 'Global ultimate', 'True sight'],
    weaknesses: ['Very fragile', 'No escape', 'Mana dependent']
  }
];