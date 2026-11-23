-- Fix build scores and add missing constraints
ALTER TABLE builds ALTER COLUMN score SET NOT NULL DEFAULT 3.0;

-- Add missing indexes for better performance
CREATE INDEX IF NOT EXISTS idx_builds_hero_id ON builds(hero_id);
CREATE INDEX IF NOT EXISTS idx_builds_mood ON builds(mood);
CREATE INDEX IF NOT EXISTS idx_hero_moods_hero_id ON hero_moods(hero_id);
CREATE INDEX IF NOT EXISTS idx_hero_moods_mood ON hero_moods(mood);

-- Add constraint to ensure hero_id exists
ALTER TABLE builds DROP CONSTRAINT IF EXISTS fk_builds_hero_id;
ALTER TABLE builds ADD CONSTRAINT fk_builds_hero_id 
  FOREIGN KEY (hero_id) REFERENCES heroes(id) ON DELETE CASCADE;

-- Add constraint to ensure mood is valid
ALTER TABLE builds DROP CONSTRAINT IF EXISTS check_valid_mood;
ALTER TABLE builds ADD CONSTRAINT check_valid_mood 
  CHECK (mood IN ('aggressive', 'defensive', 'experimental', 'creative', 'chaos'));

-- Add constraint to ensure valid difficulty
ALTER TABLE heroes DROP CONSTRAINT IF EXISTS check_valid_difficulty;
ALTER TABLE heroes ADD CONSTRAINT check_valid_difficulty 
  CHECK (difficulty IN ('Easy', 'Medium', 'Hard'));

-- Add constraint to ensure valid role
ALTER TABLE heroes DROP CONSTRAINT IF EXISTS check_valid_role;
ALTER TABLE heroes ADD CONSTRAINT check_valid_role 
  CHECK (role IN ('Carry', 'Support', 'Mid', 'Initiator'));
