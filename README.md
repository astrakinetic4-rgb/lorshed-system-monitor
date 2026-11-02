# LORSHED HUDSON — System Monitor

A lightweight Python CLI for checking CPU, RAM and disk usage on Linux, Windows or macOS.

## Setup

Requires Python 3.9+.

```bash
python -m pip install -r requirements.txt
python monitor.py
```

Continuous monitoring every 5 seconds:

```bash
python monitor.py --interval 5
```

Machine-readable output:

```bash
python monitor.py --json
```

CPU and RAM readings require `psutil`. Without it, the script still reports disk usage and marks CPU/RAM as unavailable.

## Security

Read-only monitoring. No API keys, remote uploads, or elevated privileges required.

## License

MIT
