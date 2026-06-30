# Generated datasets

Do not commit huge CSVs. Generate locally:

```bash
# 10k mixed (≈2% breaking rows + periodic edge rows)
python3 scripts/generate_portfolio_csv.py -n 10000 --breaking 0.02 --edge-every 50 \
  -o examples/import/generated/large_10k_mixed.csv

# 100k stress (may take a few seconds; file ~tens of MB)
python3 scripts/generate_portfolio_csv.py -n 100000 --breaking 0.01 \
  -o examples/import/generated/large_100k.csv

# Reproducible
python3 scripts/generate_portfolio_csv.py -n 5000 --seed 1 -o /tmp/p.csv
```

Committed fixtures live one level up: `edge_cases.csv`, `people.csv`.
