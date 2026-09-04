#!/usr/bin/env python3
"""Mint uniform lab homes under ~/.mycelium for server@{a,b,c} and client@{1..5}."""
from __future__ import annotations

import os
import shutil
from pathlib import Path

import blake3
from nacl.signing import SigningKey

ROOT = Path("/opt/mycelium-v0.1.0")
CORPUS = ROOT / "data/arc-agi-2/training"
STATE = ROOT / "state"
BASE = Path.home() / ".mycelium"
SERVERS = ("a", "b", "c")
CLIENTS = ("1", "2", "3", "4", "5")
BINDS = {"a": "127.0.0.1:7500", "b": "127.0.0.1:7501", "c": "127.0.0.1:7502"}
BOOTSTRAP = "127.0.0.1:7500\n"
# Per-lane bond (𝒮, 𝒢, 𝒯, 𝒽) — not one scalar broadcast to four lanes.
BOND = (8, 8, 1, 8)
ALLOC = (8, 8, 1, 8)
LEDGERS = {"a": "hypha-a", "b": "hypha-b", "c": "hypha-c"}
GENOMES = {"1": "client-a", "2": "client-b", "3": "client-c", "4": "worker-a", "5": "worker-b"}


def mint_seed(path: Path) -> bytes:
    if path.exists() and path.stat().st_size == 32:
        return path.read_bytes()
    seed = os.urandom(32)
    path.write_bytes(seed)
    return seed


def vk_hex(seed: bytes) -> str:
    return SigningKey(seed).verify_key.encode().hex()


def corpus_hash(dir: Path) -> bytes:
    files = sorted(p for p in dir.iterdir() if p.suffix == ".json")
    h = blake3.blake3()
    for f in files:
        rel = f.relative_to(dir).as_posix()
        h.update(rel.encode())
        h.update(f.read_bytes())
    return h.digest()


def main() -> None:
    BASE.mkdir(parents=True, exist_ok=True)
    keys = {}
    for name in SERVERS:
        home = BASE / name
        home.mkdir(parents=True, exist_ok=True)
        (home / "corpora").mkdir(exist_ok=True)
        (home / "feed").mkdir(exist_ok=True)
        seed = mint_seed(home / "hypha.key")
        keys[name] = (seed, vk_hex(seed))
        (home / "bind").write_text(BINDS[name] + "\n")
        others = "".join(f"{BINDS[s]}\n" for s in SERVERS if s != name)
        (home / "peers").write_text(others)
        if (STATE / "tuniversal").exists():
            shutil.copy2(STATE / "tuniversal", home / "tuniversal")
        if (STATE / "court.federation").exists():
            shutil.copy2(STATE / "court.federation", home / "court.federation")
        src = STATE / f"{LEDGERS[name]}.ledger"
        if src.exists():
            shutil.copy2(src, home / "ledger")
        reg = STATE / f"{LEDGERS[name]}.ledger.registry"
        if reg.exists():
            shutil.copy2(reg, home / "ledger.registry")

    voices = {}
    for name in CLIENTS:
        home = BASE / name
        home.mkdir(parents=True, exist_ok=True)
        (home / "feed").mkdir(exist_ok=True)
        seed = mint_seed(home / "voice.key")
        voices[name] = (seed, vk_hex(seed))
        (home / "bootstrap").write_text(BOOTSTRAP)
        if (STATE / "tuniversal").exists():
            shutil.copy2(STATE / "tuniversal", home / "tuniversal")
        genome = STATE / f"{GENOMES[name]}.genome"
        if genome.exists():
            shutil.copy2(genome, home / "genome")

    digest = corpus_hash(CORPUS)
    sig = SigningKey(keys["a"][0]).sign(digest).signature
    manifest = f"{CORPUS}\n{digest.hex()}\n{sig.hex()}\n"
    genesis_lines = []
    for name in SERVERS:
        genesis_lines.append(f"hypha {keys[name][1]} {BOND[0]} {BOND[1]} {BOND[2]} {BOND[3]}")
    genesis_lines.append(f"voice {voices['1'][1]} {ALLOC[0]} {ALLOC[1]} {ALLOC[2]} {ALLOC[3]}")
    genesis = "\n".join(genesis_lines) + "\n"
    for name in SERVERS:
        home = BASE / name
        (home / "genesis").write_text(genesis)
        (home / "corpora" / "arc-agi-2.manifest").write_text(manifest)

    print("homes", BASE)
    for name in SERVERS:
        print(f"  server {name} vk {keys[name][1][:4]}… bind {BINDS[name]}")
    for name in CLIENTS:
        print(f"  client {name} vk {voices[name][1][:4]}…")


if __name__ == "__main__":
    main()
