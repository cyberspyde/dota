/*
# Create Dota 2 Helper Database Schema

1. New Tables
   - `heroes` - Main heroes table with basic info
   - `hero_moods` - Hero mood associations
   - `hero_strengths` - Hero strength descriptions
   - `hero_weaknesses` - Hero weakness descriptions
   - `builds` - Hero build guides
   - `items` - Build items with phases and priorities
   - `playstyle_dos` - Playstyle recommendations
   - `playstyle_donts` - Playstyle warnings
   - `playstyle_tips` - Pro tips for builds

2. Security
   - Enable RLS on all tables
   - Add public read access policies
   - Secure data with proper permissions

3. Performance
   - Add indexes for common queries
   - Optimize for hero/build lookups
   - Support filtering by role, difficulty, mood
*/

-- Drop existing triggers and policies to avoid conflicts
DROP TRIGGER IF EXISTS update_heroes_updated_at ON heroes;
DROP TRIGGER IF EXISTS update_builds_updated_at ON builds;

-- Drop existing policies if they exist
DO $$ 
BEGIN
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'heroes'::regclass) THEN
        DROP POLICY "Public read access" ON heroes;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'hero_moods'::regclass) THEN
        DROP POLICY "Public read access" ON hero_moods;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'hero_strengths'::regclass) THEN
        DROP POLICY "Public read access" ON hero_strengths;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'hero_weaknesses'::regclass) THEN
        DROP POLICY "Public read access" ON hero_weaknesses;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'builds'::regclass) THEN
        DROP POLICY "Public read access" ON builds;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'items'::regclass) THEN
        DROP POLICY "Public read access" ON items;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'playstyle_dos'::regclass) THEN
        DROP POLICY "Public read access" ON playstyle_dos;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'playstyle_donts'::regclass) THEN
        DROP POLICY "Public read access" ON playstyle_donts;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policy WHERE polname = 'Public read access' AND polrelid = 'playstyle_tips'::regclass) THEN
        DROP POLICY "Public read access" ON playstyle_tips;
    END IF;
END $$;

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create heroes table
CREATE TABLE IF NOT EXISTS heroes (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    role VARCHAR(20) NOT NULL,
    difficulty VARCHAR(10) CHECK (difficulty IN ('Easy', 'Medium', 'Hard')),
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create hero_moods table (many-to-many relationship)
CREATE TABLE IF NOT EXISTS hero_moods (
    hero_id VARCHAR(50) REFERENCES heroes(id) ON DELETE CASCADE,
    mood VARCHAR(20) CHECK (mood IN ('aggressive', 'defensive', 'experimental', 'creative', 'chaos')),
    PRIMARY KEY (hero_id, mood)
);

-- Create hero_strengths table
CREATE TABLE IF NOT EXISTS hero_strengths (
    id SERIAL PRIMARY KEY,
    hero_id VARCHAR(50) REFERENCES heroes(id) ON DELETE CASCADE,
    strength TEXT NOT NULL,
    order_index INTEGER DEFAULT 0
);

-- Create hero_weaknesses table
CREATE TABLE IF NOT EXISTS hero_weaknesses (
    id SERIAL PRIMARY KEY,
    hero_id VARCHAR(50) REFERENCES heroes(id) ON DELETE CASCADE,
    weakness TEXT NOT NULL,
    order_index INTEGER DEFAULT 0
);

-- Create builds table
CREATE TABLE IF NOT EXISTS builds (
    id SERIAL PRIMARY KEY,
    hero_id VARCHAR(50) REFERENCES heroes(id) ON DELETE CASCADE,
    mood VARCHAR(20) CHECK (mood IN ('aggressive', 'defensive', 'experimental', 'creative', 'chaos')),
    early_game TEXT,
    mid_game TEXT,
    late_game TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(hero_id, mood)
);

-- Create items table
CREATE TABLE IF NOT EXISTS items (
    id SERIAL PRIMARY KEY,
    build_id INTEGER REFERENCES builds(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    cost INTEGER NOT NULL,
    phase VARCHAR(10) CHECK (phase IN ('Early', 'Mid', 'Late')),
    priority VARCHAR(20) CHECK (priority IN ('Core', 'Situational', 'Luxury')),
    description TEXT,
    order_index INTEGER DEFAULT 0
);

-- Create playstyle_dos table
CREATE TABLE IF NOT EXISTS playstyle_dos (
    id SERIAL PRIMARY KEY,
    build_id INTEGER REFERENCES builds(id) ON DELETE CASCADE,
    do_item TEXT NOT NULL,
    order_index INTEGER DEFAULT 0
);

-- Create playstyle_donts table
CREATE TABLE IF NOT EXISTS playstyle_donts (
    id SERIAL PRIMARY KEY,
    build_id INTEGER REFERENCES builds(id) ON DELETE CASCADE,
    dont_item TEXT NOT NULL,
    order_index INTEGER DEFAULT 0
);

-- Create playstyle_tips table
CREATE TABLE IF NOT EXISTS playstyle_tips (
    id SERIAL PRIMARY KEY,
    build_id INTEGER REFERENCES builds(id) ON DELETE CASCADE,
    tip TEXT NOT NULL,
    order_index INTEGER DEFAULT 0
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_heroes_role ON heroes(role);
CREATE INDEX IF NOT EXISTS idx_heroes_difficulty ON heroes(difficulty);
CREATE INDEX IF NOT EXISTS idx_hero_moods_mood ON hero_moods(mood);
CREATE INDEX IF NOT EXISTS idx_builds_hero_mood ON builds(hero_id, mood);
CREATE INDEX IF NOT EXISTS idx_items_build_phase ON items(build_id, phase);

-- Enable Row Level Security (RLS)
ALTER TABLE heroes ENABLE ROW LEVEL SECURITY;
ALTER TABLE hero_moods ENABLE ROW LEVEL SECURITY;
ALTER TABLE hero_strengths ENABLE ROW LEVEL SECURITY;
ALTER TABLE hero_weaknesses ENABLE ROW LEVEL SECURITY;
ALTER TABLE builds ENABLE ROW LEVEL SECURITY;
ALTER TABLE items ENABLE ROW LEVEL SECURITY;
ALTER TABLE playstyle_dos ENABLE ROW LEVEL SECURITY;
ALTER TABLE playstyle_donts ENABLE ROW LEVEL SECURITY;
ALTER TABLE playstyle_tips ENABLE ROW LEVEL SECURITY;

-- Create policies for public read access
CREATE POLICY "Public read access" ON heroes FOR SELECT USING (true);
CREATE POLICY "Public read access" ON hero_moods FOR SELECT USING (true);
CREATE POLICY "Public read access" ON hero_strengths FOR SELECT USING (true);
CREATE POLICY "Public read access" ON hero_weaknesses FOR SELECT USING (true);
CREATE POLICY "Public read access" ON builds FOR SELECT USING (true);
CREATE POLICY "Public read access" ON items FOR SELECT USING (true);
CREATE POLICY "Public read access" ON playstyle_dos FOR SELECT USING (true);
CREATE POLICY "Public read access" ON playstyle_donts FOR SELECT USING (true);
CREATE POLICY "Public read access" ON playstyle_tips FOR SELECT USING (true);

-- Create triggers for updated_at
CREATE TRIGGER update_heroes_updated_at BEFORE UPDATE ON heroes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_builds_updated_at BEFORE UPDATE ON builds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();