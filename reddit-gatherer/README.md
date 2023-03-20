# Onion Or Not The Onion Drinking Game 2: Reddit Gatherer

[https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/reddit-gatherer](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/reddit-gatherer)

By Thimo "Tiquthon" Neumann 2023

"\[...\] Reddit Gatherer" is an application to gather the necessary game data for "Onion Or Not The Onion Drinking Game 2".
For further information see [../README.md](../README.md).

You are free to copy, modify, distribute, but not sell "Onion Or Not The Onion Drinking Game 2: Reddit Gatherer" with attribution under the terms of the GNU General Public License Version 3 with the "Commons Clause" License Condition v1.0.
See the `LICENSE` file at the repository root for details: [../LICENSE](../LICENSE).

## Setup And Use Project

Before setting up and using "\[...\] Reddit Gatherer" you need:
- [Rust](https://www.rust-lang.org/)

In order to set up and use "\[...\] Reddit Gatherer":
1. **Git Clone** or **Download** the repository
2. **Build** this project by executing `cargo build --release` within the root of this project\
   and find the executable in `./target/release/onion-or-not-the-onion-drinking-game-2-reddit-gatherer`
3. **Execute** the built project with `cargo run --release`\
   and see the options with `cargo run --release -- --help`

An example command to download game data is:
`cargo run --release -- --subreddit-name theonion --feed-type best --count 1000 --output theonion.best.1000.ron`
or with the direct executable:
`./onion-or-not-the-onion-drinking-game-2-reddit-gatherer --subreddit-name theonion --feed-type best --count 1000 --output theonion.best.1000.ron`

## Getting Help

*Please look inside the repository's README: [../README.md](../README.md)*

## Contributing

*Please look inside the repository's README: [../README.md](../README.md)*

---

README created with the help of [https://github.com/ddbeck/readme-checklist/checklist.md](https://github.com/ddbeck/readme-checklist/blob/b4e2d56fbb23d519a22b02af4fd513853d4ac1dd/checklist.md).
