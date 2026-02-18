#!/usr/bin/env bash

# Install dependencies for ubuntu systems.
# Other linux distros can be added here if necessary.
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf
