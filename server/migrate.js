import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';
import pool from './database.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import data from existing files - we'll need to read these as text and parse
async function loadHeroesData() {
  try {
    // Try to read the TypeScript file and extract the export
    const heroesPath = path.join(__dirname, '../src/data/heroes.ts');
    const heroesContent = await fs.readFile(heroesPath, 'utf8');
    
    // Extract the heroes array from the TypeScript file
    // This is a simple regex extraction - in production you might want a more robust parser
    const exportMatch = heroesContent.match(/export const heroes: Hero\[\] = (\[[\s\S]*?\]);/);
    if (!exportMatch) {
      throw new Error('Could not find heroes export in heroes.ts');
    }
    
    // Convert TypeScript to JavaScript by removing type annotations
    let heroesArray = exportMatch[1];
    
    // Remove type annotations (simple approach)
    heroesArray = heroesArray.replace(/: Hero\[\]/g, '');
    heroesArray = heroesArray.replace(/: string\[\]/g, '');
    heroesArray = heroesArray.replace(/: string/g, '');
    heroesArray = heroesArray.replace(/: 'Easy' \| 'Medium' \| 'Hard'/g, '');
    heroesArray = heroesArray.replace(/: Mood\[\]/g, '');
    
    // Parse as JavaScript
    const heroes = eval(heroesArray);
    return heroes;
  } catch (error) {
    console.error('Error loading heroes data:', error);
    return [];
  }
}

async function loadBuildsData() {
  try {
    // Try to read the builds file if it exists
    const buildsPath = path.join(__dirname, '../src/data/builds.ts');
    
    try {
      await fs.access(buildsPath);
    } catch {
      console.log('⚠️  builds.ts file not found, skipping builds');
      return [];
    }
    
    const buildsContent = await fs.readFile(buildsPath, 'utf8');
    
    // Extract the builds array from the TypeScript file
    const exportMatch = buildsContent.match(/export const builds: Build\[\] = (\[[\s\S]*?\]);/);
    if (!exportMatch) {
      console.log('⚠️  Could not find builds export in builds.ts, skipping builds');
      return [];
    }
    
    // Convert TypeScript to JavaScript by removing type annotations
    let buildsArray = exportMatch[1];
    
    // Remove type annotations
    buildsArray = buildsArray.replace(/: Build\[\]/g, '');
    buildsArray = buildsArray.replace(/: string\[\]/g, '');
    buildsArray = buildsArray.replace(/: string/g, '');
    buildsArray = buildsArray.replace(/: number/g, '');
    buildsArray = buildsArray.replace(/: 'Early' \| 'Mid' \| 'Late'/g, '');
    buildsArray = buildsArray.replace(/: 'Core' \| 'Situational' \| 'Luxury'/g, '');
    buildsArray = buildsArray.replace(/: Mood/g, '');
    
    // Parse as JavaScript
    const builds = eval(buildsArray);
    return builds;
  } catch (error) {
    console.error('Error loading builds data:', error);
    return [];
  }
}

async function createSchema() {
  try {
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
    
    // Clear existing data
    await client.query('DELETE FROM heroes CASCADE');
    console.log('🧹 Cleared existing heroes data');
    
    for (const hero of heroes) {
      // Insert hero
      await client.query(
        `INSERT INTO heroes (id, name, role, difficulty, description)
         VALUES ($1, $2, $3, $4, $5)`,
        [hero.id, hero.name, hero.role, hero.difficulty, hero.description]
      );
      
      // Insert hero moods
      for (const mood of hero.moods) {
        await client.query(
          `INSERT INTO hero_moods (hero_id, mood) VALUES ($1, $2)`,
          [hero.id, mood]
        );
      }
      
      // Insert hero strengths
      for (let i = 0; i < hero.strengths.length; i++) {
        await client.query(
          `INSERT INTO hero_strengths (hero_id, strength, order_index)
           VALUES ($1, $2, $3)`,
          [hero.id, hero.strengths[i], i]
        );
      }
      
      // Insert hero weaknesses
      for (let i = 0; i < hero.weaknesses.length; i++) {
        await client.query(
          `INSERT INTO hero_weaknesses (hero_id, weakness, order_index)
           VALUES ($1, $2, $3)`,
          [hero.id, hero.weaknesses[i], i]
        );
      }
    }
    
    await client.query('COMMIT');
    console.log(`✅ Inserted ${heroes.length} heroes successfully`);
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
    
    // Clear existing data
    await client.query('DELETE FROM builds CASCADE');
    console.log('🧹 Cleared existing builds data');
    
    for (const build of builds) {
      // Insert build
      const buildResult = await client.query(
        `INSERT INTO builds (hero_id, mood, early_game, mid_game, late_game)
         VALUES ($1, $2, $3, $4, $5) RETURNING id`,
        [build.heroId, build.mood, build.gameplan.early, build.gameplan.mid, build.gameplan.late]
      );
      
      const buildId = buildResult.rows[0].id;
      
      // Insert items
      for (let i = 0; i < build.items.length; i++) {
        const item = build.items[i];
        await client.query(
          `INSERT INTO items (build_id, name, cost, phase, priority, description, order_index)
           VALUES ($1, $2, $3, $4, $5, $6, $7)`,
          [buildId, item.name, item.cost, item.phase, item.priority, item.description, i]
        );
      }
      
      // Insert playstyle dos
      for (let i = 0; i < build.playstyle.dos.length; i++) {
        await client.query(
          `INSERT INTO playstyle_dos (build_id, do_item, order_index)
           VALUES ($1, $2, $3)`,
          [buildId, build.playstyle.dos[i], i]
        );
      }
      
      // Insert playstyle donts
      for (let i = 0; i < build.playstyle.donts.length; i++) {
        await client.query(
          `INSERT INTO playstyle_donts (build_id, dont_item, order_index)
           VALUES ($1, $2, $3)`,
          [buildId, build.playstyle.donts[i], i]
        );
      }
      
      // Insert playstyle tips
      for (let i = 0; i < build.playstyle.tips.length; i++) {
        await client.query(
          `INSERT INTO playstyle_tips (build_id, tip, order_index)
           VALUES ($1, $2, $3)`,
          [buildId, build.playstyle.tips[i], i]
        );
      }
    }
    
    await client.query('COMMIT');
    console.log(`✅ Inserted ${builds.length} builds successfully`);
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
    process.exit(0);
  } catch (error) {
    console.error('💥 Migration failed:', error);
    process.exit(1);
  }
}

// Check if this module is being run directly
if (import.meta.url === `file://${process.argv[1]}`) {
  migrate();
}

export { createSchema, insertHeroes, insertBuilds };