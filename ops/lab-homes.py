#!/usr/bin/env python3
"""Write home/mesh for the eight lab homes. Nothing else."""
from __future__ import annotations

import struct
from pathlib import Path

from nacl.signing import SigningKey

BASE = Path.home() / ".mycelium"
SERVERS = ("a", "b", "c")
CLIENTS = ("1", "2", "3", "4", "5")
BINDS = {"a": "127.0.0.1:7500", "b": "127.0.0.1:7501", "c": "127.0.0.1:7502"}
BOOTSTRAP = {
    "1": "127.0.0.1:7500",
    "2": "127.0.0.1:7501",
    "3": "127.0.0.1:7502",
    "4": "127.0.0.1:7500",
    "5": "127.0.0.1:7501",
}
BOND = (8, 8, 1, 8)
HYPHA = 0
VOICE = 1


def vk_hex(seed: bytes) -> str:
    return SigningKey(seed).verify_key.encode().hex()


def put_str(s: str) -> bytes:
    b = s.encode()
    n = min(len(b), 255)
    return bytes([n]) + b[:n]


def put_bond(vk: bytes, bond: tuple[int, int, int, int]) -> bytes:
    out = vk
    for x in bond:
        out += struct.pack("<Q", x)
    return out


def origin_of(home: Path) -> int:
    p = home / "tuniversal"
    if p.exists() and p.stat().st_size == 8:
        return struct.unpack("<Q", p.read_bytes())[0]
    mesh = home / "mesh"
    if mesh.exists() and len(mesh.read_bytes()) >= 12:
        # last 8 of prefix sit before 64-byte sig; origin is 8 bytes before sig
        b = mesh.read_bytes()
        if b[:4] == b"MSH1" and len(b) >= 72:
            return struct.unpack("<Q", b[-72:-64])[0]
    return 1788326481


def encode(kind: int, bind: str, peers: list[str], bootstrap: str, hypha: list[tuple[bytes, tuple]], voice: list[tuple[bytes, tuple]], origin: int) -> bytes:
    out = b"MSH1" + bytes([kind]) + put_str(bind)
    out += bytes([len(peers)])
    for p in peers:
        out += put_str(p)
    out += put_str(bootstrap)
    out += bytes([len(hypha)])
    for vk, bond in hypha:
        out += put_bond(vk, bond)
    out += bytes([len(voice)])
    for vk, bond in voice:
        out += put_bond(vk, bond)
    out += struct.pack("<Q", origin)
    return out


def main() -> None:
    hypha_keys = {}
    for name in SERVERS:
        seed = (BASE / name / "hypha.key").read_bytes()
        hypha_keys[name] = (seed, bytes.fromhex(vk_hex(seed)))
    voice_keys = {}
    for name in CLIENTS:
        seed = (BASE / name / "voice.key").read_bytes()
        voice_keys[name] = (seed, bytes.fromhex(vk_hex(seed)))
    hypha = [(hypha_keys[n][1], BOND) for n in SERVERS]
    voice = [(voice_keys[n][1], BOND) for n in CLIENTS]
    origin = origin_of(BASE / "a")
    for name in SERVERS:
        peers = [BINDS[s] for s in SERVERS if s != name]
        body = encode(HYPHA, BINDS[name], peers, "", hypha, voice, origin)
        sig = SigningKey(hypha_keys[name][0]).sign(body).signature
        (BASE / name / "mesh").write_bytes(body + sig)
        print(f"  server {name} bind {BINDS[name]} vk {vk_hex(hypha_keys[name][0])[:4]}…")
    for name in CLIENTS:
        body = encode(VOICE, "", [], BOOTSTRAP[name], hypha, voice, origin)
        sig = SigningKey(voice_keys[name][0]).sign(body).signature
        (BASE / name / "mesh").write_bytes(body + sig)
        print(f"  client {name} bootstrap {BOOTSTRAP[name]} vk {vk_hex(voice_keys[name][0])[:4]}…")


if __name__ == "__main__":
    main()
