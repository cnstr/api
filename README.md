# Canister API

> The API for Canister (version 2)

![Build Status](https://img.shields.io/github/actions/workflow/status/cnstr/ci/api.yaml)
[![Codacy](https://img.shields.io/codacy/grade/d7ce92011567411a85f69098196e961e)](https://www.codacy.com?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=cnstr/api&amp;utm_campaign=Badge_Grade)
![Uptime](https://img.shields.io/website?down_message=offline&label=status&up_message=online&url=https%3A%2F%2Fapi.canister.me%2Fv2%2Fhealthz)
![Release](https://img.shields.io/github/v/tag/cnstr/api?label=release&cacheSeconds=3600)
![License](https://img.shields.io/github/license/cnstr/api?cacheSeconds=3600)

This project hosts the Canister API and accompanying services.<br>
It is written in Rust and utilizes various crates such as:

- [`tide`](https://github.com/http-rs/tide) for the web server
- [`surf`](https://github.com/http-rs/surf) for Typesense fetching
- [`prisma`](https://prisma.io) for the Postgres client (via [`prisma-client-rs`](https://github.com/Brendonovich/prisma-client-rust))

The API is deployed on Kubernetes and is accessible [here](https://api.canister.me/v2/).<br>
If you're interested in the API documentation, you can find it [here](https://docs.canister.me).

### Development

This project utilizes [`task`](https://taskfile.dev) and `docker compose` for a development environment.<br>
In order to populate the databases, [`cnstr/core`](https://github.com/cnstr/core) needs to be setup and run once.<br>
The project also requires the Rust toolchain installed and `cargo-watch` installed (`cargo install cargo-watch`).<br>
Once you have setup everything, running `task dev` will start the API with hot-reloading and the databases.

### Deployment

*This project isn't intended to be deployed by anyone other than the maintainers.*<br>
*When deployed, the project is relicensed under a proprietary license.*

The `task deploy` command will trigger the [CI/CD at `cnstr/ci`](https://github.com/cnstr/ci/actions/workflows/api.yaml).<br>
The deployment will run the following steps:

- Build and publish the Docker image to the [tale.me](https://tale.me/docker) registry
- Distribute and upload the OpenAPI reference to [bump.sh](https://bump.sh)
- Rewrite the `kubernetes/api.yaml` file with the new image tag
- Apply the new deployment to the cluster

> Copyright (c) 2023, Aarnav Tale
