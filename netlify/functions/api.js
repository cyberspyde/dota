import { PrismaClient } from '@prisma/client';

// Database connection
const prisma = new PrismaClient();

// Helper function to handle CORS
const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Headers': 'Content-Type',
  'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS'
};

export async function handler(event, context) {
  // Handle CORS preflight requests
  if (event.httpMethod === 'OPTIONS') {
    return {
      statusCode: 200,
      headers: corsHeaders,
      body: ''
    };
  }

  const { path, httpMethod } = event;
  const pathSegments = path.replace('/.netlify/functions/api', '').split('/').filter(Boolean);

  try {
    // Health check
    if (pathSegments[0] === 'health') {
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify({ 
          status: 'ok', 
          timestamp: new Date().toISOString() 
        })
      };
    }

    // Get all heroes
    if (pathSegments[0] === 'heroes' && pathSegments.length === 1) {
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(formattedHeroes)
      };
    }

    // Get hero by ID
    if (pathSegments[0] === 'heroes' && pathSegments.length === 2) {
      const heroId = pathSegments[1];
      
      const hero = await prisma.hero.findUnique({
        where: { id: heroId },
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
        return {
          statusCode: 404,
          headers: corsHeaders,
          body: JSON.stringify({ error: 'Hero not found' })
        };
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(heroData)
      };
    }

    // Get all builds
    if (pathSegments[0] === 'builds' && pathSegments.length === 1) {
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(formattedBuilds)
      };
    }

    // Get builds for a specific hero
    if (pathSegments[0] === 'heroes' && pathSegments[2] === 'builds' && pathSegments.length === 3) {
      const heroId = pathSegments[1];
      
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(formattedBuilds)
      };
    }

    // Get specific build for hero and mood
    if (pathSegments[0] === 'heroes' && pathSegments[2] === 'builds' && pathSegments.length === 4) {
      const heroId = pathSegments[1];
      const mood = pathSegments[3];
      
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
        return {
          statusCode: 404,
          headers: corsHeaders,
          body: JSON.stringify({ error: 'Build not found' })
        };
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(buildData)
      };
    }

    // 404 for unknown routes
    return {
      statusCode: 404,
      headers: corsHeaders,
      body: JSON.stringify({ error: 'Route not found' })
    };

  } catch (error) {
    console.error('API Error:', error);
    return {
      statusCode: 500,
      headers: corsHeaders,
      body: JSON.stringify({ error: 'Internal server error' })
    };
  } finally {
    await prisma.$disconnect();
  }
} 