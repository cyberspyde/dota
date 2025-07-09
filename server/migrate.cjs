const fs = require('fs/promises');
const path = require('path');
const { execSync } = require('child_process');
const esbuild = require('esbuild');
const pool = require('./database.js');

// Since we are in a .cjs file, we need to build the .ts files to .cjs
// before we can require them.
const compileTsFile = (filePath) => {
  const outDir = path.join(__dirname, '..', 'dist');
  const outFile = path.join(outDir, path.basename(filePath).replace('.ts', '.cjs'));

  // Create dist directory if it doesn't exist
  if (!require('fs').existsSync(outDir)) {
    require('fs').mkdirSync(outDir);
  }

  esbuild.buildSync({
    entryPoints: [filePath],
    outfile: outFile,
    bundle: true,
    platform: 'node',
    format: 'cjs',
  });
  return outFile;
};

const heroesPath = path.join(__dirname, '../src/data/heroes.ts');
const buildsPath = path.join(__dirname, '../src/data/builds.ts');

const compiledHeroesPath = compileTsFile(heroesPath);
const compiledBuildsPath = compileTsFile(buildsPath);

const { heroes } = require(compiledHeroesPath);
const { builds } = require(compiledBuildsPath);

// Self-compiling migration script
async function selfCompile() {
  const entryPoint = __filename;
  const outfile = path.join(__dirname, 'migrate.cjs');

  // Check if we are already running the compiled version
  if (process.argv[1] === outfile) {
    return;
  }

  console.log('🚀 Compiling migration script...');
  try {
    await esbuild.build({
      entryPoints: [entryPoint],
      outfile,
      bundle: true,
      platform: 'node',
      format: 'cjs',
      external: ['pg-native'], // Exclude native pg driver
    });

    // Execute the compiled script
    console.log('🚀 Executing compiled migration script...');
    execSync(`node ${outfile}`, { stdio: 'inherit' });
    process.exit(0);
  } catch (error) {
    console.error('💥 Compilation or execution failed:', error);
    process.exit(1);
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

async function migrate() {
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
if (import.meta.url.startsWith('file://') && process.argv[1] === fileURLToPath(import.meta.url)) {
  selfCompile().then(migrate);
}

module.exports = { createSchema, insertHeroes, insertBuilds };