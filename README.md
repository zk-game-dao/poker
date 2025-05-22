# ZkPoker/PurePoker: Decentralized Poker on the Internet Computer

ZkPoker and PurePoker are fully decentralized poker platforms built on the Internet Computer Protocol (ICP), offering provably fair poker games with on-chain randomness, transparent transactions, and a trustless gaming environment.

## 🌟 Features

- **Provably Fair Gameplay**: Leveraging the Internet Computer's on-chain randomness via VRF
- **Multi-Currency Support**: Play with ICP, CKBTC, CKETH, and other ICRC-1 tokens
- **Tournament System**: From sit-and-gos to multi-table tournaments with flexible structures
- **Leaderboard & Rewards**: Competitive leaderboards with automated reward distribution
- **Proof of Humanity**: Integration with DecideAI to prevent multi-accounting and collusion

## 🏗️ Architecture Overview

The platform is built on a modular canister architecture designed for scalability, fault isolation, and efficient resource utilization:

### Core Index Canisters

- **Users Index (`users_index.rs`)**: Manages user accounts and user canisters
- **Table Index (`table_index.rs`)**: Handles table creation, discovery, and lifecycle management
- **Tournament Index (`tournament_index.rs`)**: Orchestrates poker tournaments of various formats

### Game Logic Components

- **Table Canister (`table_canister.rs`)**: Implements core poker logic and game states
- **Tournament Canister (`tournament_canister.rs`)**: Manages tournament-specific functionality
- **User Canister (`users_canister.rs`)**: Stores user data, balances, and statistics

### Infrastructure Services

- **Cycle Dispenser (`cycle_dispenser.rs`)**: Manages cycle distribution across the system
- **Log Store (`log_store.rs`)**: Records game actions for transparency and verification

## 🛠️ Technical Components

### User Management System

The users index creates and manages user canisters, with each user canister able to store up to 1,000 user accounts. This enables efficient scaling as the platform grows.

```
users_index (principal: lvq5c-nyaaa-aaaam-qdswa-cai)
├── user_canister_1 (1000 users)
├── user_canister_2 (1000 users)
└── ...
```

### Table System

Tables operate as independent canisters containing the full poker logic. The table index manages their lifecycle and provides discovery functionality.

```
table_index (principal: zbspl-ziaaa-aaaam-qbe2q-cai)
├── cash_table_1
├── cash_table_2
└── ...
```

### Tournament System

Tournaments run as independent canisters that coordinate multiple table canisters, handle player movement between tables, and manage prize pools.

```
tournament_index (principal: zocwf-5qaaa-aaaam-qdfaq-cai)
├── tournament_1
│   ├── table_1
│   ├── table_2
│   └── ...
├── tournament_2
└── ...
```

### Cycle Management

The cycle dispenser ensures all canisters have sufficient cycles to operate:

```
cycle_dispenser
├── tops up users_index
├── tops up table_index
└── tops up tournament_index
```

## 📋 Prerequisites

- [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install) (version 0.15.0 or later)
- Rust (version 1.70.0 or later)
- Node.js (version 18 or later)
- [candid-extractor](https://github.com/dfinity/candid/tree/master/tools/candid-extractor) (for updating .did files)
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools) (optional, for WASM optimization)
- [jq](https://stedolan.github.io/jq/) (required for some deployment scripts)

## 🚀 Getting Started

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/your-org/zkpoker.git
cd zkpoker
```

2. Start the local Internet Computer replica:
```bash
dfx start --clean --background
```

3. Deploy the project:
```bash
./scripts/deploy.sh local
```

4. Start the local frontend development server:
- For ZKPoker
```bash
npm run start:zkp
```
- For PurePoker
```bash
npm run start:pp
```

5. Access the frontend:
```
http://localhost:5173/
```

### Production Deployment

1. Deploy to the IC mainnet:
```bash
./scripts/deploy.sh mainnet
```

### Update Candid Definitions

To update the .did files for all canisters:
```bash
./scripts/deploy.sh update all
```

Or for specific canisters:
```bash
./scripts/deploy.sh update users_index table_canister
```

## 🧩 Canister Structure

| Canister | Description | Functionality |
|----------|-------------|---------------|
| `users_index` | User management orchestrator | Creates and manages user canisters, handles leaderboards |
| `users_canister` | User data storage | Stores user profiles, balances, and statistics |
| `table_index` | Table factory and registry | Creates, tracks, and manages poker tables |
| `table_canister` | Poker game logic | Implements game rules, betting, and card dealing |
| `tournament_index` | Tournament organizer | Creates and manages tournament structures |
| `tournament_canister` | Tournament logic | Handles tournament progression and table balancing |
| `cycle_dispenser` | Cycle management | Distributes cycles to maintain system operation |
| `log_store` | Action logging | Records game actions for verification |

## 💻 Development

## 📁 Repository Structure

```
.
├── Cargo.toml                  # Workspace configuration
├── dfx.json                    # DFX configuration
├── canister_ids.json           # Canister IDs for deployments
├── libraries/                  # Shared library crates
│   ├── authentication/         # Authentication utilities
│   ├── canister_functions/     # Common canister functions
│   ├── chat/                   # In-game chat functionality
│   ├── errors/                 # Error types and handling
│   ├── frontend/               # Shared frontend utilities
│   ├── table/                  # Poker table and game logic
│   ├── table_index_types/      # Types for table indexing
│   ├── tournaments/            # Tournament framework
│   └── user/                   # User profile management
├── scripts/                    # Deployment and utility scripts
│   ├── build-canisters.sh      # Build canister Wasm modules
│   ├── deploy.sh               # Main deployment script
│   ├── run-tests.sh            # Run integration tests
│   └── top_up_canisters.sh     # Manage canister cycles
├── src/                        # Canister implementations
│   ├── app_frontend/           # Main application frontend
│   ├── btc_frontend/           # BTC-specific frontend
│   ├── cycle_dispenser/        # Cycle management canister
│   ├── log_store/              # Game action logging
│   ├── table_canister/         # Poker table implementation
│   ├── table_index/            # Table management
│   ├── tournament_canister/    # Tournament implementation
│   ├── tournament_index/       # Tournament management
│   ├── users_canister/         # User data storage
│   └── users_index/            # User management
└── tests/                      # Integration tests
    └── src/                    # Test implementation
        ├── basic_tests.rs      # Basic functionality tests
        ├── currency_tests.rs   # Currency handling tests
        ├── tournament_tests/   # Tournament-specific tests
        └── ...                 # Additional test modules
```

### Testing

Run the test suite with PocketIC (local IC simulator):

```bash
./scripts/run-tests.sh
```

You can also run specific tests:

```bash
./scripts/run-tests.sh basic_tests
```

These tests use PocketIC to create a simulated IC environment, which will be automatically downloaded the first time you run the tests.

### Key Files and Components

- **Index Canisters**:
  - `src/users_index/src/lib.rs`: Main user management logic
  - `src/table_index/src/lib.rs`: Table creation and management
  - `src/tournament_index/src/lib.rs`: Tournament orchestration

- **Game Logic**:
  - `libraries/table/src/poker/game/table_functions/table.rs`: Core poker table implementation
  - `src/table_canister/src/lib.rs`: Table canister entry points
  - `libraries/table/src/poker/core/`: Card and hand ranking logic

- **Infrastructure**:
  - `src/cycle_dispenser/src/lib.rs`: Cycle management system
  - `src/users_index/src/reset_xp_utils.rs`: Leaderboard management
  - `src/tournament_canister/src/table_balancing.rs`: Tournament table balancing

- **Deployment**:
  - `scripts/deploy.sh`: Main deployment script
  - `dfx.json`: DFX configuration
  - `canister_ids.json`: Production canister IDs

## 🔄 Deployment Workflow

The project uses a structured deployment process:

1. **Local Development**:
   - Use `dfx start --clean` to run a local IC replica
   - Deploy with `./scripts/deploy.sh local` for testing

2. **Building and Testing**:
   - Compile canisters with `./scripts/build-canisters.sh`
   - Run tests with `./scripts/run-tests.sh`

3. **Canister Updates**:
   - Extract Candid interfaces with `./scripts/deploy.sh update all`
   - Update specific canisters with `./scripts/deploy.sh update <canister_name>`

4. **Production Deployment**:
   - Deploy to mainnet with `./scripts/deploy.sh mainnet`
   - Manage canister cycles with `./scripts/top_up_canisters.sh`

5. **Maintenance**:
   - Upgrade user canisters with `./scripts/upgrade_user_canisters.sh`
   - Upgrade table canisters with `./scripts/upgrade_table_canisters.sh`

## Troubleshooting

### error: failed to get `currency` as a dependency of package

Add to your `~/.cargo/config.toml`
```toml
[net]
git-fetch-with-cli = true
```

- [ ] Clans

## 🛣️ Roadmap

- [ ] Clans

## 🤝 Contributing

TODO!
Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📜 License

This project is licensed under the [GNU Affero General Public License v3.0 (AGPL-3.0)](LICENSE).

You are free to use, modify, and distribute this software, provided that any modified versions made available over a network also make their source code available under the same license.

See the `LICENSE` file for full details.


## 📞 Contact

- ZkPoker: [zkpoker.app](https://zkpoker.app)
- PurePoker: [purepoker.app](https://purepoker.app)
- ZkPoker Twitter: [@zkpoker](https://x.com/zkpokerapp)
- PurePoker Twitter: [@purepoker](https://x.com/purepokerapp)
- ZkPoker Discord: [ZkPoker Discord](https://discord.gg/hGEcEhCt)
