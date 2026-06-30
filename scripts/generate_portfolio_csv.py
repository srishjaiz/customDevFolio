#!/usr/bin/env python3
"""Generate large portfolio CSV datasets for multi-portfolio import testing.

Free stdlib only. Writes streaming row-by-row (does not hold all rows in memory
beyond the current line).

Examples:
  python3 scripts/generate_portfolio_csv.py -n 10000 -o examples/import/generated/large_10k.csv
  python3 scripts/generate_portfolio_csv.py -n 100000 --edge-every 50 -o /tmp/huge.csv
  python3 scripts/generate_portfolio_csv.py -n 500 --breaking 0.02 -o /tmp/mixed.csv
"""

from __future__ import annotations

import argparse
import csv
import random
import sys
from pathlib import Path

DOMAINS = [
    "frontend",
    "backend",
    "fullstack",
    "ml",
    "mobile",
    "devops",
    "data",
    "security",
    "game",
    "general",
]

THEMES = ["system", "light", "dark"]

HEADERS = [
    "slug",
    "domain",
    "name",
    "title",
    "bio",
    "location",
    "email",
    "github",
    "linkedin",
    "website",
    "resume_url",
    "primary",
    "theme",
]


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("-n", "--rows", type=int, default=10_000, help="Number of data rows")
    p.add_argument(
        "-o",
        "--output",
        type=Path,
        default=Path("examples/import/generated/large.csv"),
        help="Output CSV path",
    )
    p.add_argument("--seed", type=int, default=42, help="RNG seed for reproducibility")
    p.add_argument(
        "--edge-every",
        type=int,
        default=100,
        help="Insert a benign edge-case row every N rows (0=disable)",
    )
    p.add_argument(
        "--breaking",
        type=float,
        default=0.01,
        help="Fraction of rows that are intentionally invalid (0.0–1.0)",
    )
    p.add_argument(
        "--dup-name-every",
        type=int,
        default=200,
        help="Repeat the same display name every N rows (slug stress)",
    )
    return p.parse_args()


def hex_color(rng: random.Random) -> str:
    return f"#{rng.randint(0, 0xFFFFFF):06x}"


def normal_row(i: int, rng: random.Random, force_name: str | None = None) -> dict[str, str]:
    domain = DOMAINS[i % len(DOMAINS)]
    surnames = ["Lee", "Patel", "Nguyen", "Garcia", "Obrien", "Kim", "Silva"]
    name = force_name or f"Person {i:06d} {rng.choice(surnames)}"
    slug = f"person-{i:06d}" if force_name is None else ""
    return {
        "slug": slug,
        "domain": domain,
        "name": name,
        "title": f"{domain.title()} Engineer",
        "bio": f"Synthetic bio for row {i}. Focus: {domain}.",
        "location": rng.choice(["Remote", "NYC", "London", "Bengaluru", "São Paulo", ""]),
        "email": f"person{i:06d}@example.test",
        "github": rng.choice([f"user{i}", f"https://github.com/user{i}", ""]),
        "linkedin": rng.choice(["", f"https://linkedin.com/in/user{i}"]),
        "website": rng.choice(["", f"https://user{i}.example.test"]),
        "resume_url": rng.choice(["", f"https://user{i}.example.test/cv.pdf"]),
        "primary": hex_color(rng),
        "theme": rng.choice(THEMES),
    }


def edge_row(i: int, rng: random.Random) -> dict[str, str]:
    kind = i % 6
    base = normal_row(i, rng)
    if kind == 0:
        base["name"] = f"Unicode {i} 李雷 José"
        base["bio"] = "café — résumé — 東京"
    elif kind == 1:
        base["bio"] = f'Quoted "bio", with commas, for row {i}'
        base["title"] = 'Staff, "Principal"'
    elif kind == 2:
        base["bio"] = "A" * 2000
    elif kind == 3:
        base["github"] = "https://github.com/edgeuser"
        base["slug"] = ""
    elif kind == 4:
        base["domain"] = ""  # default fullstack
        base["slug"] = ""
    else:
        base["name"] = f"  Spaced {i}  "
    return base


def breaking_row(i: int, rng: random.Random) -> dict[str, str]:
    kind = rng.randint(0, 4)
    if kind == 0:
        return {h: "" for h in HEADERS} | {"domain": "frontend", "email": f"noname{i}@x.test"}
    if kind == 1:
        return normal_row(i, rng) | {"name": "", "email": f"empty{i}@x.test"}
    if kind == 2:
        return normal_row(i, rng) | {"domain": "not-a-real-domain"}
    if kind == 3:
        return normal_row(i, rng) | {"domain": "front-end"}
    return normal_row(i, rng) | {"theme": "rainbow"}


def main() -> int:
    args = parse_args()
    if args.rows < 0:
        print("rows must be >= 0", file=sys.stderr)
        return 2
    if not 0.0 <= args.breaking <= 1.0:
        print("--breaking must be between 0 and 1", file=sys.stderr)
        return 2

    rng = random.Random(args.seed)
    args.output.parent.mkdir(parents=True, exist_ok=True)

    breaking_count = 0
    edge_count = 0

    with args.output.open("w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=HEADERS)
        w.writeheader()
        for i in range(1, args.rows + 1):
            if args.breaking > 0 and rng.random() < args.breaking:
                row = breaking_row(i, rng)
                breaking_count += 1
            elif args.edge_every and i % args.edge_every == 0:
                row = edge_row(i, rng)
                edge_count += 1
            elif args.dup_name_every and i % args.dup_name_every == 0:
                row = normal_row(i, rng, force_name="Dup Name Stress")
            else:
                row = normal_row(i, rng)
            w.writerow({h: row.get(h, "") for h in HEADERS})

    size = args.output.stat().st_size
    print(
        f"Wrote {args.output} rows={args.rows} bytes={size} "
        f"edge≈{edge_count} breaking≈{breaking_count} seed={args.seed}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
