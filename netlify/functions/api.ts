import { Handler } from '@netlify/functions';
import { createClient } from '@supabase/supabase-js';
import { Hero, Build } from '../../src/types';

// Environment variables for Netlify Functions
const supabaseUrl = process.env.VITE_SUPABASE_URL;
const supabaseAnonKey = process.env.VITE_SUPABASE_ANON_KEY;

if (!supabaseUrl || !supabaseAnonKey) {
  console.error('Missing Supabase environment variables');
  console.error('VITE_SUPABASE_URL:', !!supabaseUrl);
  console.error('VITE_SUPABASE_ANON_KEY:', !!supabaseAnonKey);
}

const supabase = createClient(supabaseUrl || '', supabaseAnonKey || '');

// CORS headers
const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type, Authorization'
};

class ApiService {
  async getHeroes(): Promise<Hero[]> {
    const { data: heroes, error } = await supabase
      .from('heroes')
      .select(`
        id,
        name,
        role,
        difficulty,
        description,
        hero_moods (mood),
        hero_strengths (strength, order_index),
        hero_weaknesses (weakness, order_index)
      `)
      .order('name');

    if (error) {
      console.error('Error fetching heroes:', error);
      throw error;
    }

    return heroes.map(hero => ({
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty as 'Easy' | 'Medium' | 'Hard',
      description: hero.description || '',
      moods: hero.hero_moods?.map(m => m.mood) || [],
      strengths: hero.hero_strengths
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(s => s.strength) || [],
      weaknesses: hero.hero_weaknesses
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(w => w.weakness) || []
    }));
  }

  async getHero(id: string): Promise<Hero> {
    const { data: hero, error } = await supabase
      .from('heroes')
      .select(`
        id,
        name,
        role,
        difficulty,
        description,
        hero_moods (mood),
        hero_strengths (strength, order_index),
        hero_weaknesses (weakness, order_index)
      `)
      .eq('id', id)
      .single();

    if (error) {
      console.error('Error fetching hero:', error);
      throw error;
    }

    return {
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty as 'Easy' | 'Medium' | 'Hard',
      description: hero.description || '',
      moods: hero.hero_moods?.map(m => m.mood) || [],
      strengths: hero.hero_strengths
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(s => s.strength) || [],
      weaknesses: hero.hero_weaknesses
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(w => w.weakness) || []
    };
  }

  async getBuilds(): Promise<Build[]> {
    const { data: builds, error } = await supabase
      .from('builds')
      .select(`
        hero_id,
        mood,
        early_game,
        mid_game,
        late_game,
        items (id, name, cost, phase, priority, description, order_index),
        playstyle_dos (do_item, order_index),
        playstyle_donts (dont_item, order_index),
        playstyle_tips (tip, order_index)
      `)
      .order('hero_id')
      .order('mood');

    if (error) {
      console.error('Error fetching builds:', error);
      throw error;
    }

    return builds.map(build => ({
      heroId: build.hero_id,
      mood: build.mood,
      items: build.items
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(item => ({
          id: item.id,
          name: item.name,
          cost: item.cost,
          phase: item.phase as 'Early' | 'Mid' | 'Late',
          priority: item.priority as 'Core' | 'Situational' | 'Luxury',
          description: item.description || ''
        })) || [],
      playstyle: {
        dos: build.playstyle_dos
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.do_item) || [],
        donts: build.playstyle_donts
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.dont_item) || [],
        tips: build.playstyle_tips
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.tip) || []
      },
      gameplan: {
        early: build.early_game || '',
        mid: build.mid_game || '',
        late: build.late_game || ''
      }
    }));
  }

  async getHeroBuilds(heroId: string): Promise<Build[]> {
    const { data: builds, error } = await supabase
      .from('builds')
      .select(`
        hero_id,
        mood,
        early_game,
        mid_game,
        late_game,
        items (id, name, cost, phase, priority, description, order_index),
        playstyle_dos (do_item, order_index),
        playstyle_donts (dont_item, order_index),
        playstyle_tips (tip, order_index)
      `)
      .eq('hero_id', heroId)
      .order('mood');

    if (error) {
      console.error('Error fetching hero builds:', error);
      throw error;
    }

    return builds.map(build => ({
      heroId: build.hero_id,
      mood: build.mood,
      items: build.items
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(item => ({
          id: item.id,
          name: item.name,
          cost: item.cost,
          phase: item.phase as 'Early' | 'Mid' | 'Late',
          priority: item.priority as 'Core' | 'Situational' | 'Luxury',
          description: item.description || ''
        })) || [],
      playstyle: {
        dos: build.playstyle_dos
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.do_item) || [],
        donts: build.playstyle_donts
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.dont_item) || [],
        tips: build.playstyle_tips
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.tip) || []
      },
      gameplan: {
        early: build.early_game || '',
        mid: build.mid_game || '',
        late: build.late_game || ''
      }
    }));
  }

  async getBuild(heroId: string, mood: string): Promise<Build> {
    const { data: build, error } = await supabase
      .from('builds')
      .select(`
        hero_id,
        mood,
        early_game,
        mid_game,
        late_game,
        items (id, name, cost, phase, priority, description, order_index),
        playstyle_dos (do_item, order_index),
        playstyle_donts (dont_item, order_index),
        playstyle_tips (tip, order_index)
      `)
      .eq('hero_id', heroId)
      .eq('mood', mood)
      .single();

    if (error) {
      console.error('Error fetching build:', error);
      throw error;
    }

    return {
      heroId: build.hero_id,
      mood: build.mood,
      items: build.items
        ?.sort((a, b) => a.order_index - b.order_index)
        .map(item => ({
          id: item.id,
          name: item.name,
          cost: item.cost,
          phase: item.phase as 'Early' | 'Mid' | 'Late',
          priority: item.priority as 'Core' | 'Situational' | 'Luxury',
          description: item.description || ''
        })) || [],
      playstyle: {
        dos: build.playstyle_dos
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.do_item) || [],
        donts: build.playstyle_donts
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.dont_item) || [],
        tips: build.playstyle_tips
          ?.sort((a, b) => a.order_index - b.order_index)
          .map(item => item.tip) || []
      },
      gameplan: {
        early: build.early_game || '',
        mid: build.mid_game || '',
        late: build.late_game || ''
      }
    };
  }

  async checkHealth(): Promise<{ status: string; timestamp: string }> {
    const { data, error } = await supabase
      .from('heroes')
      .select('count')
      .limit(1);

    if (error) {
      throw error;
    }

    return {
      status: 'ok',
      timestamp: new Date().toISOString()
    };
  }
}

const apiService = new ApiService();

export const handler: Handler = async (event, context) => {
  // Handle preflight requests
  if (event.httpMethod === 'OPTIONS') {
    return {
      statusCode: 200,
      headers: corsHeaders,
      body: ''
    };
  }

  try {
    const basePath = '/.netlify/functions/api';
    let path = event.path;

    if (path.startsWith(basePath)) {
      path = path.substring(basePath.length);
    } else if (path.startsWith('/api')) {
      path = path.substring('/api'.length);
    }
    
    const method = event.httpMethod;

    console.log(`API Request: ${method} ${path}`);

    // Route handling
    if (path === '/health' && method === 'GET') {
      const health = await apiService.checkHealth();
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(health)
      };
    }

    if (path === '/heroes' && method === 'GET') {
      const heroes = await apiService.getHeroes();
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(heroes)
      };
    }

    if (path === '/builds' && method === 'GET') {
      const builds = await apiService.getBuilds();
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(builds)
      };
    }

    // Dynamic routes
    const heroMatch = path.match(/^\/heroes\/([^\/]+)$/);
    if (heroMatch && method === 'GET') {
      const heroId = heroMatch[1];
      const hero = await apiService.getHero(heroId);
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(hero)
      };
    }

    const heroBuildsMatch = path.match(/^\/heroes\/([^\/]+)\/builds$/);
    if (heroBuildsMatch && method === 'GET') {
      const heroId = heroBuildsMatch[1];
      const builds = await apiService.getHeroBuilds(heroId);
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(builds)
      };
    }

    const buildMatch = path.match(/^\/heroes\/([^\/]+)\/builds\/([^\/]+)$/);
    if (buildMatch && method === 'GET') {
      const heroId = buildMatch[1];
      const mood = buildMatch[2];
      const build = await apiService.getBuild(heroId, mood);
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(build)
      };
    }

    return {
      statusCode: 404,
      headers: corsHeaders,
      body: JSON.stringify({ error: 'Not Found' })
    };

  } catch (error) {
    console.error('API Error:', error);
    return {
      statusCode: 500,
      headers: corsHeaders,
      body: JSON.stringify({ 
        error: 'Internal Server Error',
        message: error instanceof Error ? error.message : 'Unknown error'
      })
    };
  }
};