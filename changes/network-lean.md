### scripts/network resumes a network it provisioned, and works fewer tasks by default

- **`MYCELIUM_NETWORK_RESUME=1`** starts the nodes on the homes `DIR` already holds, keeping their keys, genesis, ledgers and the house's domain. Nothing is provisioned and no genesis is minted. The house founds nothing again: its seats house and seated mind sit the domain founded before. No worker runs unless `MYCELIUM_NETWORK_WORK` is set. A home that was not provisioned there, or that binds other ports, is refused by name.
- **Bring-up time:** a resumed network is serving in seconds, against the minutes a fresh provision takes, which is mostly the worker and the house's founding.
- **`MYCELIUM_NETWORK_WORK`** now defaults to 8, as `scripts/quickstart` does; 30 is the whole corpus, and 0 runs no worker.
