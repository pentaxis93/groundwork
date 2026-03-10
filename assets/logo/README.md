# Logo Assets

Canonical sources live in this directory:

- `groundwork-mark.svg`
- `groundwork-full.svg`

Derived assets are generated into `assets/logo/dist/` and are intentionally not committed.

## Regenerate

From the repo root:

```bash
python3 assets/logo/generate.py
```

If dependencies are missing, use a virtual environment:

```bash
python3 -m venv /tmp/groundwork-logo-venv
/tmp/groundwork-logo-venv/bin/python -m pip install cairosvg Pillow
/tmp/groundwork-logo-venv/bin/python assets/logo/generate.py
```

## Generated Outputs

- `dist/favicon.ico`
- `dist/favicon.svg`
- `dist/apple-touch-icon.png`
- `dist/logo-32.png`
- `dist/logo-64.png`
- `dist/logo-128.png`
- `dist/logo-256.png`
- `dist/logo-512.png`
- `dist/github-social-preview.png`
- `dist/og-image.png`
- `dist/avatar-500.png`
