# Onion Or Not The Onion Drinking Game 2: Server

[https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/server](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/server)

By Thimo "Tiquthon" Neumann 2023

"\[...\] Server" is the server part of the "Onion Or Not The Onion Drinking Game 2".
For further information see [../README.md](../README.md).

You are free to copy, modify, distribute, but not sell "Onion Or Not The Onion Drinking Game 2: Server" with attribution under the terms of the GNU General Public License Version 3 with the "Commons Clause" License Condition v1.0.
See the `LICENSE` file at the repository root for details: [../LICENSE](../LICENSE).

## Setup And Use Project

Before setting up and using "\[...\] Server" you need:
- [Rust](https://www.rust-lang.org/)

In order to set up and use "\[...\] Server":
1. **Git Clone** or **Download** the repository
2. **Build** the "\[...\] Client" as it is described in its [../client/README.md](../client/README.md) under **for distribution with the "\[...\] Server"**.
3. **Build** this project by executing `cargo build --release` within this directory\
   and find the executable in `./target/release/onion-or-not-the-onion-drinking-game-2-server`
4. **Execute** the built project with `cargo run --release` and visit it in the browser [http://localhost:8080/](http://localhost:8080/)

In order to use the production configuration, set the environment variable `APP_ENVIRONMENT` to `production`.
With this the server will use the files `./configuration/base.yml` and `./configuration/production.yml`.

## Getting Help

*Please look inside the repository's README: [../README.md](../README.md)*

## Contributing

*Please look inside the repository's README: [../README.md](../README.md)*

---

README created with the help of [https://github.com/ddbeck/readme-checklist/checklist.md](https://github.com/ddbeck/readme-checklist/blob/b4e2d56fbb23d519a22b02af4fd513853d4ac1dd/checklist.md).
