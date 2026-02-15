#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <deb-directory> <output-directory>" >&2
  exit 1
fi

if ! command -v dpkg-scanpackages >/dev/null 2>&1; then
  echo "dpkg-scanpackages is required (install dpkg-dev)." >&2
  exit 1
fi

deb_dir="$1"
output_dir="$2"

if [ ! -d "$deb_dir" ]; then
  echo "Input directory not found: $deb_dir" >&2
  exit 1
fi

mkdir -p "$output_dir"
cp "$deb_dir"/*.deb "$output_dir"/

cd "$output_dir"
dpkg-scanpackages . /dev/null | gzip -9c > Packages.gz

cat <<EOF > Release
Origin: pmon
Label: pmon
Suite: stable
Codename: stable
Architectures: amd64
Components: main
Description: pmon apt repository
EOF

echo "Apt repository created at $output_dir"
