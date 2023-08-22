# SteamKit VPK

This crate is a simple library to help parse and read files based on the [VPK file format](https://developer.valvesoftware.com/wiki/VPK_(file_format)).

# To-do

- [ ] Remove the CS:GO & Portal 2 dir assets (even though they mostly only describe the rest of the pack).
- [ ] Add some generated/custom packs for testing reading and value checking.
- [ ] Add a test for reading data from preload.
- [ ] Add a test for reading from data section.
- [ ] Add a test for reading from a separate archive-part.
- [x] Optimize the tree parser (with capacity hints).
