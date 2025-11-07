# Setup Guide

This guide covers the setup and installation process for DotRepute's **Rust backend**, **frontend**, and **optional components**.

---

## Prerequisites

### Required Software
- **Rust** v1.70+ (with cargo) - [Install via rustup](https://rustup.rs/)
- **Node.js** v18+ (LTS recommended)
- **npm** v9+ or **yarn** v1.22+
- **Git**
- **Docker** (optional, for containerized setup)

### Recommended Tools
- **VS Code** with rust-analyzer extension
- **Polkadot.js Extension** for browser wallet integration
- **PostgreSQL** (if using backend with database)
- **cargo-watch** for development (`cargo install cargo-watch`)

---

## Frontend Setup

### 1. Navigate to Frontend Directory

```bash
cd frontend
```

### 2. Install Dependencies

```bash
npm install
# or
yarn install
```

### 3. Environment Configuration

Create a `.env.local` file in the `frontend/` directory:

```env
# Polkadot RPC Endpoints
NEXT_PUBLIC_POLKADOT_RPC=wss://rpc.polkadot.io
NEXT_PUBLIC_KUSAMA_RPC=wss://kusama-rpc.polkadot.io

# SubQuery Endpoint
NEXT_PUBLIC_SUBQUERY_ENDPOINT=http://localhost:3001

# Backend API (if using backend)
NEXT_PUBLIC_API_URL=http://localhost:4000/api

# Contract Address (if using ink! contracts)
NEXT_PUBLIC_CONTRACT_ADDRESS=

# Application Settings
NEXT_PUBLIC_APP_NAME=DotRepute
NEXT_PUBLIC_NETWORK=polkadot
```

### 4. Start Development Server

```bash
npm run dev
# or
yarn dev
```

The frontend will be available at `http://localhost:3000`

### 5. Build for Production

```bash
npm run build
npm run start
# or
yarn build
yarn start
```

### 6. Type Generation (if using PolkadotJS types)

```bash
npm run generate:types
# or
yarn generate:types
```

---

## Rust Backend Setup

### 1. Navigate to Backend Directory

```bash
cd backend
```

### 2. Verify Rust Installation

```bash
rustc --version
cargo --version
```

### 3. Build the Project

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### 4. Environment Configuration

Create a `.env` file in the `backend/` directory:

```env
# Server Configuration
PORT=4000
NODE_ENV=development

# Polkadot Configuration
POLKADOT_RPC=wss://rpc.polkadot.io
KUSAMA_RPC=wss://kusama-rpc.polkadot.io

# Database (PostgreSQL)
DATABASE_URL=postgresql://user:password@localhost:5432/dotrepute
DB_HOST=localhost
DB_PORT=5432
DB_NAME=dotrepute
DB_USER=your_db_user
DB_PASSWORD=your_db_password

# SubQuery
SUBQUERY_ENDPOINT=http://localhost:3001

# GitHub Integration (optional)
GITHUB_API_TOKEN=your_github_personal_access_token

# Redis (for caching, optional)
REDIS_URL=redis://localhost:6379

# CORS
CORS_ORIGIN=http://localhost:3000

# API Keys
API_SECRET=your_secret_key_here

# Reputation Scoring Weights
WEIGHT_IDENTITY=0.25
WEIGHT_GOVERNANCE=0.25
WEIGHT_STAKING=0.20
WEIGHT_ACTIVITY=0.20
WEIGHT_DEV_CONTRIBUTIONS=0.10
```

### 4. Database Setup (if using PostgreSQL)

Install PostgreSQL and create a database:

```bash
# Using psql
createdb dotrepute

# Or using Docker
docker run --name dotrepute-db \
  -e POSTGRES_PASSWORD=your_password \
  -e POSTGRES_DB=dotrepute \
  -p 5432:5432 \
  -d postgres:15
```

Run migrations (if applicable):

```bash
npm run migrate
# or
yarn migrate
```

### 5. Start Development Server

```bash
# Run with cargo
cargo run

# Or with auto-reload during development
cargo watch -x run

# Run in release mode (faster runtime)
cargo run --release
```

The backend API will be available at `http://localhost:4000`

### 6. Build for Production

```bash
# Build optimized binary
cargo build --release

# The binary will be at target/release/dotrepute-backend
./target/release/dotrepute-backend
```

### 7. Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench
```

### 8. API Documentation

Once running, API documentation will be available at:
- Swagger/OpenAPI: `http://localhost:4000/api-docs`

---

## ink! Smart Contract Setup (Optional)

### 1. Install cargo-contract

```bash
cargo install cargo-contract --force
```

### 2. Navigate to Contract Directory

```bash
cd contracts/crs_contract
```

### 3. Build the Contract

```bash
# Using build script
./build.sh

# Or manually
cargo contract build --release
```

### 4. Run Contract Tests

```bash
cargo test
```

### 5. Deploy to Testnet

```bash
# Deploy using Contracts UI or via script
cargo contract instantiate --constructor new --args "arg1" --suri //Alice
```

---

## Running Both Frontend and Backend

### Option 1: Separate Terminals

Terminal 1 (Rust Backend):
```bash
cd backend
cargo run
```

Terminal 2 (Frontend):
```bash
cd frontend
npm run dev
```

### Option 2: Using Concurrently (from root)

Install concurrently at the root level:

```bash
npm install -g concurrently
```

Create a `package.json` at the root:

```json
{
  "scripts": {
    "dev": "concurrently \"cargo run --manifest-path=backend/Cargo.toml\" \"npm run dev --prefix frontend\"",
    "dev:watch": "concurrently \"cargo watch -x 'run --manifest-path=backend/Cargo.toml'\" \"npm run dev --prefix frontend\""
  }
}
```

Then run:
```bash
npm run dev
# or with auto-reload
npm run dev:watch
```

### Option 3: Docker Compose

Create a `docker-compose.yml` at the root:

```yaml
version: '3.8'

services:
  frontend:
    build: ./frontend
    ports:
      - "3000:3000"
    environment:
      - NEXT_PUBLIC_API_URL=http://localhost:4000/api
    depends_on:
      - backend

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    ports:
      - "4000:4000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/dotrepute
      - RUST_LOG=info
    depends_on:
      - db

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=dotrepute
      - POSTGRES_PASSWORD=password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

Create `backend/Dockerfile`:

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dotrepute-backend /usr/local/bin/dotrepute-backend

EXPOSE 4000
CMD ["dotrepute-backend"]
```

Run with:
```bash
docker-compose up --build
```

---

## Troubleshooting

### Frontend Issues

**Port already in use:**
```bash
# Kill process on port 3000
npx kill-port 3000
```

**Polkadot connection issues:**
- Check RPC endpoint is accessible
- Try alternative endpoints:
  - `wss://polkadot.api.onfinality.io/public-ws`
  - `wss://rpc.dotters.network/polkadot`

**Build errors:**
```bash
# Clear cache and reinstall
rm -rf node_modules .next
npm install
```

### Rust Backend Issues

**Compilation errors:**
```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for errors
cargo check
```

**Database connection errors:**
- Verify PostgreSQL is running
- Check DATABASE_URL format
- Ensure database exists

**Port already in use:**
```bash
# Find process using port 4000 (Linux/Mac)
lsof -i :4000

# Kill process
kill -9 <PID>

# Windows
netstat -ano | findstr :4000
taskkill /PID <PID> /F
```

**Missing environment variables:**
- Check `.env` file exists
- Verify all required variables are set
- Rust requires explicit .env loading (use `dotenv` crate)

**WASM compilation issues:**
```bash
# Install wasm32 target
rustup target add wasm32-unknown-unknown

# Rebuild
cargo build --target wasm32-unknown-unknown
```

---

## Testing

### Rust Backend Tests
```bash
cd backend

# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test scoring::tests

# Run integration tests
cargo test --test integration_tests

# Run with coverage (requires tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Frontend Tests
```bash
cd frontend
npm run test
npm run test:e2e
```

### ink! Contract Tests
```bash
cd contracts/crs_contract
cargo test
```

---

## Next Steps

After setup:

1. **Configure Indexer** - See `indexer/README.md`
2. **Deploy Contracts** (optional) - See `contracts/README.md`
3. **Review Scoring Model** - See `docs/scoring-model.md`
4. **Check API Documentation** - See `docs/api-spec.md`

---

## Additional Resources

- [Architecture Documentation](./architecture.md)
- [Scoring Model](./scoring-model.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Polkadot.js Documentation](https://polkadot.js.org/docs/)
- [Next.js Documentation](https://nextjs.org/docs)
