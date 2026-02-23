# Premier League Predictor - Backend

Rust-based backend API for real-time Premier League match predictions.

## Prerequisites

- **Rust** 1.75+ ([Install Rust](https://rustup.rs/))
- **Docker** and **Docker Compose** (for running PostgreSQL and Redis)
- **PostgreSQL** 16+ (or use Docker)
- **Redis** 7+ (or use Docker)

## Quick Start

### 1. Install Rust

```bash
# Windows
# Download and run rustup-init.exe from https://rustup.rs/

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Set up environment variables

```bash
cp .env.example .env
# Edit .env with your configuration
```

### 3. Start PostgreSQL and Redis with Docker

```bash
docker-compose up -d postgres redis
```

This will:
- Start PostgreSQL on port 5432
- Start Redis on port 6379
- Automatically run database migrations

### 4. Build and run the application

```bash
# Check if everything compiles
cargo check

# Run in development mode
cargo run

# Run with release optimizations
cargo run --release
```

The API will be available at `http://localhost:3000`

## Project Structure

```
backend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── api/                 # REST API endpoints
│   │   ├── matches.rs       # Match-related endpoints
│   │   ├── predictions.rs   # Prediction endpoints
│   │   └── teams.rs         # Team endpoints
│   ├── models/              # Data models
│   │   ├── match_model.rs   # Match structures
│   │   ├── prediction.rs    # Prediction structures
│   │   └── team.rs          # Team structures
│   ├── db/                  # Database layer
│   │   └── postgres.rs      # PostgreSQL connection
│   ├── ml/                  # ML inference
│   │   ├── feature_engineering.rs
│   │   └── inference.rs     # ONNX model loading
│   ├── external/            # External API clients
│   │   └── football_api.rs  # Football data API
│   └── utils/
│       └── config.rs        # Configuration
├── migrations/              # SQL migration files
├── Cargo.toml              # Rust dependencies
├── Dockerfile              # Production Docker image
└── docker-compose.yml      # Development services
```

## API Endpoints

### Health Check
- `GET /` - API info
- `GET /health` - Health check

### Matches
- `GET /api/v1/matches` - List all matches
- `GET /api/v1/matches/upcoming` - Upcoming matches with predictions
- `GET /api/v1/matches/:id` - Get specific match

### Teams
- `GET /api/v1/teams` - List all teams
- `GET /api/v1/teams/:id` - Get specific team
- `GET /api/v1/teams/:id/stats` - Get team statistics

### Predictions
- `GET /api/v1/predictions/:match_id` - Get prediction for match
- `POST /api/v1/predictions` - Create new prediction

## Development

### Running tests

```bash
cargo test
```

### Checking code

```bash
# Check compilation without building
cargo check

# Run clippy (linter)
cargo clippy

# Format code
cargo fmt
```

### Database migrations

Migrations are automatically run when using Docker Compose. To run manually:

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run
```

## Docker Deployment

### Build the Docker image

```bash
docker build -t premier-league-backend .
```

### Run with Docker Compose

```bash
# Start all services (postgres, redis, api)
docker-compose up -d

# View logs
docker-compose logs -f api

# Stop all services
docker-compose down
```

## Configuration

All configuration is done via environment variables (see `.env.example`):

- `DATABASE_URL` - PostgreSQL connection string
- `REDIS_URL` - Redis connection string
- `MODEL_PATH` - Path to ONNX model file
- `JWT_SECRET` - Secret for JWT tokens
- `FOOTBALL_API_KEY` - External API key
- `API_RATE_LIMIT` - Rate limit per IP

## Next Steps

1. **Install Rust** if you haven't already
2. **Add ONNX model** - Export your ML model to ONNX and place in `models/` directory
3. **Configure API keys** - Add your football data API key to `.env`
4. **Implement database queries** - The endpoints have TODOs for actual database queries
5. **Test the API** - Use curl or Postman to test endpoints

## ML Model Export

To export your scikit-learn model to ONNX format:

```python
# In your Python environment
import pickle
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

# Load your trained model
model = pickle.load(open('model.pkl', 'rb'))

# Define input shape (16 features as per your model)
initial_type = [('input', FloatTensorType([None, 16]))]

# Convert to ONNX
onnx_model = convert_sklearn(model, initial_types=initial_type)

# Save
with open("predictor.onnx", "wb") as f:
    f.write(onnx_model.SerializeToString())
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [ONNX Runtime Rust](https://docs.rs/ort/latest/ort/)

## License

MIT
