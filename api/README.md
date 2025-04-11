# API

## Configuration
Configs are located in the `configs/`.

As configuration example there are file `dev.template.toml` and `prod.template.toml`.\
**You need to create** a `dev.toml` and `prod.toml` files, and fill it in as specified in the template with own values.

If you run project by `docker-compose.yaml`, you also need to specify environment variables. An configuration example is `.env.template`, you need to create `.env` file and fill it in as specified in the template with own values.

## Deployment
**For testing and development purposes**, you probably would like to run the database and API separately to avoid a long project building and database startup in a docker environment, so by default, `dev.toml` uses database and API hosts as `127.0.0.1`, but if you want to deploy database and API in a docker environment, probably you need to specify container name instead of database host and `0.0.0.0` as API host, that done in `prod.toml`.

If you run project by `docker-compose.yaml`, `prod.toml` used by default, to change this specify `CONFIG_PATH` environment variable.
If you run the project outside of docker, you need to specify `CONFIG_PATH` environment variable, but it's done by scripts in `justfile`: `run-dev` and `run-prod`.
