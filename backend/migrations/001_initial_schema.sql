-- Initial database schema for Premier League Predictor

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Teams table
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    short_name VARCHAR(10),
    logo_url TEXT,

    -- Season statistics
    matches_played INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    draws INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    goals_for INTEGER DEFAULT 0,
    goals_against INTEGER DEFAULT 0,
    points INTEGER DEFAULT 0,

    -- Advanced metrics (nullable for historical teams)
    avg_xg DOUBLE PRECISION,
    avg_xg_against DOUBLE PRECISION,
    avg_possession DOUBLE PRECISION,
    avg_shots DOUBLE PRECISION,
    avg_shots_on_target DOUBLE PRECISION,

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Matches table
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    home_team_id UUID NOT NULL REFERENCES teams(id),
    away_team_id UUID NOT NULL REFERENCES teams(id),
    match_date TIMESTAMP WITH TIME ZONE NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'scheduled',
    season VARCHAR(10) NOT NULL, -- e.g., '2023-24'
    gameweek INTEGER NOT NULL,

    -- Score (null if not finished)
    home_score INTEGER,
    away_score INTEGER,

    -- Match statistics (populated during/after match)
    home_xg DOUBLE PRECISION,
    away_xg DOUBLE PRECISION,
    home_possession DOUBLE PRECISION,
    away_possession DOUBLE PRECISION,
    home_shots INTEGER,
    away_shots INTEGER,
    home_shots_on_target INTEGER,
    away_shots_on_target INTEGER,

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT different_teams CHECK (home_team_id != away_team_id)
);

-- Players table
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    team_id UUID NOT NULL REFERENCES teams(id),
    name VARCHAR(255) NOT NULL,
    position VARCHAR(50),

    -- Season statistics
    minutes_played INTEGER DEFAULT 0,
    goals INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    xg DOUBLE PRECISION,
    xa DOUBLE PRECISION,
    goals_per_90 DOUBLE PRECISION,

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Predictions table
CREATE TABLE predictions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    match_id UUID NOT NULL REFERENCES matches(id),
    model_version VARCHAR(50) NOT NULL,

    -- Prediction probabilities
    prob_home_win DOUBLE PRECISION NOT NULL,
    prob_draw DOUBLE PRECISION NOT NULL,
    prob_away_win DOUBLE PRECISION NOT NULL,

    -- Predicted outcome
    predicted_result INTEGER NOT NULL, -- 0=Draw, 1=Home, 2=Away
    confidence DOUBLE PRECISION NOT NULL,

    -- Actual result (filled after match)
    actual_result INTEGER,
    was_correct BOOLEAN,

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT valid_probabilities CHECK (
        prob_home_win >= 0 AND prob_home_win <= 1 AND
        prob_draw >= 0 AND prob_draw <= 1 AND
        prob_away_win >= 0 AND prob_away_win <= 1
    ),
    CONSTRAINT valid_prediction CHECK (predicted_result IN (0, 1, 2)),
    CONSTRAINT valid_actual CHECK (actual_result IS NULL OR actual_result IN (0, 1, 2))
);

-- Model versions table
CREATE TABLE model_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    version VARCHAR(50) NOT NULL UNIQUE,
    model_type VARCHAR(50) NOT NULL,
    features TEXT[], -- Array of feature names
    accuracy DOUBLE PRECISION,
    notes TEXT,
    is_active BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_matches_date ON matches(match_date);
CREATE INDEX idx_matches_status ON matches(status);
CREATE INDEX idx_matches_season ON matches(season);
CREATE INDEX idx_matches_home_team ON matches(home_team_id);
CREATE INDEX idx_matches_away_team ON matches(away_team_id);
CREATE INDEX idx_predictions_match ON predictions(match_id);
CREATE INDEX idx_players_team ON players(team_id);
CREATE INDEX idx_teams_name ON teams(name);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to auto-update updated_at
CREATE TRIGGER update_teams_updated_at
    BEFORE UPDATE ON teams
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_matches_updated_at
    BEFORE UPDATE ON matches
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_players_updated_at
    BEFORE UPDATE ON players
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
