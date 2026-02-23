# Getting Started - Premier League Predictor Backend

This guide will help you set up the Rust backend for the first time.

## Step 1: Install Rust

### Windows
1. Download rustup-init.exe from https://rustup.rs/
2. Run the installer and follow the prompts
3. Restart your terminal/command prompt
4. Verify installation:
   ```bash
   rustc --version
   cargo --version
   ```

### macOS/Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

## Step 2: Install Docker Desktop

Download and install Docker Desktop from https://www.docker.com/products/docker-desktop

This provides both Docker and Docker Compose, which we'll use to run PostgreSQL and Redis.

## Step 3: Set Up the Project

```bash
# Navigate to the backend directory
cd backend

# Copy the example environment file
cp .env.example .env

# Edit .env and set your configuration
# For now, the defaults should work for local development
```

## Step 4: Start the Database Services

```bash
# Start PostgreSQL and Redis
docker-compose up -d postgres redis

# Check that they're running
docker-compose ps

# View logs if needed
docker-compose logs postgres
docker-compose logs redis
```

This will:
- Start PostgreSQL on localhost:5432
- Start Redis on localhost:6379
- Create the database schema automatically

## Step 5: Verify the Project Compiles

```bash
# Check if everything compiles (this will download dependencies)
cargo check

# This may take a few minutes the first time as it downloads and compiles all dependencies
```

If you see errors, they're likely because:
1. The ONNX model file doesn't exist yet (that's okay, we'll create it)
2. Some features aren't fully implemented yet (expected)

## Step 6: Export Your ML Model to ONNX

Before the backend can make predictions, you need to export your trained model:

```bash
# Install required Python packages
pip install skl2onnx onnx onnxruntime scikit-learn

# In your Jupyter notebook, train your model, then save it:
import pickle
with open('model.pkl', 'wb') as f:
    pickle.dump(best_model, f)  # Replace 'best_model' with your trained model variable

# Then run the export script
python export_model_to_onnx.py
```

The ONNX model will be saved to `backend/models/predictor.onnx`

## Step 7: Run the Backend (when ready)

Once you have the ONNX model in place:

```bash
cd backend

# Run in development mode
cargo run

# The API will start on http://localhost:3000
```

Test it:
```bash
# In another terminal
curl http://localhost:3000/health
# Should return: OK
```

## Step 8: Explore the API

The basic endpoints are:

- `GET /` - API info
- `GET /health` - Health check
- `GET /api/v1/matches` - List matches
- `GET /api/v1/teams` - List teams
- `GET /api/v1/predictions/:match_id` - Get prediction

Note: Most endpoints return "NOT IMPLEMENTED" initially. You'll need to:
1. Populate the database with team and match data
2. Implement the database queries (see TODOs in the code)
3. Set up an external API for live data

## What's Next?

### Phase 1: Learn Rust Basics (1-2 weeks)
- Complete "The Rust Programming Language" chapters 1-10
- Do the Rustlings exercises: https://github.com/rust-lang/rustlings
- Build a simple CLI tool to get comfortable

### Phase 2: Set Up Data Pipeline (2-3 weeks)
1. Research and choose a football data API
2. Create scripts to fetch and populate the database
3. Implement the database queries in the Rust code
4. Test predictions with real data

### Phase 3: Build the Frontend (3-4 weeks)
1. Set up a Next.js or React project
2. Create pages for matches and predictions
3. Connect to the Rust API
4. Add styling and polish

## Troubleshooting

### "cargo: command not found"
- Make sure you restart your terminal after installing Rust
- Check that `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin` on Windows) is in your PATH

### "Cannot connect to database"
- Make sure Docker is running
- Check that PostgreSQL is running: `docker-compose ps`
- Verify the DATABASE_URL in .env matches the Docker settings

### "ONNX model not found"
- You need to export your model first (see Step 6)
- Make sure the MODEL_PATH in .env points to the correct location

### Compilation errors
- Make sure you have the latest Rust: `rustup update`
- Try cleaning and rebuilding: `cargo clean && cargo build`

## Useful Commands

```bash
# Rust development
cargo check          # Check for errors without building
cargo build         # Build in debug mode
cargo build --release  # Build optimized version
cargo test          # Run tests
cargo clippy        # Run linter
cargo fmt           # Format code

# Docker
docker-compose up -d        # Start services in background
docker-compose down         # Stop services
docker-compose logs -f api  # Follow logs
docker-compose ps           # List running services

# Database
docker-compose exec postgres psql -U postgres -d premier_league  # Connect to DB
```

## Learning Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/) - Best place to start
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples

### Axum (Web Framework)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)

### Database
- [SQLx Guide](https://docs.rs/sqlx/latest/sqlx/)
- [PostgreSQL Tutorial](https://www.postgresql.org/docs/current/tutorial.html)

## Need Help?

1. Check the [backend/README.md](backend/README.md) for more details
2. Review the [ROADMAP.md](ROADMAP.md) for the overall plan
3. Look at the code comments - many have TODO items explaining what needs to be done

Good luck building your Premier League prediction app! 🚀⚽
