# Python Supabase Data Manager

A comprehensive command-line tool for managing Dota 2 hero and build data in your Supabase database.

## Features

- ✅ Add individual heroes and builds
- ✅ Bulk import from JSON files
- ✅ Interactive input mode
- ✅ Data validation before upload
- ✅ List existing heroes
- ✅ Validate JSON files
- ✅ Create template files
- ✅ Full error handling and feedback

## Installation

### 1. Install Python Dependencies

```bash
pip install -r scripts/requirements.txt
```

### 2. Set Up Environment Variables

Make sure your `.env` file contains:
```env
VITE_SUPABASE_URL=your_supabase_project_url
SUPABASE_SERVICE_ROLE_KEY=your_service_role_key
```

### 3. Make Script Executable (Optional)

```bash
chmod +x scripts/manage_dota_data.py
```

## Usage

### Basic Commands

```bash
# Add a hero from JSON file
python scripts/manage_dota_data.py add-hero --json hero_data.json

# Add a build from JSON file
python scripts/manage_dota_data.py add-build --json build_data.json

# Interactive mode for adding heroes
python scripts/manage_dota_data.py add-hero --interactive

# Interactive mode for adding builds
python scripts/manage_dota_data.py add-build --interactive

# Bulk import heroes and builds
python scripts/manage_dota_data.py bulk-import --heroes heroes.json --builds builds.json

# List all heroes in database
python scripts/manage_dota_data.py list-heroes

# Validate JSON data
python scripts/manage_dota_data.py validate --json data.json

# Create template files
python scripts/manage_dota_data.py create-templates
```

### Advanced Examples

```bash
# Validate specific data type
python scripts/manage_dota_data.py validate --json hero.json --type hero
python scripts/manage_dota_data.py validate --json build.json --type build

# Bulk import only heroes
python scripts/manage_dota_data.py bulk-import --heroes heroes.json

# Bulk import only builds
python scripts/manage_dota_data.py bulk-import --builds builds.json
```

## Data Structure

### Hero JSON Format

```json
{
  "id": "hero-id",
  "name": "Hero Name",
  "role": "Carry",
  "difficulty": "Medium",
  "moods": ["aggressive", "experimental"],
  "description": "Hero description",
  "strengths": [
    "First strength",
    "Second strength",
    "Third strength"
  ],
  "weaknesses": [
    "First weakness",
    "Second weakness",
    "Third weakness"
  ]
}
```

### Build JSON Format

```json
{
  "heroId": "hero-id",
  "mood": "aggressive",
  "items": [
    {
      "id": "boots",
      "name": "Boots of Speed",
      "cost": 500,
      "phase": "Early",
      "priority": "Core",
      "description": "Basic mobility"
    }
  ],
  "playstyle": {
    "dos": [
      "First thing to do",
      "Second thing to do"
    ],
    "donts": [
      "First thing not to do",
      "Second thing not to do"
    ],
    "tips": [
      "First pro tip",
      "Second pro tip"
    ]
  },
  "gameplan": {
    "early": "Early game strategy",
    "mid": "Mid game strategy",
    "late": "Late game strategy"
  }
}
```

## Valid Values

### Hero Fields
- **role**: `Carry`, `Support`, `Mid`, `Initiator`
- **difficulty**: `Easy`, `Medium`, `Hard`
- **moods**: `aggressive`, `defensive`, `experimental`, `creative`, `chaos`

### Build Fields
- **mood**: `aggressive`, `defensive`, `experimental`, `creative`, `chaos`
- **item.phase**: `Early`, `Mid`, `Late`
- **item.priority**: `Core`, `Situational`, `Luxury`

## Interactive Mode

The interactive mode guides you through creating heroes and builds step by step:

```bash
# Interactive hero creation
python scripts/manage_dota_data.py add-hero --interactive

# Interactive build creation
python scripts/manage_dota_data.py add-build --interactive
```

## Bulk Import

For bulk importing, create JSON files with arrays of objects:

**heroes.json**:
```json
[
  {
    "id": "hero1",
    "name": "Hero 1",
    // ... other hero fields
  },
  {
    "id": "hero2",
    "name": "Hero 2",
    // ... other hero fields
  }
]
```

**builds.json**:
```json
[
  {
    "heroId": "hero1",
    "mood": "aggressive",
    // ... other build fields
  },
  {
    "heroId": "hero2",
    "mood": "defensive",
    // ... other build fields
  }
]
```

## Data Validation

The script includes comprehensive validation:

- Required fields check
- Data type validation
- Value range validation
- Foreign key constraints
- Minimum/maximum requirements

### Validation Examples

```bash
# Validate a hero file
python scripts/manage_dota_data.py validate --json hero.json --type hero

# Validate a build file
python scripts/manage_dota_data.py validate --json build.json --type build

# Auto-detect data type
python scripts/manage_dota_data.py validate --json data.json
```

## Error Handling

The script provides detailed error messages:

- ✅ **Success**: Clear success messages with emoji indicators
- ⚠️ **Warnings**: Non-critical issues that don't stop execution
- ❌ **Errors**: Critical issues with detailed explanations
- 🔍 **Validation**: Comprehensive validation error reporting

## Template Files

Generate template files to get started quickly:

```bash
python scripts/manage_dota_data.py create-templates
```

This creates:
- `hero_template.json` - Single hero template
- `build_template.json` - Single build template
- `heroes_bulk_template.json` - Bulk heroes template
- `builds_bulk_template.json` - Bulk builds template

## Troubleshooting

### Common Issues

1. **Missing environment variables**:
   ```
   ❌ Missing Supabase environment variables
   ```
   Solution: Check your `.env` file has the correct variables.

2. **Invalid JSON format**:
   ```
   ❌ Invalid JSON in file data.json
   ```
   Solution: Validate your JSON syntax.

3. **Hero doesn't exist**:
   ```
   ❌ Hero 'hero-id' does not exist in database
   ```
   Solution: Create the hero first before adding builds.

4. **Validation errors**:
   ```
   ❌ Validation errors:
     - Invalid role: Fighter. Must be one of: ['Carry', 'Support', 'Mid', 'Initiator']
   ```
   Solution: Use valid values from the documentation.

### Debug Mode

For detailed logging, set the logging level:

```python
# Add to the top of the script
import logging
logging.basicConfig(level=logging.DEBUG)
```

## Examples

### Adding a Simple Hero

```bash
# Create hero file
cat > pudge.json << EOF
{
  "id": "pudge",
  "name": "Pudge",
  "role": "Support",
  "difficulty": "Easy",
  "moods": ["aggressive", "chaos"],
  "description": "A tanky support who excels at picking off enemies with hooks",
  "strengths": [
    "High HP pool",
    "Game-changing hooks",
    "Strong initiation"
  ],
  "weaknesses": [
    "Low mobility",
    "Mana dependent",
    "Skill shot reliant"
  ]
}
EOF

# Add to database
python scripts/manage_dota_data.py add-hero --json pudge.json
```

### Adding a Build

```bash
# Create build file
cat > pudge_build.json << EOF
{
  "heroId": "pudge",
  "mood": "aggressive",
  "items": [
    {
      "id": "boots",
      "name": "Boots of Speed",
      "cost": 500,
      "phase": "Early",
      "priority": "Core",
      "description": "Basic mobility"
    },
    {
      "id": "blink",
      "name": "Blink Dagger",
      "cost": 2250,
      "phase": "Mid",
      "priority": "Core",
      "description": "Initiation and positioning"
    }
  ],
  "playstyle": {
    "dos": [
      "Practice hook accuracy",
      "Use Rot to slow enemies",
      "Position behind trees"
    ],
    "donts": [
      "Don't waste hooks on creeps",
      "Don't hook enemies toward their team",
      "Don't neglect mana management"
    ],
    "tips": [
      "Hook has longer range than it appears",
      "Use trees to hide hook animations",
      "Dismember heals you while channeling"
    ]
  },
  "gameplan": {
    "early": "Lane with your carry, harass with Rot, practice hooks",
    "mid": "Get Blink Dagger, start making plays around the map",
    "late": "Become an unkillable tank, initiate fights"
  }
}
EOF

# Add to database
python scripts/manage_dota_data.py add-build --json pudge_build.json
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Add your heroes/builds using the script
4. Submit a pull request

## License

This project is licensed under the MIT License.