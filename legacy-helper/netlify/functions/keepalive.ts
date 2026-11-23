import { Handler } from '@netlify/functions';
import { createClient } from '@supabase/supabase-js';

// Lightweight daily (scheduled) ping to keep Supabase project warm.
// Runs via Netlify Scheduled Functions (see netlify.toml).
// Makes a minimal, cached-friendly query so the database counts activity
// but uses negligible resources.

const supabaseUrl = process.env.VITE_SUPABASE_URL || '';
const supabaseAnonKey = process.env.VITE_SUPABASE_ANON_KEY || '';

// Use anon key – no sensitive writes; just a tiny read.
const supabase = createClient(supabaseUrl, supabaseAnonKey);

export const handler: Handler = async () => {
  const start = Date.now();
  try {
    // Smallest possible query: fetch 1 hero id (works even if table empty – returns empty array)
    const { error } = await supabase.from('heroes').select('id', { count: 'exact', head: false }).limit(1);
    if (error) {
      console.error('[keepalive] Supabase ping failed:', error.message);
      return {
        statusCode: 500,
        body: JSON.stringify({ status: 'error', message: error.message })
      };
    }
    const ms = Date.now() - start;
    return {
      statusCode: 200,
      body: JSON.stringify({ status: 'ok', tookMs: ms, ts: new Date().toISOString() })
    };
  } catch (e) {
    const msg = e instanceof Error ? e.message : 'Unknown error';
    console.error('[keepalive] Unexpected error:', msg);
    return { statusCode: 500, body: JSON.stringify({ status: 'error', message: msg }) };
  }
};
