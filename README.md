# FleetGuard — contracts

Soroban reference manager and two state-compatible fixture implementations.

FleetGuard reviews shared executable upgrades for registered Stellar contract fleets. Read [INTEGRATION.md](INTEGRATION.md) for connected setup and trust boundaries.

Components: [contracts](https://github.com/fleetguard-labs/fleetguard-contracts), [analyzer](https://github.com/fleetguard-labs/fleetguard-analyzer), [backend](https://github.com/fleetguard-labs/fleetguard-backend), [frontend](https://github.com/fleetguard-labs/fleetguard-frontend), [fixtures](https://github.com/fleetguard-labs/fleetguard-fixtures).

Run cargo test --locked; sh ./build.sh rebuilds all Wasms using Stellar CLI. Committed v1.wasm/v2.wasm are test fixtures, not independent audited implementations.

MIT licensed. Testnet MVP; no production readiness or complete dependency inventory claimed.
