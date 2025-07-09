#!/usr/bin/env python3
"""
Dota 2 Hero and Build Manager for Supabase Database

A command-line tool to manage hero and build data in your Supabase database.
Supports adding individual heroes/builds or bulk importing from JSON files.

Usage:
    python manage_dota_data.py add-hero --json hero_data.json
    python manage_dota_data.py add-build --json build_data.json
    python manage_dota_data.py bulk-import --heroes heroes.json --builds builds.json
    python manage_dota_data.py add-hero --interactive
    python manage_dota_data.py list-heroes
    python manage_dota_data.py validate --json data.json
"""

import argparse
import json
import os
import sys
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict
from pathlib import Path
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

try:
    from supabase import create_client, Client
    from dotenv import load_dotenv
except ImportError:
    print("❌ Required packages not installed. Run: pip install supabase python-dotenv")
    sys.exit(1)

# Load environment variables
load_dotenv()

# Supabase configuration
SUPABASE_URL = os.getenv('VITE_SUPABASE_URL')
SUPABASE_SERVICE_KEY = os.getenv('SUPABASE_SERVICE_ROLE_KEY')

if not SUPABASE_URL or not SUPABASE_SERVICE_KEY:
    print("❌ Missing Supabase environment variables:")
    print("   VITE_SUPABASE_URL and SUPABASE_SERVICE_ROLE_KEY required")
    sys.exit(1)

# Initialize Supabase client
supabase: Client = create_client(SUPABASE_URL, SUPABASE_SERVICE_KEY)

# Data validation constants
VALID_ROLES = ['Carry', 'Support', 'Mid', 'Initiator']
VALID_DIFFICULTIES = ['Easy', 'Medium', 'Hard']
VALID_MOODS = ['aggressive', 'defensive', 'experimental', 'creative', 'chaos']
VALID_PHASES = ['Early', 'Mid', 'Late']
VALID_PRIORITIES = ['Core', 'Situational', 'Luxury']

@dataclass
class Hero:
    """Hero data structure"""
    id: str
    name: str
    role: str
    difficulty: str
    moods: List[str]
    description: str
    strengths: List[str]
    weaknesses: List[str]

@dataclass
class Item:
    """Item data structure"""
    id: Union[str, int]
    name: str
    cost: int
    phase: str
    priority: str
    description: str

@dataclass
class Playstyle:
    """Playstyle data structure"""
    dos: List[str]
    donts: List[str]
    tips: List[str]

@dataclass
class Gameplan:
    """Gameplan data structure"""
    early: str
    mid: str
    late: str

@dataclass
class Build:
    """Build data structure"""
    heroId: str
    mood: str
    items: List[Item]
    playstyle: Playstyle
    gameplan: Gameplan

class DataValidator:
    """Validates hero and build data"""
    
    @staticmethod
    def validate_hero(hero_data: Dict[str, Any]) -> List[str]:
        """Validate hero data and return list of errors"""
        errors = []
        
        # Required fields
        required_fields = ['id', 'name', 'role', 'difficulty', 'moods', 'description', 'strengths', 'weaknesses']
        for field in required_fields:
            if field not in hero_data:
                errors.append(f"Missing required field: {field}")
        
        # Validate role
        if hero_data.get('role') not in VALID_ROLES:
            errors.append(f"Invalid role: {hero_data.get('role')}. Must be one of: {VALID_ROLES}")
        
        # Validate difficulty
        if hero_data.get('difficulty') not in VALID_DIFFICULTIES:
            errors.append(f"Invalid difficulty: {hero_data.get('difficulty')}. Must be one of: {VALID_DIFFICULTIES}")
        
        # Validate moods
        moods = hero_data.get('moods', [])
        if not isinstance(moods, list) or len(moods) == 0:
            errors.append("Moods must be a non-empty list")
        else:
            for mood in moods:
                if mood not in VALID_MOODS:
                    errors.append(f"Invalid mood: {mood}. Must be one of: {VALID_MOODS}")
        
        # Validate strengths and weaknesses
        strengths = hero_data.get('strengths', [])
        weaknesses = hero_data.get('weaknesses', [])
        
        if not isinstance(strengths, list) or len(strengths) < 3:
            errors.append("Strengths must be a list with at least 3 items")
        
        if not isinstance(weaknesses, list) or len(weaknesses) < 3:
            errors.append("Weaknesses must be a list with at least 3 items")
        
        return errors
    
    @staticmethod
    def validate_build(build_data: Dict[str, Any]) -> List[str]:
        """Validate build data and return list of errors"""
        errors = []
        
        # Required fields
        required_fields = ['heroId', 'mood', 'items', 'playstyle', 'gameplan']
        for field in required_fields:
            if field not in build_data:
                errors.append(f"Missing required field: {field}")
        
        # Validate mood
        if build_data.get('mood') not in VALID_MOODS:
            errors.append(f"Invalid mood: {build_data.get('mood')}. Must be one of: {VALID_MOODS}")
        
        # Validate items
        items = build_data.get('items', [])
        if not isinstance(items, list) or len(items) < 4:
            errors.append("Items must be a list with at least 4 items")
        else:
            for i, item in enumerate(items):
                item_errors = DataValidator.validate_item(item, i)
                errors.extend(item_errors)
        
        # Validate playstyle
        playstyle = build_data.get('playstyle', {})
        if not isinstance(playstyle, dict):
            errors.append("Playstyle must be an object")
        else:
            for key in ['dos', 'donts', 'tips']:
                if key not in playstyle:
                    errors.append(f"Missing playstyle field: {key}")
                elif not isinstance(playstyle[key], list) or len(playstyle[key]) < 3:
                    errors.append(f"Playstyle {key} must be a list with at least 3 items")
        
        # Validate gameplan
        gameplan = build_data.get('gameplan', {})
        if not isinstance(gameplan, dict):
            errors.append("Gameplan must be an object")
        else:
            for key in ['early', 'mid', 'late']:
                if key not in gameplan:
                    errors.append(f"Missing gameplan field: {key}")
                elif not isinstance(gameplan[key], str) or len(gameplan[key].strip()) == 0:
                    errors.append(f"Gameplan {key} must be a non-empty string")
        
        return errors
    
    @staticmethod
    def validate_item(item_data: Dict[str, Any], index: int) -> List[str]:
        """Validate individual item data"""
        errors = []
        
        # Required fields
        required_fields = ['id', 'name', 'cost', 'phase', 'priority', 'description']
        for field in required_fields:
            if field not in item_data:
                errors.append(f"Item {index}: Missing required field: {field}")
        
        # Validate cost
        cost = item_data.get('cost')
        if not isinstance(cost, int) or cost < 0:
            errors.append(f"Item {index}: Cost must be a non-negative integer")
        
        # Validate phase
        if item_data.get('phase') not in VALID_PHASES:
            errors.append(f"Item {index}: Invalid phase: {item_data.get('phase')}. Must be one of: {VALID_PHASES}")
        
        # Validate priority
        if item_data.get('priority') not in VALID_PRIORITIES:
            errors.append(f"Item {index}: Invalid priority: {item_data.get('priority')}. Must be one of: {VALID_PRIORITIES}")
        
        return errors

class DatabaseManager:
    """Manages database operations"""
    
    def __init__(self, supabase_client: Client):
        self.supabase = supabase_client
    
    def add_hero(self, hero: Hero) -> bool:
        """Add a hero to the database"""
        try:
            print(f"🦸 Adding hero: {hero.name}...")
            
            # Insert hero
            hero_result = self.supabase.table('heroes').upsert({
                'id': hero.id,
                'name': hero.name,
                'role': hero.role,
                'difficulty': hero.difficulty,
                'description': hero.description
            }, on_conflict='id').execute()
            
            if hero_result.data is None:
                print(f"❌ Failed to add hero: {hero.name}")
                return False
            
            # Insert hero moods
            for mood in hero.moods:
                mood_result = self.supabase.table('hero_moods').upsert({
                    'hero_id': hero.id,
                    'mood': mood
                }, on_conflict='hero_id,mood').execute()
                
                if mood_result.data is None:
                    print(f"⚠️  Warning: Failed to add mood {mood} for hero {hero.name}")
            
            # Insert hero strengths
            for i, strength in enumerate(hero.strengths):
                strength_result = self.supabase.table('hero_strengths').upsert({
                    'hero_id': hero.id,
                    'strength': strength,
                    'order_index': i
                }).execute()
                
                if strength_result.data is None:
                    print(f"⚠️  Warning: Failed to add strength for hero {hero.name}")
            
            # Insert hero weaknesses
            for i, weakness in enumerate(hero.weaknesses):
                weakness_result = self.supabase.table('hero_weaknesses').upsert({
                    'hero_id': hero.id,
                    'weakness': weakness,
                    'order_index': i
                }).execute()
                
                if weakness_result.data is None:
                    print(f"⚠️  Warning: Failed to add weakness for hero {hero.name}")
            
            print(f"✅ Successfully added hero: {hero.name}")
            return True
            
        except Exception as e:
            print(f"❌ Error adding hero {hero.name}: {str(e)}")
            return False
    
    def add_build(self, build: Build) -> bool:
        """Add a build to the database"""
        try:
            print(f"🔨 Adding build: {build.heroId} ({build.mood})...")
            
            # Insert build
            build_result = self.supabase.table('builds').upsert({
                'hero_id': build.heroId,
                'mood': build.mood,
                'early_game': build.gameplan.early,
                'mid_game': build.gameplan.mid,
                'late_game': build.gameplan.late
            }, on_conflict='hero_id,mood').execute()
            
            if not build_result.data:
                print(f"❌ Failed to add build: {build.heroId} ({build.mood})")
                return False
            
            build_id = build_result.data[0]['id']
            
            # Insert items
            for i, item in enumerate(build.items):
                item_result = self.supabase.table('items').upsert({
                    'build_id': build_id,
                    'name': item.name,
                    'cost': item.cost,
                    'phase': item.phase,
                    'priority': item.priority,
                    'description': item.description,
                    'order_index': i
                }).execute()
                
                if item_result.data is None:
                    print(f"⚠️  Warning: Failed to add item {item.name}")
            
            # Insert playstyle dos
            for i, do_item in enumerate(build.playstyle.dos):
                do_result = self.supabase.table('playstyle_dos').upsert({
                    'build_id': build_id,
                    'do_item': do_item,
                    'order_index': i
                }).execute()
                
                if do_result.data is None:
                    print(f"⚠️  Warning: Failed to add playstyle do item")
            
            # Insert playstyle donts
            for i, dont_item in enumerate(build.playstyle.donts):
                dont_result = self.supabase.table('playstyle_donts').upsert({
                    'build_id': build_id,
                    'dont_item': dont_item,
                    'order_index': i
                }).execute()
                
                if dont_result.data is None:
                    print(f"⚠️  Warning: Failed to add playstyle dont item")
            
            # Insert playstyle tips
            for i, tip in enumerate(build.playstyle.tips):
                tip_result = self.supabase.table('playstyle_tips').upsert({
                    'build_id': build_id,
                    'tip': tip,
                    'order_index': i
                }).execute()
                
                if tip_result.data is None:
                    print(f"⚠️  Warning: Failed to add playstyle tip")
            
            print(f"✅ Successfully added build: {build.heroId} ({build.mood})")
            return True
            
        except Exception as e:
            print(f"❌ Error adding build {build.heroId} ({build.mood}): {str(e)}")
            return False
    
    def list_heroes(self) -> List[Dict[str, Any]]:
        """List all heroes in the database"""
        try:
            result = self.supabase.table('heroes').select('*').execute()
            return result.data or []
        except Exception as e:
            print(f"❌ Error listing heroes: {str(e)}")
            return []
    
    def hero_exists(self, hero_id: str) -> bool:
        """Check if a hero exists in the database"""
        try:
            result = self.supabase.table('heroes').select('id').eq('id', hero_id).execute()
            return len(result.data) > 0
        except Exception as e:
            print(f"❌ Error checking hero existence: {str(e)}")
            return False

class InteractiveInput:
    """Handles interactive user input"""
    
    @staticmethod
    def get_hero_input() -> Dict[str, Any]:
        """Get hero data from user input"""
        print("🦸 Creating a new hero...")
        print("=" * 50)
        
        hero_data = {}
        
        # Basic info
        hero_data['id'] = input("Hero ID (lowercase, use dashes): ").strip()
        hero_data['name'] = input("Hero Name: ").strip()
        
        # Role
        print(f"Available roles: {', '.join(VALID_ROLES)}")
        hero_data['role'] = input("Role: ").strip()
        
        # Difficulty
        print(f"Available difficulties: {', '.join(VALID_DIFFICULTIES)}")
        hero_data['difficulty'] = input("Difficulty: ").strip()
        
        # Moods
        print(f"Available moods: {', '.join(VALID_MOODS)}")
        moods_input = input("Moods (comma-separated): ").strip()
        hero_data['moods'] = [mood.strip() for mood in moods_input.split(',')]
        
        # Description
        hero_data['description'] = input("Description: ").strip()
        
        # Strengths
        print("Enter strengths (press Enter twice to finish):")
        strengths = []
        while True:
            strength = input(f"Strength {len(strengths) + 1}: ").strip()
            if not strength:
                break
            strengths.append(strength)
        hero_data['strengths'] = strengths
        
        # Weaknesses
        print("Enter weaknesses (press Enter twice to finish):")
        weaknesses = []
        while True:
            weakness = input(f"Weakness {len(weaknesses) + 1}: ").strip()
            if not weakness:
                break
            weaknesses.append(weakness)
        hero_data['weaknesses'] = weaknesses
        
        return hero_data
    
    @staticmethod
    def get_build_input() -> Dict[str, Any]:
        """Get build data from user input"""
        print("🔨 Creating a new build...")
        print("=" * 50)
        
        build_data = {}
        
        # Basic info
        build_data['heroId'] = input("Hero ID: ").strip()
        print(f"Available moods: {', '.join(VALID_MOODS)}")
        build_data['mood'] = input("Mood: ").strip()
        
        # Items
        print("Enter items (press Enter twice to finish):")
        items = []
        while True:
            print(f"\nItem {len(items) + 1}:")
            item_id = input("  Item ID: ").strip()
            if not item_id:
                break
            
            item_name = input("  Item Name: ").strip()
            item_cost = int(input("  Item Cost: ").strip())
            
            print(f"  Available phases: {', '.join(VALID_PHASES)}")
            item_phase = input("  Phase: ").strip()
            
            print(f"  Available priorities: {', '.join(VALID_PRIORITIES)}")
            item_priority = input("  Priority: ").strip()
            
            item_description = input("  Description: ").strip()
            
            items.append({
                'id': item_id,
                'name': item_name,
                'cost': item_cost,
                'phase': item_phase,
                'priority': item_priority,
                'description': item_description
            })
        
        build_data['items'] = items
        
        # Playstyle
        playstyle = {}
        
        # Dos
        print("Enter playstyle dos (press Enter twice to finish):")
        dos = []
        while True:
            do_item = input(f"Do {len(dos) + 1}: ").strip()
            if not do_item:
                break
            dos.append(do_item)
        playstyle['dos'] = dos
        
        # Donts
        print("Enter playstyle donts (press Enter twice to finish):")
        donts = []
        while True:
            dont_item = input(f"Don't {len(donts) + 1}: ").strip()
            if not dont_item:
                break
            donts.append(dont_item)
        playstyle['donts'] = donts
        
        # Tips
        print("Enter playstyle tips (press Enter twice to finish):")
        tips = []
        while True:
            tip = input(f"Tip {len(tips) + 1}: ").strip()
            if not tip:
                break
            tips.append(tip)
        playstyle['tips'] = tips
        
        build_data['playstyle'] = playstyle
        
        # Gameplan
        gameplan = {}
        gameplan['early'] = input("Early game strategy: ").strip()
        gameplan['mid'] = input("Mid game strategy: ").strip()
        gameplan['late'] = input("Late game strategy: ").strip()
        
        build_data['gameplan'] = gameplan
        
        return build_data

def load_json_file(file_path: str) -> Dict[str, Any]:
    """Load and parse JSON file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except FileNotFoundError:
        print(f"❌ File not found: {file_path}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON in file {file_path}: {str(e)}")
        sys.exit(1)

def save_json_file(data: Dict[str, Any], file_path: str):
    """Save data to JSON file"""
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        print(f"✅ Data saved to {file_path}")
    except Exception as e:
        print(f"❌ Error saving file {file_path}: {str(e)}")

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description='Manage Dota 2 heroes and builds in Supabase database',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s add-hero --json hero.json
  %(prog)s add-build --json build.json
  %(prog)s bulk-import --heroes heroes.json --builds builds.json
  %(prog)s add-hero --interactive
  %(prog)s list-heroes
  %(prog)s validate --json data.json
  %(prog)s create-templates
        """
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Add hero command
    add_hero_parser = subparsers.add_parser('add-hero', help='Add a new hero')
    add_hero_group = add_hero_parser.add_mutually_exclusive_group(required=True)
    add_hero_group.add_argument('--json', help='JSON file containing hero data')
    add_hero_group.add_argument('--interactive', action='store_true', help='Interactive input mode')
    
    # Add build command
    add_build_parser = subparsers.add_parser('add-build', help='Add a new build')
    add_build_group = add_build_parser.add_mutually_exclusive_group(required=True)
    add_build_group.add_argument('--json', help='JSON file containing build data')
    add_build_group.add_argument('--interactive', action='store_true', help='Interactive input mode')
    
    # Bulk import command
    bulk_parser = subparsers.add_parser('bulk-import', help='Bulk import heroes and builds')
    bulk_parser.add_argument('--heroes', help='JSON file containing heroes array')
    bulk_parser.add_argument('--builds', help='JSON file containing builds array')
    
    # List heroes command
    subparsers.add_parser('list-heroes', help='List all heroes in database')
    
    # Validate command
    validate_parser = subparsers.add_parser('validate', help='Validate JSON data')
    validate_parser.add_argument('--json', required=True, help='JSON file to validate')
    validate_parser.add_argument('--type', choices=['hero', 'build'], help='Data type to validate')
    
    # Create templates command
    subparsers.add_parser('create-templates', help='Create JSON template files')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    # Initialize database manager
    db_manager = DatabaseManager(supabase)
    validator = DataValidator()
    
    # Handle commands
    if args.command == 'add-hero':
        if args.json:
            hero_data = load_json_file(args.json)
            errors = validator.validate_hero(hero_data)
            if errors:
                print("❌ Validation errors:")
                for error in errors:
                    print(f"  - {error}")
                return
            
            hero = Hero(**hero_data)
            db_manager.add_hero(hero)
        
        elif args.interactive:
            hero_data = InteractiveInput.get_hero_input()
            errors = validator.validate_hero(hero_data)
            if errors:
                print("❌ Validation errors:")
                for error in errors:
                    print(f"  - {error}")
                return
            
            hero = Hero(**hero_data)
            db_manager.add_hero(hero)
    
    elif args.command == 'add-build':
        if args.json:
            build_data = load_json_file(args.json)
            errors = validator.validate_build(build_data)
            if errors:
                print("❌ Validation errors:")
                for error in errors:
                    print(f"  - {error}")
                return
            
            # Check if hero exists
            if not db_manager.hero_exists(build_data['heroId']):
                print(f"❌ Hero '{build_data['heroId']}' does not exist in database")
                return
            
            # Convert nested dictionaries to dataclasses
            items = [Item(**item) for item in build_data['items']]
            playstyle = Playstyle(**build_data['playstyle'])
            gameplan = Gameplan(**build_data['gameplan'])
            
            build = Build(
                heroId=build_data['heroId'],
                mood=build_data['mood'],
                items=items,
                playstyle=playstyle,
                gameplan=gameplan
            )
            db_manager.add_build(build)
        
        elif args.interactive:
            build_data = InteractiveInput.get_build_input()
            errors = validator.validate_build(build_data)
            if errors:
                print("❌ Validation errors:")
                for error in errors:
                    print(f"  - {error}")
                return
            
            # Check if hero exists
            if not db_manager.hero_exists(build_data['heroId']):
                print(f"❌ Hero '{build_data['heroId']}' does not exist in database")
                return
            
            # Convert nested dictionaries to dataclasses
            items = [Item(**item) for item in build_data['items']]
            playstyle = Playstyle(**build_data['playstyle'])
            gameplan = Gameplan(**build_data['gameplan'])
            
            build = Build(
                heroId=build_data['heroId'],
                mood=build_data['mood'],
                items=items,
                playstyle=playstyle,
                gameplan=gameplan
            )
            db_manager.add_build(build)
    
    elif args.command == 'bulk-import':
        success_count = 0
        fail_count = 0
        
        if args.heroes:
            heroes_data = load_json_file(args.heroes)
            if not isinstance(heroes_data, list):
                print("❌ Heroes file must contain an array of hero objects")
                return
            
            print(f"📝 Processing {len(heroes_data)} heroes...")
            for hero_data in heroes_data:
                errors = validator.validate_hero(hero_data)
                if errors:
                    print(f"❌ Validation errors for hero {hero_data.get('name', 'unknown')}:")
                    for error in errors:
                        print(f"  - {error}")
                    fail_count += 1
                    continue
                
                hero = Hero(**hero_data)
                if db_manager.add_hero(hero):
                    success_count += 1
                else:
                    fail_count += 1
        
        if args.builds:
            builds_data = load_json_file(args.builds)
            if not isinstance(builds_data, list):
                print("❌ Builds file must contain an array of build objects")
                return
            
            print(f"📝 Processing {len(builds_data)} builds...")
            for build_data in builds_data:
                errors = validator.validate_build(build_data)
                if errors:
                    print(f"❌ Validation errors for build {build_data.get('heroId', 'unknown')}:")
                    for error in errors:
                        print(f"  - {error}")
                    fail_count += 1
                    continue
                
                # Check if hero exists
                if not db_manager.hero_exists(build_data['heroId']):
                    print(f"❌ Hero '{build_data['heroId']}' does not exist in database")
                    fail_count += 1
                    continue
                
                # Convert nested dictionaries to dataclasses
                items = [Item(**item) for item in build_data['items']]
                playstyle = Playstyle(**build_data['playstyle'])
                gameplan = Gameplan(**build_data['gameplan'])
                
                build = Build(
                    heroId=build_data['heroId'],
                    mood=build_data['mood'],
                    items=items,
                    playstyle=playstyle,
                    gameplan=gameplan
                )
                
                if db_manager.add_build(build):
                    success_count += 1
                else:
                    fail_count += 1
        
        print(f"\n📊 Summary:")
        print(f"✅ Successfully processed: {success_count} items")
        print(f"❌ Failed: {fail_count} items")
    
    elif args.command == 'list-heroes':
        heroes = db_manager.list_heroes()
        if not heroes:
            print("No heroes found in database")
            return
        
        print(f"📋 Found {len(heroes)} heroes in database:")
        print("-" * 60)
        for hero in heroes:
            print(f"🦸 {hero['name']} ({hero['id']})")
            print(f"   Role: {hero['role']} | Difficulty: {hero['difficulty']}")
            print(f"   Description: {hero['description'][:60]}...")
            print()
    
    elif args.command == 'validate':
        data = load_json_file(args.json)
        
        if args.type == 'hero':
            errors = validator.validate_hero(data)
        elif args.type == 'build':
            errors = validator.validate_build(data)
        else:
            # Try to auto-detect type
            if 'heroId' in data and 'mood' in data:
                errors = validator.validate_build(data)
                print("🔍 Auto-detected as build data")
            elif 'role' in data and 'difficulty' in data:
                errors = validator.validate_hero(data)
                print("🔍 Auto-detected as hero data")
            else:
                print("❌ Cannot determine data type. Use --type parameter")
                return
        
        if errors:
            print("❌ Validation errors:")
            for error in errors:
                print(f"  - {error}")
        else:
            print("✅ Data validation passed!")
    
    elif args.command == 'create-templates':
        create_template_files()

def create_template_files():
    """Create JSON template files"""
    hero_template = {
        "id": "example-hero",
        "name": "Example Hero",
        "role": "Carry",
        "difficulty": "Medium",
        "moods": ["aggressive", "experimental"],
        "description": "An example hero for demonstration purposes",
        "strengths": [
            "High damage output",
            "Good mobility",
            "Strong late game scaling"
        ],
        "weaknesses": [
            "Fragile early game",
            "Needs farm priority",
            "Vulnerable to ganks"
        ]
    }
    
    build_template = {
        "heroId": "example-hero",
        "mood": "aggressive",
        "items": [
            {
                "id": "boots",
                "name": "Boots of Speed",
                "cost": 500,
                "phase": "Early",
                "priority": "Core",
                "description": "Basic mobility item"
            },
            {
                "id": "wraith-band",
                "name": "Wraith Band",
                "cost": 505,
                "phase": "Early",
                "priority": "Core",
                "description": "Early game stats"
            },
            {
                "id": "power-treads",
                "name": "Power Treads",
                "cost": 1400,
                "phase": "Early",
                "priority": "Core",
                "description": "Attack speed and stat switching"
            },
            {
                "id": "battle-fury",
                "name": "Battle Fury",
                "cost": 4100,
                "phase": "Mid",
                "priority": "Core",
                "description": "Farming acceleration and cleave"
            },
            {
                "id": "manta-style",
                "name": "Manta Style",
                "cost": 4850,
                "phase": "Mid",
                "priority": "Core",
                "description": "Illusions and dispel"
            },
            {
                "id": "butterfly",
                "name": "Butterfly",
                "cost": 5525,
                "phase": "Late",
                "priority": "Luxury",
                "description": "Evasion and attack speed"
            }
        ],
        "playstyle": {
            "dos": [
                "Farm efficiently with Battle Fury",
                "Use Manta to dispel debuffs",
                "Join fights when you have key items",
                "Split push to create pressure"
            ],
            "donts": [
                "Don't fight without core items",
                "Don't neglect farming patterns",
                "Don't get caught without escape",
                "Don't ignore team coordination"
            ],
            "tips": [
                "Practice last-hitting in demo mode",
                "Learn optimal farming routes",
                "Time your power spikes with objectives",
                "Use illusions to scout and farm"
            ]
        },
        "gameplan": {
            "early": "Focus on farming safely and efficiently. Get Battle Fury as quickly as possible while avoiding unnecessary fights.",
            "mid": "Use Battle Fury to farm quickly, get Manta Style, and start participating in key team fights and objectives.",
            "late": "Become a major damage dealer with Butterfly. Use split push pressure and join decisive team fights."
        }
    }
    
    # Save templates
    save_json_file(hero_template, 'hero_template.json')
    save_json_file(build_template, 'build_template.json')
    
    # Create bulk import examples
    heroes_bulk = [hero_template]
    builds_bulk = [build_template]
    
    save_json_file(heroes_bulk, 'heroes_bulk_template.json')
    save_json_file(builds_bulk, 'builds_bulk_template.json')
    
    print("✅ Template files created:")
    print("  - hero_template.json")
    print("  - build_template.json")
    print("  - heroes_bulk_template.json")
    print("  - builds_bulk_template.json")

if __name__ == '__main__':
    main()