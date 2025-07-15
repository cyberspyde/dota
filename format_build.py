# convert_format.py
import json

def reformat_build_data(input_filename="build_data.json", output_filename="scripts/build_template.json"):
    """
    Reads a JSON file where heroes are in a top-level dictionary
    and converts it to a JSON file with a top-level list of hero objects.

    Args:
        input_filename (str): The name of the source JSON file.
        output_filename (str): The name of the destination JSON file.
    """
    try:
        # Step 1: Open and read the original JSON file
        with open(input_filename, 'r', encoding='utf-8') as f:
            print(f"Reading data from '{input_filename}'...")
            original_data = json.load(f)
        
        # Step 2: The original data is a dictionary like {"abaddon": {...}, "alchemist": {...}}
        # We want a list of the values: [{...}, {...}]
        # The .values() method gives us all the hero objects. We just need to put them in a list.
        reformatted_data = list(original_data.values())

        # Step 3: Open the output file and write the new list as JSON
        with open(output_filename, 'w', encoding='utf-8') as f:
            # Use indent=2 for pretty-printing, making the file readable
            json.dump(reformatted_data, f, indent=2)
        
        print(f"Successfully converted {len(reformatted_data)} hero builds.")
        print(f"New file created: '{output_filename}'")

    except FileNotFoundError:
        print(f"Error: Input file '{input_filename}' not found. Make sure it's in the same directory as the script.")
    except json.JSONDecodeError:
        print(f"Error: Could not decode JSON from '{input_filename}'. Please check if the file is a valid JSON.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")


if __name__ == "__main__":
    reformat_build_data()