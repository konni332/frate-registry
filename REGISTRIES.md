# Frate Registries

## Overview

Frate registries are JSON files describing downloadable tools with versioned releases tied to specific OS and architecture triples. Each registry corresponds to one tool and contains:

- `name`: tool name
- `repo`: GitHub repository (owner/name)
- `releases`: map of version strings (including platform triple, e.g. `1.42.1-windows-x86_64`) to download info:
    - `url`: direct download link to release asset (zip or tar.gz)
    - `hash`: SHA256 hash of the asset, prefixed with `sha256:`

This format allows frate to resolve the correct binary per platform and validate downloads by hash.

---

## CLI Usage

The CLI tool `frate-registry-gen` supports two commands:

- `generate`  
  Generate a single registry JSON from a GitHub repo’s releases.

  Example:  
 ```shell
frate-registry-gen generate --name just --repo casey/just --out tools/just.json --max 10
```

---

Options:
- `--name`: tool name (e.g., `just`)
- `--repo`: GitHub repo `owner/name` (no URL prefix)
- `--out`: optional output file path (default: `tools/<name>.json`)
- `--max`: optional max number of releases to include

- `from-list`  
  Generate registries in batch from a JSON file describing multiple tools.

Example:  
````shell
frate-registry-gen from-list tool-list.json
````

---

The JSON file contains an array of objects with fields:
```json
[
  {
    "name": "just",
    "repo": "casey/just",
    "out": "tools/just.json",
    "max": 10
  }
]
```

---

## Included Tools

Here are some popular tools already registered with frate:

    just — casey/just

    ripgrep — BurntSushi/ripgrep

    fd — sharkdp/fd

    bat — sharkdp/bat

    exa — ogham/exa

    hyperfine — sharkdp/hyperfine

    jq — stedolan/jq

    zoxide — ajeetdsouza/zoxide

    du-dust — bootandy/dust

    nvim — neovim/neovim

(… and more — see tools/tool-list.json for the full batch.)

⚙️ The registries enable frate to seamlessly download, verify, and shim these CLI tools per platform.

---