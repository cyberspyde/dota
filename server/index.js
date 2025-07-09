import express from 'express';
import cors from 'cors';
import { PrismaClient } from '@prisma/client';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3001;

// Create Prisma client instance
const prisma = new PrismaClient();

// Middleware
app.use(cors());
app.use(express.json());

// API Routes

// Get all heroes
app.get('/api/heroes', async (req, res) => {
  try {
    const heroes = await prisma.hero.findMany({
      include: {
        moods: true,
        strengths: {
          orderBy: { orderIndex: 'asc' }
        },
        weaknesses: {
          orderBy: { orderIndex: 'asc' }
        }
      },
      orderBy: { name: 'asc' }
    });
    
    const formattedHeroes = heroes.map(hero => ({
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty,
      description: hero.description,
      moods: hero.moods.map(mood => mood.mood),
      strengths: hero.strengths.map(strength => strength.strength),
      weaknesses: hero.weaknesses.map(weakness => weakness.weakness)
    }));
    
    res.json(formattedHeroes);
  } catch (error) {
    console.error('Error fetching heroes:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get hero by ID
app.get('/api/heroes/:id', async (req, res) => {
  try {
    const { id } = req.params;
    
    const hero = await prisma.hero.findUnique({
      where: { id },
      include: {
        moods: true,
        strengths: {
          orderBy: { orderIndex: 'asc' }
        },
        weaknesses: {
          orderBy: { orderIndex: 'asc' }
        }
      }
    });
    
    if (!hero) {
      return res.status(404).json({ error: 'Hero not found' });
    }
    
    const heroData = {
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty,
      description: hero.description,
      moods: hero.moods.map(mood => mood.mood),
      strengths: hero.strengths.map(strength => strength.strength),
      weaknesses: hero.weaknesses.map(weakness => weakness.weakness)
    };
    
    res.json(heroData);
  } catch (error) {
    console.error('Error fetching hero:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get all builds
app.get('/api/builds', async (req, res) => {
  try {
    const builds = await prisma.build.findMany({
      include: {
        items: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleDos: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleDonts: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleTips: {
          orderBy: { orderIndex: 'asc' }
        }
      },
      orderBy: [
        { heroId: 'asc' },
        { mood: 'asc' }
      ]
    });
    
    const formattedBuilds = builds.map(build => ({
      heroId: build.heroId,
      mood: build.mood,
      items: build.items.map(item => ({
        id: item.id,
        name: item.name,
        cost: item.cost,
        phase: item.phase,
        priority: item.priority,
        description: item.description
      })),
      playstyle: {
        dos: build.playstyleDos.map(doItem => doItem.doItem),
        donts: build.playstyleDonts.map(dontItem => dontItem.dontItem),
        tips: build.playstyleTips.map(tip => tip.tip)
      },
      gameplan: {
        early: build.earlyGame,
        mid: build.midGame,
        late: build.lateGame
      }
    }));
    
    res.json(formattedBuilds);
  } catch (error) {
    console.error('Error fetching builds:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get builds for a specific hero
app.get('/api/heroes/:heroId/builds', async (req, res) => {
  try {
    const { heroId } = req.params;
    
    const builds = await prisma.build.findMany({
      where: { heroId },
      include: {
        items: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleDos: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleDonts: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleTips: {
          orderBy: { orderIndex: 'asc' }
        }
      },
      orderBy: { mood: 'asc' }
    });
    
    const formattedBuilds = builds.map(build => ({
      heroId: build.heroId,
      mood: build.mood,
      items: build.items.map(item => ({
        id: item.id,
        name: item.name,
        cost: item.cost,
        phase: item.phase,
        priority: item.priority,
        description: item.description
      })),
      playstyle: {
        dos: build.playstyleDos.map(doItem => doItem.doItem),
        donts: build.playstyleDonts.map(dontItem => dontItem.dontItem),
        tips: build.playstyleTips.map(tip => tip.tip)
      },
      gameplan: {
        early: build.earlyGame,
        mid: build.midGame,
        late: build.lateGame
      }
    }));
    
    res.json(formattedBuilds);
  } catch (error) {
    console.error('Error fetching hero builds:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get specific build for hero and mood
app.get('/api/heroes/:heroId/builds/:mood', async (req, res) => {
  try {
    const { heroId, mood } = req.params;
    
    const build = await prisma.build.findUnique({
      where: {
        heroId_mood: {
          heroId,
          mood
        }
      },
      include: {
        items: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleDos: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleDonts: {
          orderBy: { orderIndex: 'asc' }
        },
        playstyleTips: {
          orderBy: { orderIndex: 'asc' }
        }
      }
    });
    
    if (!build) {
      return res.status(404).json({ error: 'Build not found' });
    }
    
    const buildData = {
      heroId: build.heroId,
      mood: build.mood,
      items: build.items.map(item => ({
        id: item.id,
        name: item.name,
        cost: item.cost,
        phase: item.phase,
        priority: item.priority,
        description: item.description
      })),
      playstyle: {
        dos: build.playstyleDos.map(doItem => doItem.doItem),
        donts: build.playstyleDonts.map(dontItem => dontItem.dontItem),
        tips: build.playstyleTips.map(tip => tip.tip)
      },
      gameplan: {
        early: build.earlyGame,
        mid: build.midGame,
        late: build.lateGame
      }
    };
    
    res.json(buildData);
  } catch (error) {
    console.error('Error fetching build:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Health check endpoint
app.get('/api/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

// Start server
app.listen(PORT, () => {
  console.log(`🚀 Server running on http://localhost:${PORT}`);
  console.log(`📊 API available at http://localhost:${PORT}/api`);
});

// Graceful shutdown
process.on('SIGINT', async () => {
  console.log('\n🛑 Shutting down server...');
  await prisma.$disconnect();
  process.exit(0);
});