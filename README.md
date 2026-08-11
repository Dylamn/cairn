# Cairn

Cairn is a command-line tool for working with GPX traces. The first version
focuses on **trimming** tracks: given an existing GPX file, select two points
and produce a new, non-destructive GPX file containing only the portion
between them.

> Named after trail cairns — the stacked stone markers used to mark a path.
> The project is intended to grow into a small toolkit for cleaning up and
> manipulating GPX tracks, not just trimming.

## Status

Early / learning project. Expect breaking changes. This is also a personal
exercise in learning Rust, so the code favors clarity over cleverness.

## Features

### Available

- **Trim** — cut a GPX track down to the segment between two selected
  points, writing the result to a new file (the original is never modified).

### Planned — If time permits

- Split a track into multiple files at one or more points
- Merge multiple GPX files into one
- Reverse point order
- Simplify/downsample a track (Ramer–Douglas–Peucker)
- Track stats (distance, elevation gain/loss, duration, avg speed)
- Elevation profile export
- Remove duplicate / stationary points
- Nearest-point lookup by coordinate (foundation for a future GUI)
- A graphical version of the tool, built on the same core logic

> N.B.: Thanks to AI for the ideas!

## Installation

Cairn is built with [Cargo](https://doc.rust-lang.org/cargo/), the Rust
package manager. You'll need a recent stable Rust toolchain
([rustup.rs](https://rustup.rs) is the easiest way to get one).

```bash
git clone https://github.com/Dylamn/cairn.git
cd cairn
cargo build --release
```

The compiled binary will be available at `target/release/cairn`.

## Usage

```bash
cairn trim <input.gpx> --from <selector> --to <selector> --output <output.gpx>
```

`<selector>` currently refers to a point index in the track. Additional
selector types (by timestamp, by distance along the track, by nearest
coordinate) are planned.

### Example

```bash
cairn trim hike.gpx --from 120 --to 980 --output hike-trimmed.gpx
```

This keeps only the points between index `120` and index `980` (inclusive)
from `hike.gpx` and writes the result to `hike-trimmed.gpx`, leaving the
original file untouched.

## Project layout

```
cairn/
├── Cargo.toml
└── src/
    ├── lib.rs         — Exposes the core logic for the entrypoint
    ├── main.rs        — CLI entry point and argument parsing
    ├── cli.rs         — Command-line argument parsing
    ├── gpx_io.rs      — Loading and saving GPX files
    ├── trim.rs        — Core trimming logic (pure, no I/O)
    └── selection.rs   — Resolving a "selector" (index, time, distance, ...) to a point
```

The core logic is kept separate from the CLI so it can be reused by a future
graphical interface without rewriting the underlying operations.

## Built with

- [`gpx`](https://crates.io/crates/gpx) — GPX parsing and writing
- [`geo`](https://crates.io/crates/geo) / [`geo-types`](https://crates.io/crates/geo-types) — geographic types and calculations
- [`clap`](https://crates.io/crates/clap) — command-line argument parsing

## License

MIT (or whatever you prefer — this is currently a personal learning project).
