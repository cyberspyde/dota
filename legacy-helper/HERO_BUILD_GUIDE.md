# 🎮 Dota 2 Hero and Build Adding Guide

A comprehensive guide for adding heroes and builds to your Dota 2 Supabase database.

## 📋 Table of Contents
- [Quick Start](#quick-start)
- [Hero Data Structure](#hero-data-structure)
- [Build Data Structure](#build-data-structure)
- [Adding Heroes](#adding-heroes)
- [Adding Builds](#adding-builds)
- [Data Validation Rules](#data-validation-rules)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Common Mistakes](#common-mistakes)
- [Troubleshooting](#troubleshooting)

---

## 🚀 Quick Start

### Method 1: TypeScript Script (Recommended for developers)
```bash
# Edit the script with your data
nano scripts/add-hero-build.ts

# Run the script
npm run tsx scripts/add-hero-build.ts
```

### Method 2: Python Script (Recommended for content creators)
```bash
# Install dependencies
pip install -r scripts/requirements.txt

# Create templates
python scripts/manage_dota_data.py create-templates

# Add heroes and builds
python scripts/manage_dota_data.py add-hero --json hero_template.json
python scripts/manage_dota_data.py add-build --json build_template.json
```

---

## 🦸 Hero Data Structure

### Required Fields
```typescript
interface Hero {
  id: string;                    // Unique identifier
  name: string;                  // Display name
  role: string;                  // Game role
  difficulty: string;            // Skill level required
  moods: string[];               // Playstyle moods
  description: string;           // Hero description
  strengths: string[];           // Hero advantages
  weaknesses: string[];          // Hero disadvantages
}
```

### Field Specifications

#### **ID Field**
- **Format**: `kebab-case` (lowercase with hyphens)
- **Examples**: `anti-mage`, `crystal-maiden`, `queen-of-pain`
- **Rules**: Must be unique, no spaces, no special characters

#### **Name Field**
- **Format**: Proper case with spaces
- **Examples**: `"Anti-Mage"`, `"Crystal Maiden"`, `"Queen of Pain"`
- **Rules**: Official Dota 2 hero name

#### **Role Field**
- **Valid Options**: `Carry`, `Support`, `Mid`, `Initiator`
- **Examples**: `"Carry"`, `"Support"`
- **Rules**: Must match exactly (case-sensitive)

#### **Difficulty Field**
- **Valid Options**: `Easy`, `Medium`, `Hard`
- **Examples**: `"Easy"`, `"Medium"`, `"Hard"`
- **Rules**: Must match exactly (case-sensitive)

#### **Moods Array**
- **Valid Options**: `aggressive`, `defensive`, `experimental`, `creative`, `chaos`
- **Examples**: `["aggressive", "experimental"]`
- **Rules**: 1-5 moods per hero, must be lowercase

#### **Description Field**
- **Format**: Single descriptive sentence
- **Length**: 50-150 characters
- **Example**: `"A mobile carry that excels at farming and split pushing"`

#### **Strengths Array**
- **Count**: 3-5 strengths
- **Format**: Short phrases (3-8 words)
- **Examples**: `["High mobility", "Fast farming", "Strong late game"]`

#### **Weaknesses Array**
- **Count**: 3-5 weaknesses
- **Format**: Short phrases (3-8 words)
- **Examples**: `["Weak early game", "Needs items", "Vulnerable to ganks"]`

---

## 🔨 Build Data Structure

### Required Fields
```typescript
interface Build {
  heroId: string;                // Must match existing hero ID
  mood: string;                  // Playstyle mood
  items: Item[];                 // Item progression
  playstyle: Playstyle;          // Gameplay guidance
  gameplan: Gameplan;            // Phase-specific strategy
}
```

### Item Structure
```typescript
interface Item {
  id: string | number;           // Unique identifier
  name: string;                  // Item name
  cost: number;                  // Gold cost
  phase: string;                 // Game phase
  priority: string;              // Item importance
  description: string;           // Item purpose
}
```

### Field Specifications

#### **Hero ID Field**
- **Format**: Must match existing hero's ID exactly
- **Example**: `"anti-mage"` (for Anti-Mage hero)
- **Rules**: Case-sensitive, must exist in heroes table

#### **Mood Field**
- **Valid Options**: `aggressive`, `defensive`, `experimental`, `creative`, `chaos`
- **Example**: `"aggressive"`
- **Rules**: Must be lowercase, match hero's available moods

#### **Items Array**
- **Count**: 4-8 items recommended
- **Order**: Early → Mid → Late game progression
- **Example**:
```json
[
  {
    "id": "boots",
    "name": "Boots of Speed",
    "cost": 500,
    "phase": "Early",
    "priority": "Core",
    "description": "Basic mobility"
  }
]
```

#### **Item Phases**
- **Valid Options**: `Early`, `Mid`, `Late`
- **Early**: 0-15 minutes, basic items
- **Mid**: 15-35 minutes, core items
- **Late**: 35+ minutes, luxury items

#### **Item Priorities**
- **Core**: Essential items for the build
- **Situational**: Depends on enemy team/game state
- **Luxury**: End-game items when ahead

#### **Playstyle Structure**
```typescript
interface Playstyle {
  dos: string[];                 // Things to do
  donts: string[];              // Things to avoid
  tips: string[];               // Pro tips
}
```

Each array should contain 3-6 items with clear, actionable advice.

#### **Gameplan Structure**
```typescript
interface Gameplan {
  early: string;                // 0-15 minutes strategy
  mid: string;                  // 15-35 minutes strategy
  late: string;                 // 35+ minutes strategy
}
```

Each phase should be 1-2 sentences describing the main objectives.

---

## 🦸 Adding Heroes

### Step 1: Choose Your Method

#### **TypeScript Method**
1. Open `scripts/add-hero-build.ts`
2. Replace the example hero in `newHeroes` array
3. Run `npm run tsx scripts/add-hero-build.ts`

#### **Python Method**
1. Create hero JSON file
2. Run `python scripts/manage_dota_data.py add-hero --json hero.json`

### Step 2: Hero Data Template

```json
{
  "id": "invoker",
  "name": "Invoker",
  "role": "Mid",
  "difficulty": "Hard",
  "moods": ["creative", "experimental"],
  "description": "A versatile mage with countless spell combinations and high skill ceiling",
  "strengths": [
    "Versatile spell combinations",
    "High magical damage output",
    "Strong at all game stages",
    "Great scaling potential"
  ],
  "weaknesses": [
    "Very complex mechanics",
    "Mana hungry early game",
    "Requires extensive practice",
    "Vulnerable to gap closers"
  ]
}
```

### Step 3: Validation Checklist

Before adding a hero, verify:
- [ ] ID is unique and uses kebab-case
- [ ] Name matches official Dota 2 spelling
- [ ] Role is one of: Carry, Support, Mid, Initiator
- [ ] Difficulty is one of: Easy, Medium, Hard
- [ ] Moods are valid and lowercase
- [ ] Description is 50-150 characters
- [ ] 3-5 strengths and weaknesses
- [ ] All text is clear and descriptive

---

## 🔨 Adding Builds

### Step 1: Verify Hero Exists

Before adding a build, ensure the hero exists in the database:
```bash
# Check if hero exists
python scripts/manage_dota_data.py list-heroes
```

### Step 2: Build Data Template

```json
{
  "heroId": "invoker",
  "mood": "creative",
  "items": [
    {
      "id": "boots",
      "name": "Boots of Speed",
      "cost": 500,
      "phase": "Early",
      "priority": "Core",
      "description": "Basic mobility for positioning"
    },
    {
      "id": "null-talisman",
      "name": "Null Talisman",
      "cost": 505,
      "phase": "Early",
      "priority": "Core",
      "description": "Early mana and damage"
    },
    {
      "id": "power-treads",
      "name": "Power Treads",
      "cost": 1400,
      "phase": "Early",
      "priority": "Core",
      "description": "Attack speed and stat switching"
    },
    {
      "id": "orchid",
      "name": "Orchid Malevolence",
      "cost": 3475,
      "phase": "Mid",
      "priority": "Core",
      "description": "Silence and damage amplification"
    },
    {
      "id": "aghs",
      "name": "Aghanim's Scepter",
      "cost": 4200,
      "phase": "Mid",
      "priority": "Core",
      "description": "Additional spell charges"
    },
    {
      "id": "octarine",
      "name": "Octarine Core",
      "cost": 5275,
      "phase": "Late",
      "priority": "Luxury",
      "description": "Cooldown reduction and spell lifesteal"
    }
  ],
  "playstyle": {
    "dos": [
      "Practice spell combinations in demo mode",
      "Use Quas for regeneration between fights",
      "Invoke spells before fights begin",
      "Position safely behind your team",
      "Learn efficient spell rotations"
    ],
    "donts": [
      "Don't waste mana on unnecessary spells",
      "Don't forget to invoke new spells",
      "Don't engage without proper positioning",
      "Don't neglect farming between fights",
      "Don't panic and misclick spells"
    ],
    "tips": [
      "Bind invoke to an easily accessible key",
      "Learn at least 5 common spell combinations",
      "Use Forge Spirits to farm and scout",
      "Cold Snap is excellent for ganking",
      "Refresher Orb doubles your team fight impact"
    ]
  },
  "gameplan": {
    "early": "Focus on farming and getting levels. Practice invoking spells and use Quas for sustain. Avoid unnecessary fights until you have core items.",
    "mid": "Get Orchid for ganking potential. Start participating in team fights with your spell combinations. Use Forge Spirits to farm efficiently.",
    "late": "Become a major team fight controller with Aghanim's Scepter. Use advanced spell combinations to devastate enemy teams."
  }
}
```

### Step 3: Build Validation Checklist

Before adding a build, verify:
- [ ] Hero ID matches existing hero exactly
- [ ] Mood is valid and matches hero's available moods
- [ ] 4-8 items in logical progression order
- [ ] Item phases progress from Early → Mid → Late
- [ ] Item costs are accurate (check Dota 2 wiki)
- [ ] Each playstyle section has 3-6 items
- [ ] Gameplan phases are 1-2 sentences each
- [ ] All advice is practical and actionable

---

## ✅ Data Validation Rules

### Hero Validation
- **ID**: Must be unique, kebab-case, no spaces
- **Name**: Required, 2-50 characters
- **Role**: Must be `Carry`, `Support`, `Mid`, or `Initiator`
- **Difficulty**: Must be `Easy`, `Medium`, or `Hard`
- **Moods**: 1-5 moods, must be valid mood strings
- **Description**: 50-150 characters
- **Strengths**: 3-5 items, each 3-50 characters
- **Weaknesses**: 3-5 items, each 3-50 characters

### Build Validation
- **Hero ID**: Must match existing hero
- **Mood**: Must be valid and match hero's moods
- **Items**: 4-8 items, each with required fields
- **Item Cost**: Must be positive integer
- **Item Phase**: Must be `Early`, `Mid`, or `Late`
- **Item Priority**: Must be `Core`, `Situational`, or `Luxury`
- **Playstyle**: Each section needs 3-6 items
- **Gameplan**: Each phase needs descriptive text

---

## 🎯 Best Practices

### Hero Creation
1. **Research First**: Study the hero in-game before writing
2. **Be Accurate**: Use official names and descriptions
3. **Be Specific**: Avoid vague strengths/weaknesses
4. **Consider All Players**: Write for different skill levels
5. **Stay Updated**: Reflect current meta and patches

### Build Creation
1. **Test Builds**: Try the build in-game first
2. **Explain Choices**: Clear item descriptions
3. **Logical Progression**: Items should build upon each other
4. **Practical Advice**: Focus on actionable tips
5. **Phase Planning**: Clear objectives for each game phase

### Writing Style
- **Clear and Concise**: Easy to understand
- **Actionable**: Focus on what players should do
- **Positive Tone**: Encourage learning and improvement
- **Consistent**: Use similar phrasing across entries

---

## 📚 Examples

### Example 1: Simple Carry Hero

```json
{
  "id": "juggernaut",
  "name": "Juggernaut",
  "role": "Carry",
  "difficulty": "Easy",
  "moods": ["aggressive", "defensive"],
  "description": "A versatile carry with magic immunity and healing abilities",
  "strengths": [
    "Built-in magic immunity",
    "Self-healing ability",
    "Strong at all game stages",
    "Good escape potential"
  ],
  "weaknesses": [
    "Vulnerable during omnislash",
    "Limited hard disable",
    "Predictable playstyle",
    "Needs farm to scale"
  ]
}
```

### Example 2: Complex Support Hero

```json
{
  "id": "rubick",
  "name": "Rubick",
  "role": "Support",
  "difficulty": "Hard",
  "moods": ["creative", "experimental"],
  "description": "A spell-stealing support who turns enemy abilities against them",
  "strengths": [
    "Spell steal versatility",
    "Strong positioning tools",
    "High utility potential",
    "Game-changing steals"
  ],
  "weaknesses": [
    "Very fragile",
    "Reaction-dependent",
    "Complex timing requirements",
    "Needs good game knowledge"
  ]
}
```

### Example 3: Aggressive Build

```json
{
  "heroId": "pudge",
  "mood": "aggressive",
  "items": [
    {
      "id": "boots",
      "name": "Boots of Speed",
      "cost": 500,
      "phase": "Early",
      "priority": "Core",
      "description": "Basic mobility for positioning"
    },
    {
      "id": "soul-ring",
      "name": "Soul Ring",
      "cost": 770,
      "phase": "Early",
      "priority": "Core",
      "description": "Mana sustain for hooks"
    },
    {
      "id": "tranquil",
      "name": "Tranquil Boots",
      "cost": 925,
      "phase": "Early",
      "priority": "Core",
      "description": "Health regen and movement"
    },
    {
      "id": "blink",
      "name": "Blink Dagger",
      "cost": 2250,
      "phase": "Mid",
      "priority": "Core",
      "description": "Initiation and positioning"
    },
    {
      "id": "aghs",
      "name": "Aghanim's Scepter",
      "cost": 4200,
      "phase": "Mid",
      "priority": "Core",
      "description": "Enhanced Dismember"
    },
    {
      "id": "heart",
      "name": "Heart of Tarrasque",
      "cost": 5000,
      "phase": "Late",
      "priority": "Luxury",
      "description": "Massive HP and regen"
    }
  ],
  "playstyle": {
    "dos": [
      "Practice hook accuracy in demo mode",
      "Use Rot to slow enemies and farm",
      "Position behind trees for surprise hooks",
      "Build HP items to increase Flesh Heap stacks",
      "Use Blink Dagger for better positioning"
    ],
    "donts": [
      "Don't waste hooks on creeps",
      "Don't hook enemies toward their team",
      "Don't neglect mana management",
      "Don't chase without backup",
      "Don't ignore last-hitting in lane"
    ],
    "tips": [
      "Hook has longer range than it appears",
      "Rot damages both you and enemies",
      "Dismember heals you while channeling",
      "Each kill increases your HP permanently",
      "Use trees and fog to hide hook animations"
    ]
  },
  "gameplan": {
    "early": "Lane with your carry, harass enemies with Rot, practice hooks, get Soul Ring for mana.",
    "mid": "Get Blink Dagger, start making plays around the map, hook key targets in team fights.",
    "late": "Become an unkillable tank, initiate fights with Blink + Hook, protect your carries."
  }
}
```

---

## ❌ Common Mistakes

### Hero Mistakes
1. **Wrong ID Format**: Using spaces or capitals
   - ❌ `"Anti Mage"` or `"Anti-Mage"`
   - ✅ `"anti-mage"`

2. **Invalid Role**: Using incorrect role names
   - ❌ `"DPS"` or `"Tank"`
   - ✅ `"Carry"` or `"Initiator"`

3. **Too Many Moods**: Assigning too many moods
   - ❌ `["aggressive", "defensive", "experimental", "creative", "chaos"]`
   - ✅ `["aggressive", "experimental"]`

4. **Vague Descriptions**: Non-specific strengths/weaknesses
   - ❌ `"Good hero"` or `"Hard to play"`
   - ✅ `"High mobility"` or `"Complex mechanics"`

### Build Mistakes
1. **Wrong Hero ID**: Not matching existing hero
   - ❌ `"heroId": "Anti-Mage"` (hero ID is `"anti-mage"`)
   - ✅ `"heroId": "anti-mage"`

2. **Incorrect Item Progression**: Items in wrong order
   - ❌ Luxury items before core items
   - ✅ Early → Mid → Late progression

3. **Unrealistic Costs**: Wrong or outdated item costs
   - ❌ `"cost": 1000` for Boots of Speed
   - ✅ `"cost": 500` for Boots of Speed

4. **Poor Advice**: Vague or unhelpful tips
   - ❌ `"Play well"` or `"Don't die"`
   - ✅ `"Use Blink Dagger for positioning"` or `"Avoid fighting without BKB"`

---

## 🔧 Troubleshooting

### Common Issues

#### "Hero ID doesn't exist"
**Problem**: Build references non-existent hero
**Solution**: Check hero exists with `list-heroes` command

#### "Invalid mood for hero"
**Problem**: Build mood not in hero's mood list
**Solution**: Check hero's available moods, add mood to hero if needed

#### "Validation failed"
**Problem**: Data doesn't meet validation requirements
**Solution**: Check validation rules and fix data format

#### "Duplicate hero ID"
**Problem**: Hero ID already exists
**Solution**: Choose unique ID or update existing hero

#### "Database connection failed"
**Problem**: Supabase credentials incorrect
**Solution**: Check `.env` file has correct URL and key

### Debug Steps
1. **Check Environment**: Verify `.env` file exists and has correct values
2. **Test Connection**: Run `list-heroes` to test database connection
3. **Validate Data**: Use `validate` command before adding
4. **Check Logs**: Look for specific error messages
5. **Restart Script**: Sometimes fixes temporary issues

### Getting Help
- Check the error message carefully
- Verify your JSON syntax is correct
- Ensure all required fields are present
- Test with the provided templates first
- Check the database schema matches your data

---

## 📞 Support

### Quick Reference
- **Heroes**: ID, name, role, difficulty, moods, description, strengths, weaknesses
- **Builds**: heroId, mood, items, playstyle, gameplan
- **Roles**: Carry, Support, Mid, Initiator
- **Difficulties**: Easy, Medium, Hard
- **Moods**: aggressive, defensive, experimental, creative, chaos
- **Phases**: Early, Mid, Late
- **Priorities**: Core, Situational, Luxury

### Useful Commands
```bash
# Create templates
python scripts/manage_dota_data.py create-templates

# Validate before adding
python scripts/manage_dota_data.py validate --json hero.json

# List existing heroes
python scripts/manage_dota_data.py list-heroes

# Add hero
python scripts/manage_dota_data.py add-hero --json hero.json

# Add build
python scripts/manage_dota_data.py add-build --json build.json
```

---

## 🎉 Final Tips

1. **Start Small**: Begin with simple heroes and builds
2. **Test Everything**: Verify builds work in-game
3. **Stay Organized**: Use consistent naming and formatting
4. **Keep Learning**: Study pro players and meta changes
5. **Be Patient**: Building a comprehensive database takes time

Remember: Quality over quantity. A few well-crafted heroes and builds are better than many poorly designed ones.

Happy hero and build creation! 🎮✨