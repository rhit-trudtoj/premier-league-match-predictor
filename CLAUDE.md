# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository implements a **machine learning pipeline to predict Premier League match outcomes** (Home Win, Away Win, or Draw). The system uses a novel **synthetic feature generation** approach to combine current season detailed statistics (2023-24) with historical match data (1993-2022), overcoming the data availability gap between modern advanced metrics and historical basic statistics.

**Target Variable**: Match winner encoded as 0 (Draw), 1 (Home Win), 2 (Away Win)

## Architecture

### Core Innovation: Synthetic Feature Generation

The key architectural pattern is using a trained MultiOutputRegressor to generate advanced metrics for historical data that only contains basic match information:

1. Train a multi-output model on current season data to learn relationships between basic features and advanced metrics (xG, possession, ratings, etc.)
2. Use this model to predict synthetic advanced features for historical matches (1993-2022)
3. Combine enhanced historical data with current data for training, increasing dataset from ~380 to ~11,000+ matches

This is implemented in the `SyntheticFeatureGenerator` class, which is the architectural centerpiece of the project.

### Data Pipeline Workflow

1. **Data Loading** (Cells 2-9): Load and merge 14 team-level CSVs and 5 player-level CSVs from Kaggle dataset "premier-league-2324-team-and-player-insights"

2. **Player Feature Engineering** (Cells 10-13): Extract top players by statistics (xG, xA, goals per 90) and create binary features for teams with elite players (minimum 1500 minutes playing time)

3. **Baseline Data Integration** (Cells 14-25): Load current season matches (2023-24) and historical matches (1993-2022) from "english-premier-league-results" dataset

4. **Synthetic Feature Generation** (Cells 26-46): Train MultiOutputRegressor to predict 20+ advanced metrics for historical data using basic features

5. **Team Statistics Merging** (Cells 30-35): Join team statistics to match data, creating separate _home and _away features for each match

6. **Advanced Feature Engineering** (Cells 36-42): Create 50+ features including differentials (home - away), ratios (xG/Goals), and composite metrics (offensive/defensive ratings)

7. **Dataset Integration** (Cells 49-56): Align columns and concatenate enhanced historical data with current data

8. **Model Training** (Cells 57-68): Apply RFE to select top 16 features, use SMOTE for class balancing, and train KNeighborsClassifier with GridSearchCV hyperparameter tuning

9. **Evaluation** (Cells 69-74): Generate predictions, classification reports, and t-SNE visualizations

### Data Sources

**Current Season (2023-24)**: Team statistics (14 CSVs including xG, possession, ratings), player statistics (5 CSVs), and match results

**Historical (1993-2022)**: Basic match results only (HomeTeam, AwayTeam, Score) - encoding ISO-8859-1

All CSVs use 'Team' or 'Player' as merge keys. Match scores formatted as "X_Y" (underscore-separated).

### Key Classes and Functions

**`SyntheticFeatureGenerator`**: Main class for generating advanced metrics for historical data
- `_identify_base_features()`: Finds common columns between datasets
- `_identify_advanced_metrics()`: Identifies 20+ advanced statistics to generate
- `train_synthetic_model()`: Trains MultiOutputRegressor with RandomForest base (100 estimators)
- `generate_synthetic_features(model)`: Predicts synthetic features for historical matches

**`merge_team_stats_with_matches(matches_df, team_stats_df)`**: Joins team statistics to matches, creating _home and _away versions of all features

**`advanced_feature_engineering(df)`**: Creates differential, ratio, and composite features (20+ new features)

**`calculate_winner(df)`**: Parses score strings and assigns winner labels (0/1/2)

**`encode_teams_and_UTC(current_data, historical_data)`**: Label encodes team names consistently across datasets

**`optimize_model(X, y)`**: Runs GridSearchCV for KNeighborsClassifier hyperparameter tuning

### Model Architecture

**Primary Model**: K-Nearest Neighbors (selected over Random Forest, XGBoost, Gradient Boosting, Logistic Regression)
- Test accuracy: 47% (3-class classification, baseline ~33%)
- Hyperparameters tuned: n_neighbors, leaf_size, p, weights, metric
- Selected because RF/XGBoost showed 98%+ CV accuracy (likely overfitting)

**Feature Selection**: Recursive Feature Elimination (RFE) reduces 50+ features to 16 most important

**Class Balancing**: SMOTE applied to training data to handle Draw/Home Win/Away Win imbalance

**Synthetic Model**: MultiOutputRegressor with RandomForestRegressor (100 estimators, 5-fold CV)

### Key Configuration Parameters

```python
MIN_MINUTES = 1500  # Minimum playing time for top player consideration
TOP_N_PLAYERS = 3-5  # Number of top players tracked per statistic
TEST_SIZE = 0.2-0.3  # Train/test split
RANDOM_STATE = 42, 45, 50  # For reproducibility
N_FEATURES_TO_SELECT = 16  # RFE feature count
CV_FOLDS = 5  # Cross-validation folds
N_ESTIMATORS_SYNTH = 100  # Estimators for synthetic feature model
```

## Development Environment

This notebook is designed for **Kaggle environment** with paths using `/kaggle/input/`. When running locally, update all data loading paths accordingly.

The project is entirely contained in a single Jupyter notebook: [premier-league-predictor.ipynb](premier-league-predictor.ipynb)

### Dependencies

Core libraries:
- pandas, numpy (data manipulation)
- scikit-learn (complete ML pipeline: models, preprocessing, metrics, multioutput)
- xgboost (XGBClassifier)
- imblearn (SMOTE for class balancing)
- matplotlib, seaborn (visualization)

### Running the Notebook

Execute cells sequentially from top to bottom. The notebook has no external scripts or modules - all logic is self-contained.

Key execution notes:
- Cells 26-46 (synthetic feature generation) are computationally intensive
- GridSearchCV in cells 57-68 requires significant compute time
- Historical data requires ISO-8859-1 encoding when loading

## Current Performance and Limitations

**Test Performance**: 47% accuracy (vs 33% baseline for random guessing)
- Best at predicting: Home wins (93% recall, 63% F1)
- Worst at predicting: Draws (3% recall - severe difficulty)

**Known Limitations**:
- Kaggle-specific paths not portable without modification
- Models not persisted to disk
- Draw prediction remains very challenging despite SMOTE
- High CV scores suggest training overfitting despite modest test performance

**Architecture Strengths**:
- Synthetic feature generation enables unlimited historical data integration
- Modular design separates data loading, feature engineering, and modeling
- Reproducible with fixed random seeds
- Extensible for adding new statistics or features
