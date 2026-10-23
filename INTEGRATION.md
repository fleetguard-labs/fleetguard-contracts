# Connected setup

FleetGuard is a local, non-custodial Stellar testnet MVP. Use Node 24+; contracts use Rust 1.84+ and Stellar CLI 28.1.0+ with the wasm32v1-none target. The contract SDK is pinned to 29.0.1 and the JavaScript SDK to 17.3.0.

Clone the five repositories as sibling directories:

```sh
for part in contracts analyzer backend frontend fixtures; do
  git clone https://github.com/fleetguard-labs/fleetguard-$part.git
done
cd fleetguard-backend
npm ci
cd ../fleetguard-frontend
npm run dev
```

Open http://127.0.0.1:4350; API binds 127.0.0.1:4351. Use the synthetic sample, import complete review JSON, or capture up to 20 explicit testnet instance IDs. Capture resolves CAP-85 owner/tag references through fixed testnet RPC. Enter the proposed uploaded Wasm hash and independently verified before/after interface declarations, then analyze. Reports and history are stored in backend SQLite. API_TOKEN is optional for local development; set it on both processes when running separately. Do not expose either server publicly.

## Components

- contracts: manager with admin authorization, minimum two-ledger delay, one pending proposal and expected-current-hash checks; deploys instances sharing a tagged executable. v1/v2 fixture contracts store an admin and integer value under unchanged keys.
- analyzer: bounded registered-fleet impact analysis and comparison of explicitly declared function names, input types and output types.
- backend: fixed-testnet reference capture and persisted reviews; no signing/submission routes.
- frontend: local workbench, JSON import/export, live capture and public pilot evidence.
- fixtures: deterministic timing, stale-reference and breaking-interface cases.

## Verification

Run npm test in analyzer, backend, frontend and fixtures. In contracts, run cargo test --locked (committed fixture Wasms support the host integration test). Run sh ./build.sh to rebuild manager and v1/v2 Wasms using Stellar CLI; compiled artifacts go to ignored build/. Backend TESTNET.md explains evidence replay and live verification.

## Trust and limitations

The inventory is only the explicitly registered instances; no global dependent discovery is claimed. RPC reads are TLS-provided, span ledgers, and are not independent consensus proofs. Captured manager administrators are unknown unless independently inspected. A matching declared interface is not a storage-layout or semantic compatibility guarantee. Empty declarations are blocked. Reviewable means no implemented blocker was found; it does not mean audited, safe, authorized or approved. Custom manager policies are not automatically inferred. The reference manager administrator can upgrade every linked instance after its delay. Rollbacks cannot undo state migrations. The tool does not automate production upgrades.
