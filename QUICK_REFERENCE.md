# Quick Reference Card

## Project Commands

### Start Development Environment
```bash
# Start database services
cd backend
docker-compose up -d postgres redis

# Run Rust API
cargo run
```

### Stop Everything
```bash
docker-compose down
```

## File Locations

| What | Where |
|------|-------|
| Main Rust code | `backend/src/main.rs` |
| API endpoints | `backend/src/api/` |
| Database schema | `backend/migrations/001_initial_schema.sql` |
| Configuration | `backend/.env` |
| ML model | `backend/models/predictor.onnx` |
| Jupyter notebook | `premier-league-predictor.ipynb` |
| Roadmap | `ROADMAP.md` |

## API Endpoints

Base URL: `http://localhost:3000`

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/v1/matches` | GET | List all matches |
| `/api/v1/matches/upcoming` | GET | Upcoming matches with predictions |
| `/api/v1/matches/:id` | GET | Get specific match |
| `/api/v1/teams` | GET | List all teams |
| `/api/v1/teams/:id` | GET | Get specific team |
| `/api/v1/teams/:id/stats` | GET | Team statistics |
| `/api/v1/predictions/:match_id` | GET | Get prediction |
| `/api/v1/predictions` | POST | Create prediction |

## Common Tasks

### Check if Rust is working
```bash
cargo --version
cargo check
```

### View database
```bash
docker-compose exec postgres psql -U postgres -d premier_league
# Then run SQL: SELECT * FROM teams;
```

### View logs
```bash
docker-compose logs -f postgres
docker-compose logs -f redis
# Or when API is running:
docker-compose logs -f api
```

### Reset database
```bash
docker-compose down -v  # Deletes volumes
docker-compose up -d postgres redis
```

### Export ML model
```bash
python export_model_to_onnx.py
```

### Run tests
```bash
cargo test
cargo clippy  # Linting
cargo fmt     # Format code
```

## Environment Variables

Create `backend/.env` from `.env.example`:

```bash
DATABASE_URL=postgres://postgres:password@localhost:5432/premier_league
REDIS_URL=redis://localhost:6379
MODEL_PATH=./models/predictor.onnx
JWT_SECRET=your-secret-here
FOOTBALL_API_KEY=your-api-key
API_RATE_LIMIT=100
```

## Database Schema Summary

- **teams**: Premier League teams with stats
- **matches**: Match fixtures and results
- **players**: Player statistics
- **predictions**: Model predictions with probabilities
- **model_versions**: Track different model versions

## Model Features (16 total)

1. home_avg_xg
2. away_avg_xg
3. xg_differential
4. home_possession
5. away_possession
6. possession_differential
7. home_shots_on_target
8. away_shots_on_target
9. home_goals_for
10. away_goals_for
11. home_goals_against
12. away_goals_against
13. home_form_points
14. away_form_points
15. form_differential
16. head_to_head_ratio

## Prediction Classes

- **0**: Draw
- **1**: Home Win
- **2**: Away Win

## Current Development Phase

Check `ROADMAP.md` for full details:
- **Phase 1**: Foundation & API Research (Weeks 1-2) ← **YOU ARE HERE**
- Phase 2: Architecture Design (Weeks 3-4)
- Phase 3: Backend Development (Weeks 5-10)
- Phase 4: ML Pipeline (Weeks 11-14)
- Phase 5: Data Pipeline (Weeks 15-17)
- Phase 6: Frontend (Weeks 18-21)
- Phase 7: Deployment (Weeks 22-24)
- Phase 8: Launch (Weeks 25-26)

## Next Immediate Steps

1. ✅ Install Rust → See `GETTING_STARTED.md`
2. ✅ Start Docker services
3. ⏳ Export ONNX model
4. ⏳ Research football APIs
5. ⏳ Complete Rust basics learning

## Helpful Links

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Football-Data.org API](https://www.football-data.org/)
- [API-Football](https://www.api-football.com/)

## Troubleshooting Quick Fixes

| Problem | Solution |
|---------|----------|
| Can't connect to DB | `docker-compose up -d postgres` |
| Rust command not found | Restart terminal after Rust install |
| Port 3000 already in use | `lsof -ti:3000 \| xargs kill` (Mac/Linux) |
| Model file not found | Run `export_model_to_onnx.py` |
| Compilation errors | `cargo clean && cargo check` |
