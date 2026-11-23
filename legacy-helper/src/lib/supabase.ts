import { createClient } from '@supabase/supabase-js';

// Use environment variables differently based on environment
const getEnvVar = (name: string): string => {
  if (typeof window !== 'undefined') {
    // Client-side: use import.meta.env
    return (import.meta.env as any)[name] || '';
  } else {
    // Server-side: use process.env
    return process.env[name] || '';
  }
};

const supabaseUrl = getEnvVar('VITE_SUPABASE_URL');
const supabaseAnonKey = getEnvVar('VITE_SUPABASE_ANON_KEY');

if (!supabaseUrl || !supabaseAnonKey) {
  console.error('Missing Supabase environment variables');
  console.error('VITE_SUPABASE_URL:', !!supabaseUrl);
  console.error('VITE_SUPABASE_ANON_KEY:', !!supabaseAnonKey);
}

export const supabase = createClient(supabaseUrl, supabaseAnonKey);

// For server-side operations (when needed)
export const supabaseAdmin = typeof process !== 'undefined' 
  ? createClient(
      supabaseUrl,
      getEnvVar('SUPABASE_SERVICE_ROLE_KEY') || supabaseAnonKey
    )
  : undefined;