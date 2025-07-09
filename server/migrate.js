import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath, pathToFileURL } from 'url';
import { PrismaClient } from '@prisma/client';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Create Prisma client instance
const prisma = new PrismaClient();

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
    console.log('🚀 Pushing Prisma schema to database...');
    await prisma.$executeRaw`DROP SCHEMA IF EXISTS public CASCADE`;
    await prisma.$executeRaw`CREATE SCHEMA public`;
    await prisma.$executeRaw`GRANT ALL ON SCHEMA public TO postgres`;
    await prisma.$executeRaw`GRANT ALL ON SCHEMA public TO public`;
    
    // Push the schema to the database
    const { execSync } = await import('child_process');
    execSync('npx prisma db push', { stdio: 'inherit' });
    
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
  
  try {
    console.log('🚀 Checking for new heroes to insert...');
    let insertedCount = 0;

    for (const hero of heroes) {
      // Check if hero already exists
      const existingHero = await prisma.hero.findUnique({
        where: { id: hero.id }
      });

      if (!existingHero) {
        // Insert hero
        await prisma.hero.create({
          data: {
            id: hero.id,
            name: hero.name,
            role: hero.role,
            difficulty: hero.difficulty,
            description: hero.description,
            moods: {
              create: hero.moods.map(mood => ({ mood }))
            },
            strengths: {
              create: hero.strengths.map((strength, index) => ({
                strength,
                orderIndex: index
              }))
            },
            weaknesses: {
              create: hero.weaknesses.map((weakness, index) => ({
                weakness,
                orderIndex: index
              }))
            }
          }
        });
        insertedCount++;
      }
    }
    
    if (insertedCount > 0) {
      console.log(`✅ Inserted ${insertedCount} new heroes successfully`);
    } else {
      console.log('✅ No new heroes to insert.');
    }
  } catch (error) {
    console.error('❌ Error inserting heroes:', error);
    throw error;
  }
}

async function insertBuilds() {
  const builds = await loadBuildsData();
  
  if (builds.length === 0) {
    console.log('⚠️  No builds data found, skipping builds insertion');
    return;
  }
  
  try {
    console.log('🚀 Checking for new builds to insert...');
    let insertedCount = 0;
    
    for (const build of builds) {
      // Check if build already exists
      const existingBuild = await prisma.build.findUnique({
        where: {
          heroId_mood: {
            heroId: build.heroId,
            mood: build.mood
          }
        }
      });
      
      if (!existingBuild) {
        // Insert build
        await prisma.build.create({
          data: {
            heroId: build.heroId,
            mood: build.mood,
            earlyGame: build.gameplan.early,
            midGame: build.gameplan.mid,
            lateGame: build.gameplan.late,
            items: {
              create: build.items.map((item, index) => ({
                name: item.name,
                cost: item.cost,
                phase: item.phase,
                priority: item.priority,
                description: item.description,
                orderIndex: index
              }))
            },
            playstyleDos: {
              create: build.playstyle.dos.map((doItem, index) => ({
                doItem,
                orderIndex: index
              }))
            },
            playstyleDonts: {
              create: build.playstyle.donts.map((dontItem, index) => ({
                dontItem,
                orderIndex: index
              }))
            },
            playstyleTips: {
              create: build.playstyle.tips.map((tip, index) => ({
                tip,
                orderIndex: index
              }))
            }
          }
        });
        insertedCount++;
      }
    }
    
    if (insertedCount > 0) {
      console.log(`✅ Inserted ${insertedCount} new builds successfully`);
    } else {
      console.log('✅ No new builds to insert.');
    }
  } catch (error) {
    console.error('❌ Error inserting builds:', error);
    throw error;
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
    await prisma.$disconnect();
    process.exit();
  }
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  migrate();
} 