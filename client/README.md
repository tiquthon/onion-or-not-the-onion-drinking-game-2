# Onion Or Not The Onion Drinking Game 2: Client

[https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/client](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/client)

By Thimo "Tiquthon" Neumann 2023

"\[...\] Client" is the client part of the "Onion Or Not The Onion Drinking Game 2".
For further information see [../README.md](../README.md).

You are free to copy, modify, distribute, but not sell "Onion Or Not The Onion Drinking Game 2: Client" with attribution under the terms of the GNU General Public License Version 3 with the "Commons Clause" License Condition v1.0.
See the `LICENSE` file at the repository root for details: [../LICENSE](../LICENSE).

## Setup And Use Project

Before setting up and using "\[...\] Client" you need:
- [Rust](https://www.rust-lang.org/)
- [Trunk](https://trunkrs.dev/) *(`cargo install --locked trunk`)*

In order to set up and use "\[...\] Client" **for development purposes**:
1. **Git Clone** or **Download** the repository
2. **Serve** this project by executing `trunk serve --open --port 8081 --features render` within this directory

In order to build "\[...\] Client" for distribution **with** the "\[...\] Server":
1. **Git Clone** or **Download** the repository
2. **Build** this project by executing `trunk build --release --features hydration` within this directory\
   and find the built project files in `./dist`
3. **Build** the "\[...\] Server" part, which uses the files in `./dist` during its compilation

In order to build "\[...\] Client" for distribution **without** the "\[...\] Server":
1. **Git Clone** or **Download** the repository
2. **Build** this project by executing `trunk build --release --features render` within this directory\
   and find the built project files in `./dist`
3. **Serve** it with any web server *(e.g.: nginx)*
   
The distinction between using it **with** or **without** the "\[...\] Server" is, whether the HTML structure gets initially rendered by the server *(client uses `--features hydration`)* or by the client *(client uses `--features render`)*.
If it gets wrongly distributed, the client application will just crash within the browser.

## Getting Help

*Please look inside the repository's README: [../README.md](../README.md)*

## Contributing

*Please look inside the repository's README: [../README.md](../README.md)*

---

README created with the help of [https://github.com/ddbeck/readme-checklist/checklist.md](https://github.com/ddbeck/readme-checklist/blob/b4e2d56fbb23d519a22b02af4fd513853d4ac1dd/checklist.md).
