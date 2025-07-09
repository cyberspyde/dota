import { Pool } from 'pg';

// Database connection
const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  ssl: process.env.NODE_ENV === 'production' ? { rejectUnauthorized: false } : false
});

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
      const heroesQuery = `
        SELECT 
          h.*,
          (SELECT array_agg(m.mood) FROM (SELECT DISTINCT hm.mood FROM hero_moods hm WHERE hm.hero_id = h.id) m) as moods,
          (SELECT array_agg(s.strength ORDER BY s.order_index) FROM hero_strengths s WHERE s.hero_id = h.id) as strengths,
          (SELECT array_agg(w.weakness ORDER BY w.order_index) FROM hero_weaknesses w WHERE w.hero_id = h.id) as weaknesses
        FROM heroes h
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(heroes)
      };
    }

    // Get hero by ID
    if (pathSegments[0] === 'heroes' && pathSegments.length === 2) {
      const heroId = pathSegments[1];
      
      const heroQuery = `
        SELECT 
          h.*,
          (SELECT array_agg(m.mood) FROM (SELECT DISTINCT hm.mood FROM hero_moods hm WHERE hm.hero_id = h.id) m) as moods,
          (SELECT array_agg(s.strength ORDER BY s.order_index) FROM hero_strengths s WHERE s.hero_id = h.id) as strengths,
          (SELECT array_agg(w.weakness ORDER BY w.order_index) FROM hero_weaknesses w WHERE w.hero_id = h.id) as weaknesses
        FROM heroes h
        WHERE h.id = $1
      `;
      
      const result = await pool.query(heroQuery, [heroId]);
      
      if (result.rows.length === 0) {
        return {
          statusCode: 404,
          headers: corsHeaders,
          body: JSON.stringify({ error: 'Hero not found' })
        };
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(heroData)
      };
    }

    // Get all builds
    if (pathSegments[0] === 'builds' && pathSegments.length === 1) {
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(builds)
      };
    }

    // Get builds for a specific hero
    if (pathSegments[0] === 'heroes' && pathSegments[2] === 'builds' && pathSegments.length === 3) {
      const heroId = pathSegments[1];
      
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
      
      return {
        statusCode: 200,
        headers: corsHeaders,
        body: JSON.stringify(builds)
      };
    }

    // Get specific build for hero and mood
    if (pathSegments[0] === 'heroes' && pathSegments[2] === 'builds' && pathSegments.length === 4) {
      const heroId = pathSegments[1];
      const mood = pathSegments[3];
      
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
        return {
          statusCode: 404,
          headers: corsHeaders,
          body: JSON.stringify({ error: 'Build not found' })
        };
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
  }
} 