#!/bin/bash
cd "$(dirname "$0")/.."

# cleanup error
rm -rf tmp src/protos src/resources

# download protobufs
git clone https://github.com/SteamDatabase/Protobufs tmp
mkdir src/protos
mv -f tmp/steam/* src/protos
rm -rf tmp

# download resources
git clone https://github.com/SteamRE/SteamKit tmp
mkdir src/resources
mv -f tmp/Resources/SteamLanguage/*.steamd src/resources
rm -rf tmp
