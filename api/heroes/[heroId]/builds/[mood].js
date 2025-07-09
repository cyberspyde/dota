import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export default async function handler(req, res) {
  // Set CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  // Handle preflight requests
  if (req.method === 'OPTIONS') {
    res.status(200).end();
    return;
  }

  if (req.method !== 'GET') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  const { heroId, mood } = req.query;

  try {
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
    
    res.status(200).json(buildData);
  } catch (error) {
    console.error('Error fetching build:', error);
    res.status(500).json({ error: 'Internal server error' });
  } finally {
    await prisma.$disconnect();
  }
} 