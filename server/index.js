import express from 'express';
import cors from 'cors';
import pool from './database.js';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// API Routes

// Get all heroes
app.get('/api/heroes', async (req, res) => {
  try {
    const heroesQuery = `
      SELECT 
        h.*,
        array_agg(DISTINCT hm.mood) as moods,
        array_agg(DISTINCT hs.strength ORDER BY hs.order_index) as strengths,
        array_agg(DISTINCT hw.weakness ORDER BY hw.order_index) as weaknesses
      FROM heroes h
      LEFT JOIN hero_moods hm ON h.id = hm.hero_id
      LEFT JOIN hero_strengths hs ON h.id = hs.hero_id
      LEFT JOIN hero_weaknesses hw ON h.id = hw.hero_id
      GROUP BY h.id, h.name, h.role, h.difficulty, h.description, h.created_at, h.updated_at
      ORDER BY h.name
    `;
    
    const result = await pool.query(heroesQuery);
    
    const heroes = result.rows.map(hero => ({
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty,
      description: hero.description,
      moods: hero.moods.filter(mood => mood !== null),
      strengths: hero.strengths.filter(strength => strength !== null),
      weaknesses: hero.weaknesses.filter(weakness => weakness !== null)
    }));
    
    res.json(heroes);
  } catch (error) {
    console.error('Error fetching heroes:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get hero by ID
app.get('/api/heroes/:id', async (req, res) => {
  try {
    const { id } = req.params;
    
    const heroQuery = `
      SELECT 
        h.*,
        array_agg(DISTINCT hm.mood) as moods,
        array_agg(DISTINCT hs.strength ORDER BY hs.order_index) as strengths,
        array_agg(DISTINCT hw.weakness ORDER BY hw.order_index) as weaknesses
      FROM heroes h
      LEFT JOIN hero_moods hm ON h.id = hm.hero_id
      LEFT JOIN hero_strengths hs ON h.id = hs.hero_id
      LEFT JOIN hero_weaknesses hw ON h.id = hw.hero_id
      WHERE h.id = $1
      GROUP BY h.id, h.name, h.role, h.difficulty, h.description, h.created_at, h.updated_at
    `;
    
    const result = await pool.query(heroQuery, [id]);
    
    if (result.rows.length === 0) {
      return res.status(404).json({ error: 'Hero not found' });
    }
    
    const hero = result.rows[0];
    const heroData = {
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty,
      description: hero.description,
      moods: hero.moods.filter(mood => mood !== null),
      strengths: hero.strengths.filter(strength => strength !== null),
      weaknesses: hero.weaknesses.filter(weakness => weakness !== null)
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
    const buildsQuery = `
      SELECT 
        b.*,
        json_agg(
          json_build_object(
            'id', i.id,
            'name', i.name,
            'cost', i.cost,
            'phase', i.phase,
            'priority', i.priority,
            'description', i.description
          ) ORDER BY i.order_index
        ) as items,
        (
          SELECT array_agg(pd.do_item ORDER BY pd.order_index)
          FROM playstyle_dos pd WHERE pd.build_id = b.id
        ) as dos,
        (
          SELECT array_agg(pdn.dont_item ORDER BY pdn.order_index)
          FROM playstyle_donts pdn WHERE pdn.build_id = b.id
        ) as donts,
        (
          SELECT array_agg(pt.tip ORDER BY pt.order_index)
          FROM playstyle_tips pt WHERE pt.build_id = b.id
        ) as tips
      FROM builds b
      LEFT JOIN items i ON b.id = i.build_id
      GROUP BY b.id, b.hero_id, b.mood, b.early_game, b.mid_game, b.late_game, b.created_at, b.updated_at
      ORDER BY b.hero_id, b.mood
    `;
    
    const result = await pool.query(buildsQuery);
    
    const builds = result.rows.map(build => ({
      heroId: build.hero_id,
      mood: build.mood,
      items: build.items.filter(item => item.id !== null),
      playstyle: {
        dos: build.dos || [],
        donts: build.donts || [],
        tips: build.tips || []
      },
      gameplan: {
        early: build.early_game,
        mid: build.mid_game,
        late: build.late_game
      }
    }));
    
    res.json(builds);
  } catch (error) {
    console.error('Error fetching builds:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get builds for a specific hero
app.get('/api/heroes/:heroId/builds', async (req, res) => {
  try {
    const { heroId } = req.params;
    
    const buildsQuery = `
      SELECT 
        b.*,
        json_agg(
          json_build_object(
            'id', i.id,
            'name', i.name,
            'cost', i.cost,
            'phase', i.phase,
            'priority', i.priority,
            'description', i.description
          ) ORDER BY i.order_index
        ) as items,
        (
          SELECT array_agg(pd.do_item ORDER BY pd.order_index)
          FROM playstyle_dos pd WHERE pd.build_id = b.id
        ) as dos,
        (
          SELECT array_agg(pdn.dont_item ORDER BY pdn.order_index)
          FROM playstyle_donts pdn WHERE pdn.build_id = b.id
        ) as donts,
        (
          SELECT array_agg(pt.tip ORDER BY pt.order_index)
          FROM playstyle_tips pt WHERE pt.build_id = b.id
        ) as tips
      FROM builds b
      LEFT JOIN items i ON b.id = i.build_id
      WHERE b.hero_id = $1
      GROUP BY b.id, b.hero_id, b.mood, b.early_game, b.mid_game, b.late_game, b.created_at, b.updated_at
      ORDER BY b.mood
    `;
    
    const result = await pool.query(buildsQuery, [heroId]);
    
    const builds = result.rows.map(build => ({
      heroId: build.hero_id,
      mood: build.mood,
      items: build.items.filter(item => item.id !== null),
      playstyle: {
        dos: build.dos || [],
        donts: build.donts || [],
        tips: build.tips || []
      },
      gameplan: {
        early: build.early_game,
        mid: build.mid_game,
        late: build.late_game
      }
    }));
    
    res.json(builds);
  } catch (error) {
    console.error('Error fetching hero builds:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Get specific build for hero and mood
app.get('/api/heroes/:heroId/builds/:mood', async (req, res) => {
  try {
    const { heroId, mood } = req.params;
    
    const buildQuery = `
      SELECT 
        b.*,
        json_agg(
          json_build_object(
            'id', i.id,
            'name', i.name,
            'cost', i.cost,
            'phase', i.phase,
            'priority', i.priority,
            'description', i.description
          ) ORDER BY i.order_index
        ) as items,
        (
          SELECT array_agg(pd.do_item ORDER BY pd.order_index)
          FROM playstyle_dos pd WHERE pd.build_id = b.id
        ) as dos,
        (
          SELECT array_agg(pdn.dont_item ORDER BY pdn.order_index)
          FROM playstyle_donts pdn WHERE pdn.build_id = b.id
        ) as donts,
        (
          SELECT array_agg(pt.tip ORDER BY pt.order_index)
          FROM playstyle_tips pt WHERE pt.build_id = b.id
        ) as tips
      FROM builds b
      LEFT JOIN items i ON b.id = i.build_id
      WHERE b.hero_id = $1 AND b.mood = $2
      GROUP BY b.id, b.hero_id, b.mood, b.early_game, b.mid_game, b.late_game, b.created_at, b.updated_at
    `;
    
    const result = await pool.query(buildQuery, [heroId, mood]);
    
    if (result.rows.length === 0) {
      return res.status(404).json({ error: 'Build not found' });
    }
    
    const build = result.rows[0];
    const buildData = {
      heroId: build.hero_id,
      mood: build.mood,
      items: build.items.filter(item => item.id !== null),
      playstyle: {
        dos: build.dos || [],
        donts: build.donts || [],
        tips: build.tips || []
      },
      gameplan: {
        early: build.early_game,
        mid: build.mid_game,
        late: build.late_game
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
  await pool.end();
  process.exit(0);
});