-- Create database schema for Dota 2 Helper

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

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_heroes_updated_at BEFORE UPDATE ON heroes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_builds_updated_at BEFORE UPDATE ON builds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();