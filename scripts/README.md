# Hero and Build Addition Scripts

This directory contains scripts to help you easily add new heroes and builds to your Dota 2 Supabase database.

## Files Overview

- `add-hero-build.ts` - Main script to add heroes and builds
- `hero-templates.ts` - Templates and examples for creating heroes and builds
- `populate-database.ts` - Original script to populate the entire database

## Quick Start

### 1. Add a New Hero

1. Open `add-hero-build.ts`
2. Replace the example hero in the `newHeroes` array with your hero data
3. Run the script

### 2. Add a New Build

1. Open `add-hero-build.ts`
2. Replace the example build in the `newBuilds` array with your build data
3. Run the script

### 3. Run the Script

```bash
npm run tsx scripts/add-hero-build.ts
```

## Step-by-Step Guide

### Creating a New Hero

1. **Copy the template** from `hero-templates.ts`:
```typescript
{
  id: 'invoker',                    // Unique ID (lowercase, dashes)
  name: 'Invoker',                  // Display name
  role: 'Mid',                      // Role: Carry/Support/Mid/Initiator
  difficulty: 'Hard',               // Easy/Medium/Hard
  moods: ['creative', 'experimental'], // Available moods
  description: 'A versatile mage with countless spell combinations',
  strengths: [
    'Versatile spells',
    'High damage potential',
    'Great scaling',
    'Multiple abilities'
  ],
  weaknesses: [
    'Very complex',
    'Mana hungry',
    'Needs levels',
    'High skill requirement'
  ]
}
```

2. **Fill in the details**:
   - `id`: Unique identifier (use lowercase with dashes)
   - `name`: Hero's display name
   - `role`: Choose from Carry, Support, Mid, Initiator
   - `difficulty`: Easy, Medium, or Hard
   - `moods`: Array of moods this hero fits (aggressive, defensive, experimental, creative, chaos)
   - `description`: Short description of the hero
   - `strengths`: 3-5 hero strengths
   - `weaknesses`: 3-5 hero weaknesses

### Creating a New Build

1. **Copy the template** from `hero-templates.ts`:
```typescript
{
  heroId: 'invoker',                // Must match hero ID
  mood: 'creative',                 // Mood this build represents
  items: [
    {
      id: 'boots',
      name: 'Boots of Speed',
      cost: 500,
      phase: 'Early',               // Early/Mid/Late
      priority: 'Core',             // Core/Situational/Luxury
      description: 'Basic mobility'
    },
    // Add 4-8 items total
  ],
  playstyle: {
    dos: [                          // 4-6 things to do
      'Invoke spells efficiently',
      'Use Quas for regen',
      'Practice spell combinations'
    ],
    donts: [                        // 4-6 things not to do
      "Don't waste mana",
      "Don't forget to invoke",
      "Don't neglect positioning"
    ],
    tips: [                         // 4-6 pro tips
      'Practice in demo mode',
      'Learn common combos',
      'Use invoke hotkey'
    ]
  },
  gameplan: {
    early: 'Farm safely, get levels, practice invoking',
    mid: 'Get key items, join fights, use spell combinations',
    late: 'Become a major damage dealer with full spell arsenal'
  }
}
```

2. **Fill in the details**:
   - `heroId`: Must match an existing hero's ID
   - `mood`: The playstyle mood (aggressive, defensive, experimental, creative, chaos)
   - `items`: 4-8 items in order of purchase
   - `playstyle.dos`: Things the player should do
   - `playstyle.donts`: Things the player should avoid
   - `playstyle.tips`: Pro tips and advanced strategies
   - `gameplan`: Strategy for early, mid, and late game

## Running the Script

### Prerequisites

Make sure you have:
1. Your `.env` file configured with Supabase credentials
2. The database schema set up (run migrations)

### Commands

```bash
# Add heroes and builds
npm run tsx scripts/add-hero-build.ts

# Populate entire database (first time only)
npm run db:populate

# Check types
npm run tsx scripts/hero-templates.ts
```

### Success Indicators

You should see output like:
```
🚀 Adding new heroes and builds...

🦸 Adding hero: Invoker...
✅ Successfully added hero: Invoker

🔨 Adding build: Invoker (creative)...
✅ Successfully added build: Invoker (creative)

📊 Summary:
✅ Successfully added: 2 items
❌ Failed: 0 items
🎉 All heroes and builds added successfully!
```

## Common Issues & Solutions

### 1. "Missing Supabase environment variables"
- Make sure your `.env` file has `VITE_SUPABASE_URL` and `SUPABASE_SERVICE_ROLE_KEY`

### 2. "Error inserting hero: duplicate key value"
- Hero ID already exists, use a different ID or update the existing hero

### 3. "Error inserting build: foreign key constraint"
- The `heroId` in your build doesn't match any existing hero ID

### 4. "Error inserting item: check constraint"
- Invalid `phase` (must be Early/Mid/Late) or `priority` (must be Core/Situational/Luxury)

### 5. "Error inserting mood: check constraint"
- Invalid mood (must be aggressive/defensive/experimental/creative/chaos)

## Best Practices

1. **Test First**: Add one hero/build at a time to test
2. **Use Templates**: Copy from `hero-templates.ts` to avoid typos
3. **Consistent Naming**: Use kebab-case for IDs (e.g., `anti-mage`, `queen-of-pain`)
4. **Realistic Costs**: Use actual Dota 2 item costs
5. **Clear Descriptions**: Write helpful item descriptions and playstyle tips
6. **Balanced Builds**: Include early, mid, and late game items

## Data Structure Reference

### Hero Object
```typescript
interface Hero {
  id: string;                       // Unique identifier
  name: string;                     // Display name
  role: 'Carry' | 'Support' | 'Mid' | 'Initiator';
  difficulty: 'Easy' | 'Medium' | 'Hard';
  moods: Mood[];                    // Available moods
  description: string;              // Short description
  strengths: string[];              // 3-5 strengths
  weaknesses: string[];             // 3-5 weaknesses
}
```

### Build Object
```typescript
interface Build {
  heroId: string;                   // Must match hero ID
  mood: Mood;                       // Build mood
  items: Item[];                    // 4-8 items
  playstyle: {
    dos: string[];                  // 4-6 dos
    donts: string[];                // 4-6 donts
    tips: string[];                 // 4-6 tips
  };
  gameplan: {
    early: string;                  // Early game plan
    mid: string;                    // Mid game plan
    late: string;                   // Late game plan
  };
}
```

### Item Object
```typescript
interface Item {
  id: string | number;              // Unique identifier
  name: string;                     // Item name
  cost: number;                     // Gold cost
  phase: 'Early' | 'Mid' | 'Late'; // Game phase
  priority: 'Core' | 'Situational' | 'Luxury'; // Priority
  description: string;              // Item description
}
```

## Need Help?

If you encounter issues:
1. Check the console output for specific error messages
2. Verify your data matches the templates
3. Make sure your `.env` file is configured correctly
4. Check that your database schema is up to date

Happy hero and build creation! 🎮