# Onion Or Not The Onion Drinking Game 2

[![CI](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/actions/workflows/general.yml/badge.svg)](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/actions/workflows/general.yml)
[![Security Audit](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/actions/workflows/audit.yml/badge.svg)](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2/actions/workflows/audit.yml)

[https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2](https://github.com/tiquthon/onion-or-not-the-onion-drinking-game-2)

By Thimo "Tiquthon" Neumann 2023

"Onion Or Not The Onion Drinking Game 2" is a web base game which presents newspaper headlines from the satirical newspaper [the ONION](https://www.theonion.com/) and non-satirical ones.
The players have to guess if it's from the ONION or a non satirical newspaper and get points for being right.

You are free to copy, modify, distribute, but not sell "Onion Or Not The Onion Drinking Game 2" with attribution under the terms of the GNU General Public License Version 3 with the "Commons Clause" License Condition v1.0.
See the `LICENSE` file for details.

## Preview

https://user-images.githubusercontent.com/61014652/221667398-55690c2d-9979-4986-94e6-e08e81f6ec32.mp4

*[video inside repository](assets/preview.mp4)*

## Setup And Use Project

Before setting up and using "Onion Or Not The Onion Drinking Game 2" you need:
- [Docker](https://www.docker.com/) with [Docker Compose](https://docs.docker.com/compose/)

In order to set up and use "Onion Or Not The Onion Drinking Game 2":
1. **Git Clone** or **Download** the repository
2. **Execute** `docker compose up -d` within the root of this project\
   If this app is behind another reverse-proxy and has a path-prefix, the docker build arg `BUILD_URL_PATH_PREFIX` has to be changed from `/` like: `docker compose up --build-arg BUILD_URL_PATH_PREFIX=/onion2/ -d`
3. Access game at [http://localhost:6362/](http://localhost:6362/).

## Project Documentation

*Work In Progress*

## Getting Help

If you have any questions or need help regarding this project please open an issue within this GitHub project.

This is only a hobby project, so don't expect every fast responses from my side, please.

## Contributing

If you're interested in contributing, there are many ways to contribute to this project.
Get started in the file [CONTRIBUTING.md](CONTRIBUTING.md).

**Attention:** Whenever you contribute to this project, you agree to provide your work under the same license as this project is provided under.
See the `LICENSE` file.

### Working with the project

Working on the "\[...\] Reddit Gatherer" does not need any special configuration.
But working on the "\[...\] Client" and the "\[...\] Server" may need some more steps.

*Usually the "\[...\] Client" gets built before the "\[...\] Server" and gets baked into the "\[...\] Server".*
*So any code changes made to the "\[...\] Client" may only trigger no recompilation for the "\[...\] Server" and thus no changes get shown on checking in the browser.*
*For this a **development reverse proxy** is available, which reroutes requests to each software part during development.*

In order to use the setup for "\[...\] Client" and "\[...\] Server":
1. **Start** the development reverse proxy through docker: `docker compose -f docker-compose.dev.yml up -d`
2. **Start** the "\[...\] Client" as mentioned within [client/README.md](client/README.md) under the **for development purposes** part.
3. **Start** the "\[...\] Server" as described in [server/README.md](server/README.md)

The "\[...\] Server" still will bake the "\[...\] Client" into it, but the baked state won't be accessible, because the development reverse proxy will reroute any request to that URL to the served "\[...\] Client". 

---

README created with the help of [https://github.com/ddbeck/readme-checklist/checklist.md](https://github.com/ddbeck/readme-checklist/blob/b4e2d56fbb23d519a22b02af4fd513853d4ac1dd/checklist.md).

