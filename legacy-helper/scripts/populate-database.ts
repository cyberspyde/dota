import { createClient } from '@supabase/supabase-js';
import { heroes } from '../src/data/heroes';
import { builds } from '../src/data/builds';

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

async function populateHeroes() {
  console.log('🚀 Populating heroes...');
  
  for (const hero of heroes) {
    // Insert hero
    const { data: heroData, error: heroError } = await supabase
      .from('heroes')
      .upsert({
        id: hero.id,
        name: hero.name,
        role: hero.role,
        difficulty: hero.difficulty,
        description: hero.description
      }, { onConflict: 'id' });

    if (heroError) {
      console.error(`Error inserting hero ${hero.name}:`, heroError);
      continue;
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
        console.error(`Error inserting mood for ${hero.name}:`, moodError);
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
        console.error(`Error inserting strength for ${hero.name}:`, strengthError);
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
        console.error(`Error inserting weakness for ${hero.name}:`, weaknessError);
      }
    }
  }

  console.log(`✅ Inserted ${heroes.length} heroes`);
}

async function populateBuilds() {
  console.log('🚀 Populating builds...');
  
  for (const build of builds) {
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
      console.error(`Error inserting build for ${build.heroId}:`, buildError);
      continue;
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
        console.error(`Error inserting item for ${build.heroId}:`, itemError);
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
        console.error(`Error inserting do item for ${build.heroId}:`, doError);
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
        console.error(`Error inserting dont item for ${build.heroId}:`, dontError);
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
        console.error(`Error inserting tip for ${build.heroId}:`, tipError);
      }
    }
  }

  console.log(`✅ Inserted ${builds.length} builds`);
}

async function main() {
  console.log('🚀 Starting database population...');
  
  try {
    await populateHeroes();
    await populateBuilds();
    console.log('🎉 Database population completed successfully!');
  } catch (error) {
    console.error('💥 Database population failed:', error);
  }
}

main();