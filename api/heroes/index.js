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
    
    res.status(200).json(formattedHeroes);
  } catch (error) {
    console.error('Error fetching heroes:', error);
    res.status(500).json({ error: 'Internal server error' });
  } finally {
    await prisma.$disconnect();
  }
} 