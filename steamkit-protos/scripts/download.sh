#!/bin/bash
cd "$(dirname "$0")/.."

# cleanup error
rm -rf tmp src/protos src/resources

# download resources
git clone --recurse-submodules https://github.com/SteamRE/SteamKit tmp 
mkdir src/resources
mv -f tmp/Resources/SteamLanguage/*.steamd src/resources
mv -f tmp/Resources/Protobufs/steam src/protos
rm -rf tmp

