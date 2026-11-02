# LORSHED Sentinel (Rust)

A local, read-only **file integrity checker** for server configuration and project directories. It creates a SHA-256 baseline and later reports added, removed, and modified files.

## Build

Install Rust from [rustup.rs](https://rustup.rs/), then:

```bash
cd rust-sentinel
cargo build --release
```

## Use

Start with a small directory you control:

```bash
mkdir -p /tmp/sentinel-demo
echo hello > /tmp/sentinel-demo/config.txt
cargo run -- init /tmp/sentinel-demo ./demo-baseline.json
cargo run -- check /tmp/sentinel-demo ./demo-baseline.json
echo changed > /tmp/sentinel-demo/config.txt
cargo run -- check /tmp/sentinel-demo ./demo-baseline.json
```

Exit codes: **0** = unchanged / baseline created, **1** = differences, **2** = invalid input or error. This makes it suitable for cron, CI, or monitoring scripts.

## Important limitations

- Baselines must be created on a **known-good** system.
- **Keep the baseline outside the monitored directory and protect it**: an attacker able to rewrite both files and the baseline can conceal tampering.
- Files are hashed by content only; permissions, owners, metadata and symlink targets are **not** monitored. Symlinks are not followed.
- The scan is not atomic: files can change while being scanned.
- This tool does **not** block intrusions or replace backups, endpoint protection, audit logs, or professional security monitoring.
- No telemetry or external upload is implemented.
- On Unix, ensure only trusted administrators can modify the baseline.

## Next steps

Possible additions: permission monitoring, signed baselines, watch mode, secure alerting, and incremental scanning.

MIT licensed. See the repository root LICENSE.
