# update_moods.py
import json

# This mapping is the core logic. It's subjective and based on the hero's general playstyle.
# We map the old, descriptive moods to the new, standardized categories.
MOOD_MAP = {
    # Aggressive: Heroes that want to initiate, run at the enemy, and snowball.
    "greedy": "aggressive",
    "aggressive": "aggressive",
    "commanding": "aggressive",
    "frenzied": "aggressive",
    "unstoppable": "aggressive",
    "relentless": "aggressive",
    "dominant": "aggressive",
    "steadfast": "aggressive",
    "righteous": "aggressive",
    "destructive": "aggressive",
    "valorous": "aggressive",
    "voracious": "aggressive",
    "fiery": "aggressive",
    "radiant": "aggressive",
    "feral": "aggressive",
    "loyal": "aggressive",
    "imperious": "aggressive",
    "reckless": "aggressive",

    # Defensive: Heroes that protect, heal, save allies, and control fights from the back.
    "protective": "defensive",
    "supportive": "defensive",
    "benevolent": "defensive",
    "devout": "defensive",
    "chilling": "defensive",
    "illuminating": "defensive",
    "implacable": "defensive",

    # Creative: Heroes with a high skill ceiling, complex combos, or unique mechanics.
    "calculating": "creative",
    "artistic": "creative",
    "dynamic": "creative",
    "seismic": "creative",
    "primordial": "creative",
    "agile": "creative",
    "mysterious": "creative",
    "sly": "creative",
    "creative": "creative",
    "swashbuckling": "creative",
    "empowering": "creative",
    "synergistic": "creative", # Could be experimental, but fits creative due to micro
    "playful": "creative",
    "elusive": "creative", # Could be chaos, but skill expression is high
    "chaotic": "creative", # For Meepo, whose chaos comes from creative micro

    # Chaos: Heroes that cause disruption, thrive in messy fights, and sow confusion.
    "debilitating": "chaos",
    "chaotic": "chaos",
    "unruly": "chaos",
    "unpredictable": "chaos",
    "opportunistic": "chaos",
    "disruptive": "chaos",
    "mischievous": "chaos",
    "controlling": "chaos",
    "malevolent": "chaos",
    "versatile": "chaos",

    # Experimental: Heroes with very unique, strategic playstyles like global presence or split-pushing.
    "strategic": "experimental",
    "elusive": "experimental", # For Antimage, where elusiveness is a split-push strategy
    "overwhelming": "experimental",
    "symbiotic": "experimental",
    "whimsical": "experimental",
    "inevitable": "experimental", # Faceless Void's game is about one big ultimate
    "precise": "experimental", # Drow's game is about positioning strategy
}


def update_hero_moods(input_filename="build_data.json", output_filename="build_data_final.json"):
    """
    Reads a JSON file with a list of hero data and updates the 'mood'
    field based on a predefined mapping.

    Args:
        input_filename (str): The name of the source JSON file (list format).
        output_filename (str): The name for the final, updated JSON file.
    """
    try:
        # Step 1: Read the reformatted JSON data (the list of heroes)
        with open(input_filename, 'r', encoding='utf-8') as f:
            print(f"Reading hero data from '{input_filename}'...")
            hero_list = json.load(f)

        updated_count = 0
        unmapped_moods = set()

        # Step 2: Iterate through each hero object in the list
        for hero in hero_list:
            old_mood = hero.get("mood")

            if old_mood in MOOD_MAP:
                # Update the mood using our mapping
                hero["mood"] = MOOD_MAP[old_mood]
                updated_count += 1
            else:
                # If a mood isn't in our map, we'll keep the old one and report it
                if old_mood:
                    unmapped_moods.add(old_mood)

        # Step 3: Write the modified list to a new file
        with open(output_filename, 'w', encoding='utf-8') as f:
            json.dump(hero_list, f, indent=2)

        print("-" * 30)
        print(f"Successfully processed {len(hero_list)} heroes.")
        print(f"Updated the 'mood' for {updated_count} heroes.")
        print(f"Final data written to '{output_filename}'.")

        if unmapped_moods:
            print("\nWarning: The following moods were not found in the MOOD_MAP and were not changed:")
            for mood in sorted(list(unmapped_moods)):
                print(f"- {mood}")
            print("You may want to add these to the MOOD_MAP in the script.")

    except FileNotFoundError:
        print(f"Error: Input file '{input_filename}' not found. Please run the first script to generate it.")
    except json.JSONDecodeError:
        print(f"Error: Could not decode JSON from '{input_filename}'. The file might be corrupted.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

if __name__ == "__main__":
    update_hero_moods()