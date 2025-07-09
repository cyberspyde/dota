/*
  # Fix RLS policies for anonymous access

  1. Security Updates
    - Drop existing policies that might be blocking access
    - Create new policies that properly allow anonymous users to read data
    - Ensure all tables have proper read access for unauthenticated users

  2. Tables Updated
    - `heroes` - Allow anonymous read access
    - `hero_moods` - Allow anonymous read access  
    - `hero_strengths` - Allow anonymous read access
    - `hero_weaknesses` - Allow anonymous read access
    - `builds` - Allow anonymous read access
    - `items` - Allow anonymous read access
    - `playstyle_dos` - Allow anonymous read access
    - `playstyle_donts` - Allow anonymous read access
    - `playstyle_tips` - Allow anonymous read access

  3. Important Notes
    - All policies allow SELECT operations for anonymous users
    - RLS remains enabled for security
    - No write access is granted to anonymous users
*/

-- Drop existing policies to avoid conflicts
DROP POLICY IF EXISTS "Public read access" ON heroes;
DROP POLICY IF EXISTS "Public read access" ON hero_moods;
DROP POLICY IF EXISTS "Public read access" ON hero_strengths;
DROP POLICY IF EXISTS "Public read access" ON hero_weaknesses;
DROP POLICY IF EXISTS "Public read access" ON builds;
DROP POLICY IF EXISTS "Public read access" ON items;
DROP POLICY IF EXISTS "Public read access" ON playstyle_dos;
DROP POLICY IF EXISTS "Public read access" ON playstyle_donts;
DROP POLICY IF EXISTS "Public read access" ON playstyle_tips;

-- Create new policies that allow anonymous read access
CREATE POLICY "Allow anonymous read access" ON heroes
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON hero_moods
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON hero_strengths
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON hero_weaknesses
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON builds
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON items
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON playstyle_dos
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON playstyle_donts
    FOR SELECT TO anon
    USING (true);

CREATE POLICY "Allow anonymous read access" ON playstyle_tips
    FOR SELECT TO anon
    USING (true);

-- Also create policies for authenticated users (in case they exist)
CREATE POLICY "Allow authenticated read access" ON heroes
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON hero_moods
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON hero_strengths
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON hero_weaknesses
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON builds
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON items
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON playstyle_dos
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON playstyle_donts
    FOR SELECT TO authenticated
    USING (true);

CREATE POLICY "Allow authenticated read access" ON playstyle_tips
    FOR SELECT TO authenticated
    USING (true);