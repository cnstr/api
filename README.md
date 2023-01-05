# Canister API
This is the API frontend of Canister (version 2).<br>
It works by completing the following tasks in its pipeline:
* Generate and publish the OpenAPI reference
* Generate the database client through Prisma
* Advertise API routes for functionality

### Development
This project utilizes [`task`](https://taskfile.dev) and `docker compose` for the databases relied on by the API.<br>
In order to populate the databases, [`cnstr/core`](https://github.com/cnstr/core) needs to be setup and run once.<br>
The project also requires that you have a Rust toolchain installed and run `cargo install cargo-watch`.<br>
Once you have setup everything, simply running `task dev` will start the API with hot-reloading and the databases.

### Deployment
The project is deployed as a Deployment on Kubernetes.<br>
Ensure that you have `cargo-release` installed and `kubectl` installed and configured.<br>
The `task deploy` command will automatically configure the following:
* Build and publish the Docker image to the [tale.me](https://tale.me/docker) registry
* Distribute and upload the OpenAPI reference to [bump.sh](https://bump.sh)
* Rewrite the `kubernetes/deployment.yaml` file with the new image tag
* Apply the new deployment to the cluster

> Copyright (c) 2023, Aarnav Tale
