#!/bin/bash
cd "$(dirname "$0")/.."

# cleanup error
rm -rf tmp src/protos

# download resources
git clone --recurse-submodules https://github.com/SteamRE/SteamKit tmp
mv -f tmp/Resources/Protobufs/steam src/protos
rm -rf tmp
