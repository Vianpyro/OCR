#!/bin/sh
# Download MNIST dataset files into a folder (default: /data/mnist)

# Use the first argument as folder, default to /data/mnist
DEST_DIR="${1:-/data/mnist}"

BASE_URL="https://raw.githubusercontent.com/fgnt/mnist/master"

FILES="
train-images-idx3-ubyte.gz
train-labels-idx1-ubyte.gz
t10k-images-idx3-ubyte.gz
t10k-labels-idx1-ubyte.gz
"

# Create the destination folder if it doesn't exist
mkdir -p /workspaces/OCR/data/mnist
cd /workspaces/OCR/data/mnist || exit

# Download each file if it doesn't exist
for FILE in $FILES; do
    if [ ! -f "$FILE" ]; then
        echo "Downloading $FILE..."
        wget "$BASE_URL/$FILE"
    else
        echo "$FILE already exists, skipping."
    fi
done

# Unzip the .gz files
echo "Unzipping files..."
gunzip -k *.gz

# Remove the .gz files
rm -f *.gz

echo "Done! MNIST dataset is in $DEST_DIR"
