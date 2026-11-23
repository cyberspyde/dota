# convert_hero_ids.py
import json

def convert_hero_ids_format(input_filename="build_data_final.json", output_filename="build_data_final_v2.json"):
    """
    Reads hero data and converts the 'heroId' field from snake_case
    (e.g., 'death_prophet') to kebab-case (e.g., 'death-prophet').

    Args:
        input_filename (str): The name of the source JSON file.
        output_filename (str): The name for the final, updated JSON file.
    """
    try:
        # Step 1: Read the JSON data with the updated moods
        with open(input_filename, 'r', encoding='utf-8') as f:
            print(f"Reading hero data from '{input_filename}'...")
            hero_list = json.load(f)

        modified_ids_report = []

        # Step 2: Iterate through each hero and modify the 'heroId'
        for hero in hero_list:
            # Use .get() to safely access the key in case it's missing
            original_id = hero.get("heroId")

            # Proceed only if the ID exists and contains an underscore
            if original_id and "_" in original_id:
                new_id = original_id.replace("_", "-")
                hero["heroId"] = new_id
                # Add a record of the change for our report
                modified_ids_report.append(f"'{original_id}' -> '{new_id}'")

        # Step 3: Write the fully modified list to a new file
        with open(output_filename, 'w', encoding='utf-8') as f:
            json.dump(hero_list, f, indent=2)

        print("-" * 30)
        print(f"Successfully processed {len(hero_list)} heroes.")
        if modified_ids_report:
            print(f"Converted {len(modified_ids_report)} hero IDs to kebab-case.")
            # Optional: uncomment the following lines to see every single change
            # for change in modified_ids_report:
            #     print(f"  - {change}")
        else:
            print("No hero IDs required conversion.")
        
        print(f"Final data written to '{output_filename}'.")

    except FileNotFoundError:
        print(f"Error: Input file '{input_filename}' not found. Please run the previous scripts first.")
    except json.JSONDecodeError:
        print(f"Error: Could not decode JSON from '{input_filename}'. The file might be corrupted.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

if __name__ == "__main__":
    convert_hero_ids_format()