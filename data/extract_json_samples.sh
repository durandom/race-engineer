#!/bin/bash

# Directory containing the source JSON files
SRC_DIR="rbr"
# Directory to store the sampled JSON files
DEST_DIR="rbr.samples"

# Create the destination directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Loop through all JSON files in the source directory
for file in "$SRC_DIR"/*.json; do
  filename=$(basename "$file")
  dest_file="$DEST_DIR/$filename"

  # Use jq to extract a sample: first 5 elements for arrays, first 5 key-value pairs for objects
  jq 'if type == "array" then .[:5] elif type == "object" then to_entries[:5] | from_entries else . end' "$file" > "$dest_file"
done

echo "Sampled JSON files have been written to $DEST_DIR"