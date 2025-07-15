import { Handler } from '@netlify/functions';
import { createClient } from '@supabase/supabase-js';
import { Hero, Build } from '../../src/types';
import { calculateBuildScore } from './scoring';

// Environment variables for Netlify Functions
const supabaseUrl = process.env.VITE_SUPABASE_URL;
const supabaseAnonKey = process.env.VITE_SUPABASE_ANON_KEY;
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;

if (!supabaseUrl || !supabaseAnonKey || !supabaseServiceKey) {
  console.error('Missing Supabase environment variables');
  console.error('VITE_SUPABASE_URL:', !!supabaseUrl);
  console.error('VITE_SUPABASE_ANON_KEY:', !!supabaseAnonKey);
  console.error('SUPABASE_SERVICE_ROLE_KEY:', !!supabaseServiceKey);
}

const supabase = createClient(supabaseUrl || '', supabaseAnonKey || '');
const supabaseAdmin = createClient(supabaseUrl || '', supabaseServiceKey || '');

// CORS headers
const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type, Authorization'
};

class ApiService {
  async getHeroes(limit: number = 100, offset: number = 0, mood?: string): Promise<{ heroes: Hero[], total: number }> {
    let query = supabase.from('heroes').select(`
      id,
      name,
      role,
      difficulty,
      description,
      hero_moods (mood),
      hero_strengths (strength, order_index),
      hero_weaknesses (weakness, order_index)
    `, { count: 'exact' });

    if (mood) {
      const { data: heroIds } = await supabase
        .from('builds')
        .select('hero_id')
        .eq('mood', mood);
      
      if (heroIds && heroIds.length > 0) {
        query = query.in('id', heroIds.map(h => h.hero_id));
      }
    }

    const { data, error, count } = await query
      .order('name')
      .range(offset, offset + limit - 1);

    if (error) {
      console.error('Error fetching heroes:', error);
      throw error;
    }

    const heroes = data.map(hero => ({
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
    
    return { heroes, total: count || 0 };
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
        id,
        hero_id,
        mood,
        score,
        early_game,
        mid_game,
        late_game,
        items:items(id, name, cost, phase, priority, description, order_index),
        playstyle_dos:playstyle_dos(do_item, order_index),
        playstyle_donts:playstyle_donts(dont_item, order_index),
        playstyle_tips:playstyle_tips(tip, order_index)
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
      score: build.score,
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
        id,
        hero_id,
        mood,
        score,
        early_game,
        mid_game,
        late_game,
        items:items(id, name, cost, phase, priority, description, order_index),
        playstyle_dos:playstyle_dos(do_item, order_index),
        playstyle_donts:playstyle_donts(dont_item, order_index),
        playstyle_tips:playstyle_tips(tip, order_index)
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
      score: build.score,
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
        id,
        hero_id,
        mood,
        score,
        early_game,
        mid_game,
        late_game,
        items:items(id, name, cost, phase, priority, description, order_index),
        playstyle_dos:playstyle_dos(do_item, order_index),
        playstyle_donts:playstyle_donts(dont_item, order_index),
        playstyle_tips:playstyle_tips(tip, order_index)
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
      score: build.score,
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

  async createHero(heroData: Hero): Promise<Hero> {
    // First, insert the main hero record
    const { data: hero, error: heroError } = await supabaseAdmin
      .from('heroes')
      .insert({
        id: heroData.id,
        name: heroData.name,
        role: heroData.role,
        difficulty: heroData.difficulty,
        description: heroData.description
      })
      .select()
      .single();

    if (heroError) throw heroError;

    // Insert moods
    if (heroData.moods.length > 0) {
      const { error: moodError } = await supabaseAdmin
        .from('hero_moods')
        .insert(heroData.moods.map(mood => ({
          hero_id: heroData.id,
          mood: mood
        })));
      if (moodError) throw moodError;
    }

    // Insert strengths
    if (heroData.strengths.length > 0) {
      const { error: strengthError } = await supabaseAdmin
        .from('hero_strengths')
        .insert(heroData.strengths.map((strength, index) => ({
          hero_id: heroData.id,
          strength: strength,
          order_index: index
        })));
      if (strengthError) throw strengthError;
    }

    // Insert weaknesses
    if (heroData.weaknesses.length > 0) {
      const { error: weaknessError } = await supabaseAdmin
        .from('hero_weaknesses')
        .insert(heroData.weaknesses.map((weakness, index) => ({
          hero_id: heroData.id,
          weakness: weakness,
          order_index: index
        })));
      if (weaknessError) throw weaknessError;
    }

    return heroData;
  }

  async updateHero(id: string, heroData: Partial<Hero>): Promise<Hero> {
    // Update main hero record
    const { data: hero, error: heroError } = await supabaseAdmin
      .from('heroes')
      .update({
        name: heroData.name,
        role: heroData.role,
        difficulty: heroData.difficulty,
        description: heroData.description
      })
      .eq('id', id)
      .select()
      .single();

    if (heroError) throw heroError;

    // Update moods if provided
    if (heroData.moods) {
      // Delete existing moods
      await supabaseAdmin.from('hero_moods').delete().eq('hero_id', id);
      
      // Insert new moods
      if (heroData.moods.length > 0) {
        const { error: moodError } = await supabaseAdmin
          .from('hero_moods')
          .insert(heroData.moods.map(mood => ({
            hero_id: id,
            mood: mood
          })));
        if (moodError) throw moodError;
      }
    }

    // Update strengths if provided
    if (heroData.strengths) {
      // Delete existing strengths
      await supabaseAdmin.from('hero_strengths').delete().eq('hero_id', id);
      
      // Insert new strengths
      if (heroData.strengths.length > 0) {
        const { error: strengthError } = await supabaseAdmin
          .from('hero_strengths')
          .insert(heroData.strengths.map((strength, index) => ({
            hero_id: id,
            strength: strength,
            order_index: index
          })));
        if (strengthError) throw strengthError;
      }
    }

    // Update weaknesses if provided
    if (heroData.weaknesses) {
      // Delete existing weaknesses
      await supabaseAdmin.from('hero_weaknesses').delete().eq('hero_id', id);
      
      // Insert new weaknesses
      if (heroData.weaknesses.length > 0) {
        const { error: weaknessError } = await supabaseAdmin
          .from('hero_weaknesses')
          .insert(heroData.weaknesses.map((weakness, index) => ({
            hero_id: id,
            weakness: weakness,
            order_index: index
          })));
        if (weaknessError) throw weaknessError;
      }
    }

    return await this.getHero(id);
  }

  async deleteHero(id: string): Promise<void> {
    const { error } = await supabaseAdmin.from('heroes').delete().eq('id', id);
    if (error) throw error;
  }

  async createBuild(buildData: Build): Promise<Build> {
    const hero = await this.getHero(buildData.heroId);
    const score = calculateBuildScore(hero, buildData);

    // First, insert the main build record
    const { data: build, error: buildError } = await supabaseAdmin
      .from('builds')
      .insert({
        hero_id: buildData.heroId,
        mood: buildData.mood,
        score: score,
        early_game: buildData.gameplan.early,
        mid_game: buildData.gameplan.mid,
        late_game: buildData.gameplan.late
      })
      .select()
      .single();

    if (buildError) throw buildError;

    // Insert build items
    if (buildData.items.length > 0) {
      const { error: itemsError } = await supabaseAdmin
        .from('items')
        .insert(
          buildData.items.map((item, index) => ({
            build_id: build.id,
            id: item.id,
            name: item.name,
            cost: item.cost,
            phase: item.phase,
            priority: item.priority,
            description: item.description,
            order_index: index
          }))
        );
      if (itemsError) throw itemsError;
    }

    // Insert playstyle dos
    if (buildData.playstyle.dos.length > 0) {
      const { error: dosError } = await supabaseAdmin
        .from('playstyle_dos')
        .insert(
          buildData.playstyle.dos.map((doItem, index) => ({
            build_id: build.id,
            do_item: doItem,
            order_index: index
          }))
        );
      if (dosError) throw dosError;
    }

    // Insert playstyle don'ts
    if (buildData.playstyle.donts.length > 0) {
      const { error: dontsError } = await supabaseAdmin
        .from('playstyle_donts')
        .insert(
          buildData.playstyle.donts.map((dontItem, index) => ({
            build_id: build.id,
            dont_item: dontItem,
            order_index: index
          }))
        );
      if (dontsError) throw dontsError;
    }

    // Insert playstyle tips
    if (buildData.playstyle.tips.length > 0) {
      const { error: tipsError } = await supabaseAdmin
        .from('playstyle_tips')
        .insert(
          buildData.playstyle.tips.map((tip, index) => ({
            build_id: build.id,
            tip: tip,
            order_index: index
          }))
        );
      if (tipsError) throw tipsError;
    }

    // Return the complete build
    return buildData;
  }

  async updateBuild(id: number, buildData: Partial<Build>): Promise<Build> {
    // Calculate new score if we have all the required data
    let score: number | undefined;
    if (buildData.heroId && buildData.items && buildData.playstyle && buildData.gameplan) {
      const hero = await this.getHero(buildData.heroId);
      score = calculateBuildScore(hero, buildData as Build);
    }

    // Update main build record
    const { data: build, error: buildError } = await supabaseAdmin
      .from('builds')
      .update({
        hero_id: buildData.heroId,
        mood: buildData.mood,
        score: score,
        early_game: buildData.gameplan?.early,
        mid_game: buildData.gameplan?.mid,
        late_game: buildData.gameplan?.late
      })
      .eq('id', id)
      .select()
      .single();

    if (buildError) throw buildError;

    // Update items if provided
    if (buildData.items) {
      // Delete existing items
      await supabaseAdmin.from('items').delete().eq('build_id', id);

      // Insert new items
      if (buildData.items.length > 0) {
        const { error: itemsError } = await supabaseAdmin
          .from('items')
          .insert(
            buildData.items.map((item, index) => ({
              build_id: id,
              id: item.id,
              name: item.name,
              cost: item.cost,
              phase: item.phase,
              priority: item.priority,
              description: item.description,
              order_index: index
            }))
          );
        if (itemsError) throw itemsError;
      }
    }

    // Update playstyle if provided
    if (buildData.playstyle) {
      // Update dos
      if (buildData.playstyle.dos) {
        await supabaseAdmin.from('playstyle_dos').delete().eq('build_id', id);
        if (buildData.playstyle.dos.length > 0) {
          const { error: dosError } = await supabaseAdmin
            .from('playstyle_dos')
            .insert(
              buildData.playstyle.dos.map((doItem, index) => ({
                build_id: id,
                do_item: doItem,
                order_index: index
              }))
            );
          if (dosError) throw dosError;
        }
      }

      // Update don'ts
      if (buildData.playstyle.donts) {
        await supabaseAdmin.from('playstyle_donts').delete().eq('build_id', id);
        if (buildData.playstyle.donts.length > 0) {
          const { error: dontsError } = await supabaseAdmin
            .from('playstyle_donts')
            .insert(
              buildData.playstyle.donts.map((dontItem, index) => ({
                build_id: id,
                dont_item: dontItem,
                order_index: index
              }))
            );
          if (dontsError) throw dontsError;
        }
      }

      // Update tips
      if (buildData.playstyle.tips) {
        await supabaseAdmin.from('playstyle_tips').delete().eq('build_id', id);
        if (buildData.playstyle.tips.length > 0) {
          const { error: tipsError } = await supabaseAdmin
            .from('playstyle_tips')
            .insert(
              buildData.playstyle.tips.map((tip, index) => ({
                build_id: id,
                tip: tip,
                order_index: index
              }))
            );
          if (tipsError) throw tipsError;
        }
      }
    }

    // Return the complete updated build
    return await this.getBuild(build.hero_id, build.mood);
  }

  async deleteBuild(id: number): Promise<void> {
    const { error } = await supabaseAdmin.from('builds').delete().eq('id', id);
    if (error) throw error;
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
      const url = new URL(event.rawUrl);
      const limit = parseInt(url.searchParams.get('limit') || '20', 10);
      const offset = parseInt(url.searchParams.get('offset') || '0', 10);
      const mood = url.searchParams.get('mood') || undefined;
      const heroesResult = await apiService.getHeroes(limit, offset, mood);
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(heroesResult)
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

    if (path === '/builds' && method === 'POST') {
      const buildData = JSON.parse(event.body || '{}');
      const newBuild = await apiService.createBuild(buildData);
      return {
        statusCode: 201,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(newBuild)
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
    
    if (heroMatch && method === 'PUT') {
      const heroId = heroMatch[1];
      const heroData = JSON.parse(event.body || '{}');
      const updatedHero = await apiService.updateHero(heroId, heroData);
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(updatedHero)
      };
    }

    if (heroMatch && method === 'DELETE') {
      const heroId = heroMatch[1];
      await apiService.deleteHero(heroId);
      return {
        statusCode: 204,
        headers: corsHeaders
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
    
    const buildIdMatch = path.match(/^\/builds\/(\d+)$/);
    if (buildIdMatch && method === 'PUT') {
      const buildId = parseInt(buildIdMatch[1], 10);
      const buildData = JSON.parse(event.body || '{}');
      const updatedBuild = await apiService.updateBuild(buildId, buildData);
      return {
        statusCode: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        body: JSON.stringify(updatedBuild)
      };
    }

    if (buildIdMatch && method === 'DELETE') {
      const buildId = parseInt(buildIdMatch[1], 10);
      await apiService.deleteBuild(buildId);
      return {
        statusCode: 204,
        headers: corsHeaders
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