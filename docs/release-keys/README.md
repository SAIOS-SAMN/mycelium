# Release keys

Release tags are signed (MU-RELEASE Part II). While the network is a
prototype, tags are signed with a **prototype key**. It proves which tag
the maintainers cut. It is not a production key, and it will be replaced
when real nodes or cloud infrastructure come online.

| Key | Fingerprint | Used for | Status |
|---|---|---|---|
| [`prototype-2026-11.pub`](prototype-2026-11.pub) (ed25519) | `SHA256:G0WgTTsRvfexVZRL9OQjxUZFs4rbGvusjnFrCVUZeHg` | Tags from `v0.3.0-rc.3` | **Prototype** |

**Verifying a tag** with git's SSH signatures:

```sh
echo "release $(cat docs/release-keys/prototype-2026-11.pub)" > allowed_signers
git -c gpg.ssh.allowedSignersFile=allowed_signers verify-tag v0.3.0-rc.3
```

A replacement key is added here beside the old one, with the tag from which
it signs, and the old key is marked retired. A key is never removed.
