# Sleipnir Testing Environment

---

## Build Sleipnir Testing Image

At first you have to build a [static binary](../README.md#build-static).

Next run docker compose command:

```shell
docker compose build
```

---

## Metrics Names Generator

For run this testing environment you have to generate a dataset of
uniq metrics names (json file), for this purpose you can use
`generate-metrics-names.py` python script.

Just run:

```shell
python 3.12 generate-metrics-names.py
```

Please note that generation process could take a while.

---

## Metrics Generator

You can find a metrics generator into `./py/generator.py` file.
It runs in to `generator` service with a several deployment replicas.

### Metrics Generator Configuration

You can configure some params via environment values.

- `APP_HOST`: hostname for sleipnir service (e.g. `sleipnir`)

- `APP_PORT`: port where sleipnir service listening on

---

## Clickhouse Stack

This testing environment provides Clickhouse stack with next components:

- `clickhouse`: database server

    - use http://127.0.0.1:8123 to connect via web-interface
    - use tcp://127.0.0.1:9000 to connect via clickhouse-interface

- `cliclhouse-init`: database initialization script

- `prometheus`: prometheus server

- `grafana`: grafana server with built-in clickhouse dashboard

    - use http://127.0.0.1:3000 to connect to grafana server

- `tabix`: clickhouse web-ui

    - use http://127.0.0.1:80 to connect to tabix console

Please note that we are using resource limiting for deployments for prevent
resource leaking.
