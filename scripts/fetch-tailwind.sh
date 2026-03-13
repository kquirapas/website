#!/bin/bash
VERSION=${1:-latest}
FILE=${2:-tailwindcss-linux-x64}
echo "Fetching ${VERSION}:${FILE}..."
curl -L https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 -o ./bin/tailwindcss --create-dirs
chmod +x ./bin/tailwindcss
