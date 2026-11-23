#!/usr/bin/env python3
"""
Dota 2 Database Duplicate Cleanup Script

This script identifies and removes duplicate entries from the Supabase database.
It checks for duplicates in heroes, builds, and related tables.

Usage:
    python cleanup_duplicates.py --dry-run  # Show duplicates without removing
    python cleanup_duplicates.py --remove   # Remove duplicates
    python cleanup_duplicates.py --check    # Just check for duplicates
"""

import argparse
import os
import sys
from typing import Dict, List, Any, Tuple
from collections import defaultdict
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

class DuplicateCleaner:
    """Handles duplicate detection and removal"""
    
    def __init__(self, supabase_client: Client):
        self.supabase = supabase_client
        self.duplicates = {}
    
    def find_hero_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate heroes based on ID"""
        try:
            # Get all heroes
            result = self.supabase.table('heroes').select('*').execute()
            heroes = result.data or []
            
            # Group by ID
            hero_groups = defaultdict(list)
            for hero in heroes:
                hero_groups[hero['id']].append(hero)
            
            # Find duplicates
            duplicates = []
            for hero_id, hero_list in hero_groups.items():
                if len(hero_list) > 1:
                    duplicates.append({
                        'type': 'hero',
                        'id': hero_id,
                        'count': len(hero_list),
                        'records': hero_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding hero duplicates: {str(e)}")
            return []
    
    def find_build_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate builds based on hero_id and mood"""
        try:
            # Get all builds
            result = self.supabase.table('builds').select('*').execute()
            builds = result.data or []
            
            # Group by hero_id and mood
            build_groups = defaultdict(list)
            for build in builds:
                key = f"{build['hero_id']}_{build['mood']}"
                build_groups[key].append(build)
            
            # Find duplicates
            duplicates = []
            for key, build_list in build_groups.items():
                if len(build_list) > 1:
                    hero_id, mood = key.split('_', 1)
                    duplicates.append({
                        'type': 'build',
                        'hero_id': hero_id,
                        'mood': mood,
                        'count': len(build_list),
                        'records': build_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding build duplicates: {str(e)}")
            return []
    
    def find_hero_mood_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate hero moods"""
        try:
            # Get all hero moods
            result = self.supabase.table('hero_moods').select('*').execute()
            moods = result.data or []
            
            # Group by hero_id and mood
            mood_groups = defaultdict(list)
            for mood in moods:
                key = f"{mood['hero_id']}_{mood['mood']}"
                mood_groups[key].append(mood)
            
            # Find duplicates
            duplicates = []
            for key, mood_list in mood_groups.items():
                if len(mood_list) > 1:
                    hero_id, mood_name = key.split('_', 1)
                    duplicates.append({
                        'type': 'hero_mood',
                        'hero_id': hero_id,
                        'mood': mood_name,
                        'count': len(mood_list),
                        'records': mood_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding hero mood duplicates: {str(e)}")
            return []
    
    def find_hero_strength_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate hero strengths"""
        try:
            # Get all hero strengths
            result = self.supabase.table('hero_strengths').select('*').execute()
            strengths = result.data or []
            
            # Group by hero_id and strength
            strength_groups = defaultdict(list)
            for strength in strengths:
                key = f"{strength['hero_id']}_{strength['strength']}"
                strength_groups[key].append(strength)
            
            # Find duplicates
            duplicates = []
            for key, strength_list in strength_groups.items():
                if len(strength_list) > 1:
                    hero_id, strength_text = key.split('_', 1)
                    duplicates.append({
                        'type': 'hero_strength',
                        'hero_id': hero_id,
                        'strength': strength_text,
                        'count': len(strength_list),
                        'records': strength_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding hero strength duplicates: {str(e)}")
            return []
    
    def find_hero_weakness_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate hero weaknesses"""
        try:
            # Get all hero weaknesses
            result = self.supabase.table('hero_weaknesses').select('*').execute()
            weaknesses = result.data or []
            
            # Group by hero_id and weakness
            weakness_groups = defaultdict(list)
            for weakness in weaknesses:
                key = f"{weakness['hero_id']}_{weakness['weakness']}"
                weakness_groups[key].append(weakness)
            
            # Find duplicates
            duplicates = []
            for key, weakness_list in weakness_groups.items():
                if len(weakness_list) > 1:
                    hero_id, weakness_text = key.split('_', 1)
                    duplicates.append({
                        'type': 'hero_weakness',
                        'hero_id': hero_id,
                        'weakness': weakness_text,
                        'count': len(weakness_list),
                        'records': weakness_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding hero weakness duplicates: {str(e)}")
            return []
    
    def find_item_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate items based on build_id and name"""
        try:
            # Get all items
            result = self.supabase.table('items').select('*').execute()
            items = result.data or []
            
            # Group by build_id and name
            item_groups = defaultdict(list)
            for item in items:
                key = f"{item['build_id']}_{item['name']}"
                item_groups[key].append(item)
            
            # Find duplicates
            duplicates = []
            for key, item_list in item_groups.items():
                if len(item_list) > 1:
                    build_id, name = key.split('_', 1)
                    duplicates.append({
                        'type': 'item',
                        'build_id': int(build_id),
                        'name': name,
                        'count': len(item_list),
                        'records': item_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding item duplicates: {str(e)}")
            return []

    def find_playstyle_do_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate playstyle dos"""
        try:
            result = self.supabase.table('playstyle_dos').select('*').execute()
            dos = result.data or []
            
            groups = defaultdict(list)
            for do_item in dos:
                key = f"{do_item['build_id']}_{do_item['do_item']}"
                groups[key].append(do_item)
            
            duplicates = []
            for key, item_list in groups.items():
                if len(item_list) > 1:
                    build_id, text = key.split('_', 1)
                    duplicates.append({
                        'type': 'playstyle_do',
                        'build_id': int(build_id),
                        'text': text,
                        'count': len(item_list),
                        'records': item_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding playstyle_do duplicates: {str(e)}")
            return []

    def find_playstyle_dont_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate playstyle donts"""
        try:
            result = self.supabase.table('playstyle_donts').select('*').execute()
            donts = result.data or []
            
            groups = defaultdict(list)
            for dont_item in donts:
                key = f"{dont_item['build_id']}_{dont_item['dont_item']}"
                groups[key].append(dont_item)
            
            duplicates = []
            for key, item_list in groups.items():
                if len(item_list) > 1:
                    build_id, text = key.split('_', 1)
                    duplicates.append({
                        'type': 'playstyle_dont',
                        'build_id': int(build_id),
                        'text': text,
                        'count': len(item_list),
                        'records': item_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding playstyle_dont duplicates: {str(e)}")
            return []

    def find_playstyle_tip_duplicates(self) -> List[Dict[str, Any]]:
        """Find duplicate playstyle tips"""
        try:
            result = self.supabase.table('playstyle_tips').select('*').execute()
            tips = result.data or []
            
            groups = defaultdict(list)
            for tip_item in tips:
                key = f"{tip_item['build_id']}_{tip_item['tip']}"
                groups[key].append(tip_item)
            
            duplicates = []
            for key, item_list in groups.items():
                if len(item_list) > 1:
                    build_id, text = key.split('_', 1)
                    duplicates.append({
                        'type': 'playstyle_tip',
                        'build_id': int(build_id),
                        'text': text,
                        'count': len(item_list),
                        'records': item_list
                    })
            
            return duplicates
        except Exception as e:
            print(f"❌ Error finding playstyle_tip duplicates: {str(e)}")
            return []
    
    def find_all_duplicates(self) -> Dict[str, List[Dict[str, Any]]]:
        """Find all duplicates across all tables"""
        print("🔍 Searching for duplicates...")
        
        duplicates = {
            'heroes': self.find_hero_duplicates(),
            'builds': self.find_build_duplicates(),
            'hero_moods': self.find_hero_mood_duplicates(),
            'hero_strengths': self.find_hero_strength_duplicates(),
            'hero_weaknesses': self.find_hero_weakness_duplicates(),
            'items': self.find_item_duplicates(),
            'playstyle_dos': self.find_playstyle_do_duplicates(),
            'playstyle_donts': self.find_playstyle_dont_duplicates(),
            'playstyle_tips': self.find_playstyle_tip_duplicates()
        }
        
        return duplicates
    
    def print_duplicates(self, duplicates: Dict[str, List[Dict[str, Any]]]):
        """Print duplicate information"""
        total_duplicates = 0
        
        for table_name, table_duplicates in duplicates.items():
            if table_duplicates:
                print(f"\n📋 {table_name.upper()} DUPLICATES:")
                print("-" * 50)
                
                for dup in table_duplicates:
                    if dup['type'] == 'hero':
                        print(f"🦸 Hero ID: {dup['id']} - {dup['count']} duplicates")
                    elif dup['type'] == 'build':
                        print(f"🔨 Build: {dup['hero_id']} ({dup['mood']}) - {dup['count']} duplicates")
                    elif dup['type'] == 'hero_mood':
                        print(f"😊 Mood: {dup['hero_id']} - {dup['mood']} - {dup['count']} duplicates")
                    elif dup['type'] == 'hero_strength':
                        print(f"💪 Strength: {dup['hero_id']} - {dup['strength'][:30]}... - {dup['count']} duplicates")
                    elif dup['type'] == 'hero_weakness':
                        print(f"💔 Weakness: {dup['hero_id']} - {dup['weakness'][:30]}... - {dup['count']} duplicates")
                
                total_duplicates += len(table_duplicates)
            else:
                print(f"✅ No duplicates found in {table_name}")
        
        print(f"\n📊 SUMMARY: Found {total_duplicates} duplicate groups")
        return total_duplicates
    
    def remove_hero_duplicates(self, duplicates: List[Dict[str, Any]]) -> int:
        """Remove hero duplicates, keeping the first one"""
        removed_count = 0
        
        for dup in duplicates:
            # Keep the first record, remove the rest
            records_to_remove = dup['records'][1:]  # Skip first record
            
            for record in records_to_remove:
                try:
                    # Remove related data first
                    self.supabase.table('hero_weaknesses').delete().eq('hero_id', record['id']).execute()
                    self.supabase.table('hero_strengths').delete().eq('hero_id', record['id']).execute()
                    self.supabase.table('hero_moods').delete().eq('hero_id', record['id']).execute()
                    
                    # Remove the hero
                    self.supabase.table('heroes').delete().eq('id', record['id']).execute()
                    removed_count += 1
                    print(f"🗑️  Removed duplicate hero: {record['id']}")
                except Exception as e:
                    print(f"❌ Error removing hero {record['id']}: {str(e)}")
        
        return removed_count
    
    def remove_build_duplicates(self, duplicates: List[Dict[str, Any]]) -> int:
        """Remove build duplicates, keeping the first one"""
        removed_count = 0
        
        for dup in duplicates:
            # Keep the first record, remove the rest
            records_to_remove = dup['records'][1:]  # Skip first record
            
            for record in records_to_remove:
                try:
                    # Remove related data first
                    self.supabase.table('playstyle_tips').delete().eq('build_id', record['id']).execute()
                    self.supabase.table('playstyle_donts').delete().eq('build_id', record['id']).execute()
                    self.supabase.table('playstyle_dos').delete().eq('build_id', record['id']).execute()
                    self.supabase.table('items').delete().eq('build_id', record['id']).execute()
                    
                    # Remove the build
                    self.supabase.table('builds').delete().eq('id', record['id']).execute()
                    removed_count += 1
                    print(f"🗑️  Removed duplicate build: {record['hero_id']} ({record['mood']})")
                except Exception as e:
                    print(f"❌ Error removing build {record['id']}: {str(e)}")
        
        return removed_count
    
    def remove_hero_mood_duplicates(self, duplicates: List[Dict[str, Any]]) -> int:
        """Remove hero mood duplicates"""
        removed_count = 0
        
        for dup in duplicates:
            # Keep the first record, remove the rest
            records_to_remove = dup['records'][1:]  # Skip first record
            
            for record in records_to_remove:
                try:
                    self.supabase.table('hero_moods').delete().eq('hero_id', record['hero_id']).eq('mood', record['mood']).execute()
                    removed_count += 1
                    print(f"🗑️  Removed duplicate mood: {record['hero_id']} - {record['mood']}")
                except Exception as e:
                    print(f"❌ Error removing mood: {str(e)}")
        
        return removed_count
    
    def remove_hero_strength_duplicates(self, duplicates: List[Dict[str, Any]]) -> int:
        """Remove hero strength duplicates"""
        removed_count = 0
        
        for dup in duplicates:
            # Keep the first record, remove the rest
            records_to_remove = dup['records'][1:]  # Skip first record
            
            for record in records_to_remove:
                try:
                    self.supabase.table('hero_strengths').delete().eq('id', record['id']).execute()
                    removed_count += 1
                    print(f"🗑️  Removed duplicate strength: {record['hero_id']} - {record['strength'][:30]}...")
                except Exception as e:
                    print(f"❌ Error removing strength: {str(e)}")
        
        return removed_count
    
    def remove_hero_weakness_duplicates(self, duplicates: List[Dict[str, Any]]) -> int:
        """Remove hero weakness duplicates"""
        removed_count = 0
        
        for dup in duplicates:
            # Keep the first record, remove the rest
            records_to_remove = dup['records'][1:]  # Skip first record
            
            for record in records_to_remove:
                try:
                    self.supabase.table('hero_weaknesses').delete().eq('id', record['id']).execute()
                    removed_count += 1
                    print(f"🗑️  Removed duplicate weakness: {record['hero_id']} - {record['weakness'][:30]}...")
                except Exception as e:
                    print(f"❌ Error removing weakness: {str(e)}")
        
        return removed_count
    
    def remove_all_duplicates(self, duplicates: Dict[str, List[Dict[str, Any]]]) -> int:
        """Remove all duplicates"""
        total_removed = 0
        
        print("\n🗑️  Removing duplicates...")
        
        # Remove in order to avoid foreign key constraint issues
        total_removed += self.remove_hero_strength_duplicates(duplicates['hero_strengths'])
        total_removed += self.remove_hero_weakness_duplicates(duplicates['hero_weaknesses'])
        total_removed += self.remove_hero_mood_duplicates(duplicates['hero_moods'])
        total_removed += self.remove_build_duplicates(duplicates['builds'])
        total_removed += self.remove_hero_duplicates(duplicates['heroes'])
        
        return total_removed

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description='Clean up duplicate entries in Dota 2 database',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s --check        # Check for duplicates
  %(prog)s --dry-run      # Show what would be removed
  %(prog)s --remove       # Remove duplicates
        """
    )
    
    parser.add_argument('--check', action='store_true', help='Check for duplicates without removing')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be removed')
    parser.add_argument('--remove', action='store_true', help='Remove duplicates')
    
    args = parser.parse_args()
    
    if not any([args.check, args.dry_run, args.remove]):
        parser.print_help()
        return
    
    # Initialize cleaner
    cleaner = DuplicateCleaner(supabase)
    
    # Find all duplicates
    duplicates = cleaner.find_all_duplicates()
    
    # Print duplicates
    total_duplicates = cleaner.print_duplicates(duplicates)
    
    if total_duplicates == 0:
        print("\n✅ No duplicates found! Database is clean.")
        return
    
    if args.check:
        print(f"\n📊 Found {total_duplicates} duplicate groups")
        return
    
    if args.dry_run:
        print(f"\n🔍 DRY RUN: Would remove {total_duplicates} duplicate groups")
        print("Run with --remove to actually remove duplicates")
        return
    
    if args.remove:
        print(f"\n⚠️  WARNING: About to remove {total_duplicates} duplicate groups")
        confirm = input("Are you sure? Type 'yes' to continue: ")
        
        if confirm.lower() != 'yes':
            print("❌ Operation cancelled")
            return
        
        # Remove duplicates
        removed_count = cleaner.remove_all_duplicates(duplicates)
        print(f"\n✅ Successfully removed {removed_count} duplicate records")

if __name__ == '__main__':
    main() 