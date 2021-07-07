![realm](https://github.com/honwen/realm/workflows/release/badge.svg)

## Introduction

realm is a simple, high performance relay server written in rust.

## Features

- Zero configuration. Setup and run in one command.
- Concurrency. Bidirectional concurrent traffic leads to high performance.
- Low resources cost.

## Usage

This executable takes 1 arguments:

- -L [--listen] listen config, can be configured multi times. scheme://[listening_address]:listening_port/[remote_address]:remote_port

An example to listen on port 30000 and forwarding traffic to example.com:12345 is as follows.

```bash
./realm -L=127.0.0.1:30000/example.com:12345
```

An example to listen on port 30000/_tcp-only_ and forwarding traffic/_tcp_ to example.com:12345 is as follows.

```bash
./realm -L=tcp://127.0.0.1:30000/example.com:12345
```

An example to listen on port 30000 and forwarding traffic/_tcp_ to example.com:12345 is as follows, forwarding traffic/_udp_ to example.com:23456.

```bash
./realm -L=tcp://127.0.0.1:30000/example.com:12345 -L=udp://127.0.0.1:30000/example.com:23456
```

An example to listen on port 30000 and forwarding traffic to example.com:12345 is as follows, to listen on port 40000 and forwarding traffic to example.com:23456 is as follows at the same time.

```bash
./realm -L=127.0.0.1:30000/example.com:12345 -L=127.0.0.1:40000/example.com:23456
```

## Docker

- https://hub.docker.com/r/chenhw2/realm

```bash
docker run -p 6666:6666 -p 6666:6666/udp -e ARGS='-L=:6666/8.8.8.8:53' chenhw2/realm
```
