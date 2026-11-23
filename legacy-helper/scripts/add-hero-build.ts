import { createClient } from '@supabase/supabase-js';
import { Hero, Build } from '../src/types';

// Load environment variables
import * as dotenv from 'dotenv';
dotenv.config();

const supabaseUrl = process.env.VITE_SUPABASE_URL || '';
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY || '';

if (!supabaseUrl || !supabaseServiceKey) {
  console.error('Missing Supabase environment variables');
  process.exit(1);
}

const supabase = createClient(supabaseUrl, supabaseServiceKey);

// Helper function to add a single hero
async function addHero(hero: Hero) {
  console.log(`🦸 Adding hero: ${hero.name}...`);
  
  try {
    // Insert hero
    const { error: heroError } = await supabase
      .from('heroes')
      .upsert({
        id: hero.id,
        name: hero.name,
        role: hero.role,
        difficulty: hero.difficulty,
        description: hero.description
      }, { onConflict: 'id' });

    if (heroError) {
      console.error(`❌ Error inserting hero ${hero.name}:`, heroError);
      return false;
    }

    // Insert hero moods
    for (const mood of hero.moods) {
      const { error: moodError } = await supabase
        .from('hero_moods')
        .upsert({
          hero_id: hero.id,
          mood: mood
        }, { onConflict: 'hero_id,mood' });

      if (moodError) {
        console.error(`❌ Error inserting mood for ${hero.name}:`, moodError);
      }
    }

    // Insert hero strengths
    for (let i = 0; i < hero.strengths.length; i++) {
      const { error: strengthError } = await supabase
        .from('hero_strengths')
        .upsert({
          hero_id: hero.id,
          strength: hero.strengths[i],
          order_index: i
        });

      if (strengthError) {
        console.error(`❌ Error inserting strength for ${hero.name}:`, strengthError);
      }
    }

    // Insert hero weaknesses
    for (let i = 0; i < hero.weaknesses.length; i++) {
      const { error: weaknessError } = await supabase
        .from('hero_weaknesses')
        .upsert({
          hero_id: hero.id,
          weakness: hero.weaknesses[i],
          order_index: i
        });

      if (weaknessError) {
        console.error(`❌ Error inserting weakness for ${hero.name}:`, weaknessError);
      }
    }

    console.log(`✅ Successfully added hero: ${hero.name}`);
    return true;
  } catch (error) {
    console.error(`❌ Failed to add hero ${hero.name}:`, error);
    return false;
  }
}

// Helper function to add a single build
async function addBuild(build: Build) {
  console.log(`🔨 Adding build: ${build.heroId} (${build.mood})...`);
  
  try {
    // Insert build
    const { data: buildData, error: buildError } = await supabase
      .from('builds')
      .upsert({
        hero_id: build.heroId,
        mood: build.mood,
        early_game: build.gameplan.early,
        mid_game: build.gameplan.mid,
        late_game: build.gameplan.late
      }, { onConflict: 'hero_id,mood' })
      .select('id')
      .single();

    if (buildError) {
      console.error(`❌ Error inserting build for ${build.heroId}:`, buildError);
      return false;
    }

    const buildId = buildData.id;

    // Insert items
    for (let i = 0; i < build.items.length; i++) {
      const item = build.items[i];
      const { error: itemError } = await supabase
        .from('items')
        .upsert({
          build_id: buildId,
          name: item.name,
          cost: item.cost,
          phase: item.phase,
          priority: item.priority,
          description: item.description,
          order_index: i
        });

      if (itemError) {
        console.error(`❌ Error inserting item for ${build.heroId}:`, itemError);
      }
    }

    // Insert playstyle dos
    for (let i = 0; i < build.playstyle.dos.length; i++) {
      const { error: doError } = await supabase
        .from('playstyle_dos')
        .upsert({
          build_id: buildId,
          do_item: build.playstyle.dos[i],
          order_index: i
        });

      if (doError) {
        console.error(`❌ Error inserting do item for ${build.heroId}:`, doError);
      }
    }

    // Insert playstyle donts
    for (let i = 0; i < build.playstyle.donts.length; i++) {
      const { error: dontError } = await supabase
        .from('playstyle_donts')
        .upsert({
          build_id: buildId,
          dont_item: build.playstyle.donts[i],
          order_index: i
        });

      if (dontError) {
        console.error(`❌ Error inserting dont item for ${build.heroId}:`, dontError);
      }
    }

    // Insert playstyle tips
    for (let i = 0; i < build.playstyle.tips.length; i++) {
      const { error: tipError } = await supabase
        .from('playstyle_tips')
        .upsert({
          build_id: buildId,
          tip: build.playstyle.tips[i],
          order_index: i
        });

      if (tipError) {
        console.error(`❌ Error inserting tip for ${build.heroId}:`, tipError);
      }
    }

    console.log(`✅ Successfully added build: ${build.heroId} (${build.mood})`);
    return true;
  } catch (error) {
    console.error(`❌ Failed to add build ${build.heroId}:`, error);
    return false;
  }
}

// Main function to add heroes and builds
async function addHeroesAndBuilds() {
  console.log('🚀 Adding new heroes and builds...\n');

  // ==============================================
  // ADD YOUR NEW HEROES HERE
  // ==============================================
  const newHeroes: Hero[] = [
    // Example hero - replace with your own
    {
      id: 'example-hero',
      name: 'Example Hero',
      role: 'Carry',
      difficulty: 'Medium',
      moods: ['aggressive', 'experimental'],
      description: 'An example hero for demonstration purposes',
      strengths: ['High damage', 'Good mobility', 'Strong late game'],
      weaknesses: ['Fragile early', 'Needs farm', 'Vulnerable to ganks']
    },
    // Add more heroes here...
  ];

  // ==============================================
  // ADD YOUR NEW BUILDS HERE
  // ==============================================
  const newBuilds: Build[] = [
    // Example build - replace with your own
    {
      heroId: 'example-hero',
      mood: 'aggressive',
      items: [
        { id: 'boots', name: 'Boots of Speed', cost: 500, phase: 'Early', priority: 'Core', description: 'Basic mobility' },
        { id: 'wraith-band', name: 'Wraith Band', cost: 505, phase: 'Early', priority: 'Core', description: 'Early stats' },
        { id: 'power-treads', name: 'Power Treads', cost: 1400, phase: 'Early', priority: 'Core', description: 'Attack speed and stats' },
        { id: 'bfury', name: 'Battle Fury', cost: 4100, phase: 'Mid', priority: 'Core', description: 'Farming and cleave' },
        { id: 'manta', name: 'Manta Style', cost: 4850, phase: 'Mid', priority: 'Core', description: 'Illusions and dispel' },
        { id: 'butterfly', name: 'Butterfly', cost: 5525, phase: 'Late', priority: 'Luxury', description: 'Evasion and attack speed' }
      ],
      playstyle: {
        dos: [
          'Farm efficiently with Battle Fury',
          'Split push with Manta illusions',
          'Join fights when you have key items',
          'Use mobility to avoid ganks'
        ],
        donts: [
          "Don't fight without key items",
          "Don't neglect farming",
          "Don't get caught without escape",
          "Don't ignore team fights completely"
        ],
        tips: [
          'Use Battle Fury to farm jungle stacks',
          'Manta can dispel debuffs',
          'Position aggressively but have exit plan',
          'Time your power spikes with team fights'
        ]
      },
      gameplan: {
        early: 'Focus on farming and getting Battle Fury as soon as possible. Avoid fights and ganks.',
        mid: 'Farm efficiently with Battle Fury, get Manta Style, start joining team fights.',
        late: 'Become a major damage dealer, use Butterfly for survivability and DPS.'
      }
    },
    // Add more builds here...
  ];

  // ==============================================
  // PROCESSING - DON'T EDIT BELOW THIS LINE
  // ==============================================
  
  let successCount = 0;
  let failCount = 0;

  // Add heroes
  console.log(`📝 Processing ${newHeroes.length} heroes...`);
  for (const hero of newHeroes) {
    const success = await addHero(hero);
    if (success) {
      successCount++;
    } else {
      failCount++;
    }
  }

  // Add builds
  console.log(`\n📝 Processing ${newBuilds.length} builds...`);
  for (const build of newBuilds) {
    const success = await addBuild(build);
    if (success) {
      successCount++;
    } else {
      failCount++;
    }
  }

  // Summary
  console.log(`\n📊 Summary:`);
  console.log(`✅ Successfully added: ${successCount} items`);
  console.log(`❌ Failed: ${failCount} items`);
  
  if (failCount === 0) {
    console.log(`🎉 All heroes and builds added successfully!`);
  } else {
    console.log(`⚠️  Some items failed to add. Check the errors above.`);
  }
}

// Run the script
addHeroesAndBuilds().catch(console.error);