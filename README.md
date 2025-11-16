# Sleipnir

Graphite to Clickhouse Metrics Obfuscation written on Rust.

Listen on provided host:port in tcp mode and read data as a plain text
in graphite format. Obfuscate the data, and write it to clickhouse
database as batches (or each 5 sec).

---

## Configuration

You can configure application with next environment variables.

- `APP_CH_URL`: clickhouse url, e.g. http://clickhouse:8123, **required**

- `APP_CH_PASSWORD`: clickhouse password, **required**

- `APP_CH_USERNAME`: clickhouse username, defaults to `default`

- `APP_CH_DATABASE`: clickhouse database name, defaults to `sleipnir`

- `APP_CH_TABLE`: clickhouse table name, defaults to `metrics`

- `APP_FLUSH_INTERVAL`: data flush (to clickhouse) interval in seconds, defaults to `5`

- `APP_BATCH_SIZE`: data batch size for upload to clickhouse, defaults to `1000`

- `APP_HOST`: listen hostname, defaults to `127.0.0.1`

- `APP_PORT`: listen port, defaults to `8080`

- `APP_NUM_WORKERS`: number of workers (futures/routines) to spawn, defaults to `nproc` or `4`

- `APP_CHANNEL_BUFFER`: channels buffer size, defaults to `10000`

More over here you can find some Prometheus Client Settings

- `APP_APPLICATION`: `application` label for prometheus, defaults to `sleipnir`

- `APP_CIRCUIT`: `circuit` label for prometheus, defaults to `unknown`

- `APP_ENV`: `env` label for prometheus, defaults to `unknown`

- `APP_PROJECT`: `project` label for prometheus, defaults to `unknown`

- `APP_PROMETHEUS_HOST`: prometheus client host, defaults to `0.0.0.0`

- `APP_PROMETHEUS_PORT`: prometheus client port, defaults to `9090`

For more info please take look to [config mod](./src/libs/config/mod.rs).

Additionally, you can use built-in rust env variables, e.g.

- `RUST_LOG`: set log level, defaults to `info`.

If you would like to skip some tests, you can use next statement:

```shell
RUSTFLAGS="-A dead_code" pre-commit
```

I could be helpful when you're adding a library...

---

## Build

For build dynamic linked binary run:

```shell
cargo build --release
```

### Build Static

Install musl library for build binaries static.

```shell
rustup target add x86_64-unknown-linux-musl
```

Also, you have to install deps

```shell
sudo apt-get install libssl-dev pkg-config musl-tools musl-dev
```

Build release binaries static.

```shell
export OPENSSL_STATIC=1
export OPENSSL_DIR=/usr

cargo clean
cargo build --release --target x86_64-unknown-linux-musl
```

### Build RPM Package

Install `generate-rpm` from cargo...

```shell
cargo install cargo-generate-rpm
```

Generate rpm package...

```shell
cargo generate-rpm --target x86_64-unknown-linux-musl
```

## Testing

You can test whole pipe-line with [testing environment](./tests/README.md)
