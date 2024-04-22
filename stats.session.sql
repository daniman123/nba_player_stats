CREATE TABLE Players (
    player_id INTEGER PRIMARY KEY,
    name TEXT
);

CREATE TABLE Seasons (
    season_id INTEGER PRIMARY KEY,
    year INTEGER,
    start_date DATE,
    end_date DATE
);

CREATE TABLE PlayerStats (
    stat_id INTEGER PRIMARY KEY,
    player_id INTEGER,
    season_id INTEGER,
    points INTEGER,
    rebounds INTEGER,
    assists INTEGER,
    FOREIGN KEY (player_id) REFERENCES Players(player_id),
    FOREIGN KEY (season_id) REFERENCES Seasons(season_id)
);

CREATE INDEX idx_player_id ON PlayerStats (player_id);
CREATE INDEX idx_season_id ON PlayerStats (season_id);
