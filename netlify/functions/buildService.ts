// Wrapper for build operations that handles scoring
import { createClient } from '@supabase/supabase-js';
import { calculateBuildScore } from './scoring';
import { Hero, Build } from '../../src/types';

const supabaseUrl = process.env.VITE_SUPABASE_URL || '';
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY || '';
const supabase = createClient(supabaseUrl, supabaseServiceKey);

export class BuildService {
  private static async getHero(heroId: string): Promise<Hero> {
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
      .eq('id', heroId)
      .single();

    if (error) throw error;
    
    return {
      id: hero.id,
      name: hero.name,
      role: hero.role,
      difficulty: hero.difficulty,
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

  static async createBuild(build: Build): Promise<Build> {
    const hero = await this.getHero(build.heroId);
    const score = calculateBuildScore(hero, build);

    // Insert build with calculated score
    const { data, error } = await supabase
      .from('builds')
      .insert({
        hero_id: build.heroId,
        mood: build.mood,
        score: score,
        early_game: build.gameplan.early,
        mid_game: build.gameplan.mid,
        late_game: build.gameplan.late
      })
      .select()
      .single();

    if (error) throw error;

    // Insert items
    for (let i = 0; i < build.items.length; i++) {
      const item = build.items[i];
      const { error: itemError } = await supabase
        .from('build_items')
        .insert({
          build_id: data.id,
          id: item.id,
          name: item.name,
          cost: item.cost,
          phase: item.phase,
          priority: item.priority,
          description: item.description,
          order_index: i
        });

      if (itemError) throw itemError;
    }

    // Insert playstyle elements
    for (let i = 0; i < build.playstyle.dos.length; i++) {
      const { error: doError } = await supabase
        .from('build_playstyle_dos')
        .insert({
          build_id: data.id,
          do_item: build.playstyle.dos[i],
          order_index: i
        });

      if (doError) throw doError;
    }

    for (let i = 0; i < build.playstyle.donts.length; i++) {
      const { error: dontError } = await supabase
        .from('build_playstyle_donts')
        .insert({
          build_id: data.id,
          dont_item: build.playstyle.donts[i],
          order_index: i
        });

      if (dontError) throw dontError;
    }

    for (let i = 0; i < build.playstyle.tips.length; i++) {
      const { error: tipError } = await supabase
        .from('build_playstyle_tips')
        .insert({
          build_id: data.id,
          tip: build.playstyle.tips[i],
          order_index: i
        });

      if (tipError) throw tipError;
    }

    return build;
  }

  static async updateBuild(buildId: number, build: Partial<Build>): Promise<Build> {
    if (build.heroId) {
      const hero = await this.getHero(build.heroId);
      const score = calculateBuildScore(hero, build as Build);
      build.score = score;
    }

    const { data, error } = await supabase
      .from('builds')
      .update({
        hero_id: build.heroId,
        mood: build.mood,
        score: build.score,
        early_game: build.gameplan?.early,
        mid_game: build.gameplan?.mid,
        late_game: build.gameplan?.late
      })
      .eq('id', buildId)
      .select()
      .single();

    if (error) throw error;

    // Update items if provided
    if (build.items) {
      // Delete existing items
      await supabase.from('build_items').delete().eq('build_id', buildId);

      // Insert new items
      for (let i = 0; i < build.items.length; i++) {
        const item = build.items[i];
        const { error: itemError } = await supabase
          .from('build_items')
          .insert({
            build_id: buildId,
            id: item.id,
            name: item.name,
            cost: item.cost,
            phase: item.phase,
            priority: item.priority,
            description: item.description,
            order_index: i
          });

        if (itemError) throw itemError;
      }
    }

    // Update playstyle if provided
    if (build.playstyle) {
      if (build.playstyle.dos) {
        await supabase.from('build_playstyle_dos').delete().eq('build_id', buildId);
        for (let i = 0; i < build.playstyle.dos.length; i++) {
          const { error: doError } = await supabase
            .from('build_playstyle_dos')
            .insert({
              build_id: buildId,
              do_item: build.playstyle.dos[i],
              order_index: i
            });

          if (doError) throw doError;
        }
      }

      if (build.playstyle.donts) {
        await supabase.from('build_playstyle_donts').delete().eq('build_id', buildId);
        for (let i = 0; i < build.playstyle.donts.length; i++) {
          const { error: dontError } = await supabase
            .from('build_playstyle_donts')
            .insert({
              build_id: buildId,
              dont_item: build.playstyle.donts[i],
              order_index: i
            });

          if (dontError) throw dontError;
        }
      }

      if (build.playstyle.tips) {
        await supabase.from('build_playstyle_tips').delete().eq('build_id', buildId);
        for (let i = 0; i < build.playstyle.tips.length; i++) {
          const { error: tipError } = await supabase
            .from('build_playstyle_tips')
            .insert({
              build_id: buildId,
              tip: build.playstyle.tips[i],
              order_index: i
            });

          if (tipError) throw tipError;
        }
      }
    }

    return data;
  }
}
