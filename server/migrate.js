import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath, pathToFileURL } from 'url';
import pool from './database.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function compareFiles() {
  const heroesPath = path.join(__dirname, '../src/data/heroes.ts');
  const buildsPath = path.join(__dirname, '../src/data/builds.ts');

  const heroesContent = await fs.readFile(heroesPath, 'utf8');
  const buildsContent = await fs.readFile(buildsPath, 'utf8');

  console.log('--- HEROES.TS ---');
  console.log(heroesContent.substring(0, 500));
  console.log('--- BUILDS.TS ---');
  console.log(buildsContent.substring(0, 500));
}

// Import data from existing files - we'll need to read these as text and parse
async function loadHeroesData() {
  try {
    // Try to read the TypeScript file and extract the export
    const heroesPath = path.join(__dirname, '../src/data/heroes.ts');
    const heroesContent = await fs.readFile(heroesPath, 'utf8');
    
    // Extract the heroes array from the TypeScript file
    const exportMatch = heroesContent.match(/export\s+const\s+heroes\s*:\s*Hero\[\]\s*=\s*(\[[\s\S]*?\]);/);
    if (!exportMatch) {
      throw new Error('Could not find heroes export in heroes.ts');
    }
    
    // This is a simplified parser and may not handle all TypeScript features.
    let heroesArrayString = exportMatch[1];
    
    // In a real-world scenario, a proper parser would be better.
    // For now, we'll use a series of replacements to make it valid JavaScript.
    heroesArrayString = heroesArrayString
      .replace(/: Hero\[\]/g, '')
      .replace(/: string\[\]/g, '')
      .replace(/: string/g, '')
      .replace(/: 'Easy' \| 'Medium' \| 'Hard'/g, '')
      .replace(/: Mood\[\]/g, '');
    
    // Use a function constructor for safer evaluation than eval()
    const heroes = new Function(`return ${heroesArrayString}`)();
    return heroes;
  } catch (error) {
    console.error('Error loading heroes data:', error);
    return [];
  }
}

async function loadBuildsData() {
  try {
    const buildsPath = path.join(__dirname, '../src/data/builds.ts');
    
    try {
      await fs.access(buildsPath);
    } catch {
      console.log('⚠️  builds.ts file not found, skipping builds');
      return [];
    }
    
    const buildsContent = await fs.readFile(buildsPath, 'utf8');
    
    console.log('--- BUILD CONTENT ---');
    console.log(buildsContent.substring(0, 500)); // Log first 500 chars
    console.log('--- CHAR CODES ---');
    console.log(buildsContent.substring(0, 20).split('').map(c => c.charCodeAt(0)));
    
    const exportMatch = buildsContent.match(/export\s+const\s+builds\s*[:\w\s\[\]]*=\s*(\[[\s\S]*?\]);/);
    
    console.log('--- EXPORT MATCH ---');
    console.log(exportMatch);
    
    if (!exportMatch) {
      console.log('⚠️  Could not find builds export in builds.ts, skipping builds');
      return [];
    }
    
    let buildsArrayString = exportMatch[1];
    
    buildsArrayString = buildsArrayString
      .replace(/: Build\[\]/g, '')
      .replace(/: string\[\]/g, '')
      .replace(/: string/g, '')
      .replace(/: number/g, '')
      .replace(/: 'Early' \| 'Mid' \| 'Late'/g, '')
      .replace(/: 'Core' \| 'Situational' \| 'Luxury'/g, '')
      .replace(/: Mood/g, '');
      
    const builds = new Function(`return ${buildsArrayString}`)();
    return builds;
  } catch (error) {
    console.error('Error loading builds data:', error);
    return [];
  }
}

async function createSchema() {
  try {
    // Drop triggers first to ensure idempotency
    await pool.query('DROP TRIGGER IF EXISTS update_heroes_updated_at ON heroes;');
    await pool.query('DROP TRIGGER IF EXISTS update_builds_updated_at ON builds;');

    const schemaSQL = await fs.readFile(path.join(__dirname, '../supabase/migrations/20250708105422_morning_fog.sql'), 'utf8');
    await pool.query(schemaSQL);
    console.log('✅ Database schema created successfully');
  } catch (error) {
    console.error('❌ Error creating schema:', error);
    throw error;
  }
}

async function insertHeroes() {
  const heroes = await loadHeroesData();
  
  if (heroes.length === 0) {
    console.log('⚠️  No heroes data found, skipping heroes insertion');
    return;
  }
  
  const client = await pool.connect();
  
  try {
    await client.query('BEGIN');
    
    console.log('🚀 Checking for new heroes to insert...');
    let insertedCount = 0;

    for (const hero of heroes) {
      const res = await client.query(
        `INSERT INTO heroes (id, name, role, difficulty, description)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (id) DO NOTHING
         RETURNING id`,
        [hero.id, hero.name, hero.role, hero.difficulty, hero.description]
      );

      if (res.rows.length > 0) {
        insertedCount++;
        const heroId = res.rows[0].id;

        for (const mood of hero.moods) {
          await client.query(
            `INSERT INTO hero_moods (hero_id, mood) VALUES ($1, $2) ON CONFLICT DO NOTHING`,
            [heroId, mood]
          );
        }

        for (let i = 0; i < hero.strengths.length; i++) {
          await client.query(
            `INSERT INTO hero_strengths (hero_id, strength, order_index)
             VALUES ($1, $2, $3)`,
            [heroId, hero.strengths[i], i]
          );
        }

        for (let i = 0; i < hero.weaknesses.length; i++) {
          await client.query(
            `INSERT INTO hero_weaknesses (hero_id, weakness, order_index)
             VALUES ($1, $2, $3)`,
            [heroId, hero.weaknesses[i], i]
          );
        }
      }
    }
    
    await client.query('COMMIT');
    
    if (insertedCount > 0) {
      console.log(`✅ Inserted ${insertedCount} new heroes successfully`);
    } else {
      console.log('✅ No new heroes to insert.');
    }
  } catch (error) {
    await client.query('ROLLBACK');
    console.error('❌ Error inserting heroes:', error);
    throw error;
  } finally {
    client.release();
  }
}

async function insertBuilds() {
  const builds = await loadBuildsData();
  
  if (builds.length === 0) {
    console.log('⚠️  No builds data found, skipping builds insertion');
    return;
  }
  
  const client = await pool.connect();
  
  try {
    await client.query('BEGIN');
    
    console.log('🚀 Checking for new builds to insert...');
    let insertedCount = 0;
    
    for (const build of builds) {
      const buildResult = await client.query(
        `INSERT INTO builds (hero_id, mood, early_game, mid_game, late_game)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (hero_id, mood) DO NOTHING
         RETURNING id`,
        [build.heroId, build.mood, build.gameplan.early, build.gameplan.mid, build.gameplan.late]
      );
      
      if (buildResult.rows.length > 0) {
        insertedCount++;
        const buildId = buildResult.rows[0].id;
      
        for (let i = 0; i < build.items.length; i++) {
          const item = build.items[i];
          await client.query(
            `INSERT INTO items (build_id, name, cost, phase, priority, description, order_index)
             VALUES ($1, $2, $3, $4, $5, $6, $7)`,
            [buildId, item.name, item.cost, item.phase, item.priority, item.description, i]
          );
        }
        
        for (let i = 0; i < build.playstyle.dos.length; i++) {
          await client.query(
            `INSERT INTO playstyle_dos (build_id, do_item, order_index)
             VALUES ($1, $2, $3)`,
            [buildId, build.playstyle.dos[i], i]
          );
        }
        
        for (let i = 0; i < build.playstyle.donts.length; i++) {
          await client.query(
            `INSERT INTO playstyle_donts (build_id, dont_item, order_index)
             VALUES ($1, $2, $3)`,
            [buildId, build.playstyle.donts[i], i]
          );
        }
        
        for (let i = 0; i < build.playstyle.tips.length; i++) {
          await client.query(
            `INSERT INTO playstyle_tips (build_id, tip, order_index)
             VALUES ($1, $2, $3)`,
            [buildId, build.playstyle.tips[i], i]
          );
        }
      }
    }
    
    await client.query('COMMIT');

    if (insertedCount > 0) {
      console.log(`✅ Inserted ${insertedCount} new builds successfully`);
    } else {
      console.log('✅ No new builds to insert.');
    }
  } catch (error) {
    await client.query('ROLLBACK');
    console.error('❌ Error inserting builds:', error);
    throw error;
  } finally {
    client.release();
  }
}

export async function migrate() {
  try {
    console.log('🚀 Starting database migration...');
    
    await createSchema();
    await insertHeroes();
    await insertBuilds();
    
    console.log('🎉 Migration completed successfully!');
  } catch (error) {
    console.error('💥 Migration failed:', error);
  } finally {
    await pool.end();
    process.exit();
  }
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  migrate();
} 