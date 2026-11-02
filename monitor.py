#!/usr/bin/env python3
"""LORSHED HUDSON - lightweight cross-platform system monitor."""
import argparse
import json
import shutil
import time
from datetime import datetime, timezone

try:
    import psutil
except ImportError:
    psutil = None


def snapshot():
    disk = shutil.disk_usage("/")
    data = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "disk_total_gb": round(disk.total / 1024**3, 2),
        "disk_used_percent": round(disk.used / disk.total * 100, 1) if disk.total else 0,
    }
    if psutil:
        data["cpu_percent"] = psutil.cpu_percent(interval=0.3)
        data["ram_percent"] = psutil.virtual_memory().percent
    else:
        data["cpu_percent"] = None
        data["ram_percent"] = None
    return data


def main():
    parser = argparse.ArgumentParser(description="LORSHED HUDSON System Monitor")
    parser.add_argument("--interval", type=float, default=0, help="Repeat every N seconds; 0 runs once")
    parser.add_argument("--json", action="store_true", help="Output JSON")
    args = parser.parse_args()
    if args.interval < 0:
        parser.error("--interval must be zero or positive")
    try:
        while True:
            data = snapshot()
            if args.json:
                print(json.dumps(data), flush=True)
            else:
                print(f"[{data['timestamp']}] CPU: {data['cpu_percent']}% | RAM: {data['ram_percent']}% | Disk: {data['disk_used_percent']}% ({data['disk_total_gb']} GB total)", flush=True)
            if args.interval == 0:
                break
            time.sleep(args.interval)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
