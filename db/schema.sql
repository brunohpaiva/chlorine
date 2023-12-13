CREATE TABLE artist (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE album (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE album_artist (
    album_id INT NOT NULL REFERENCES album(id),
    artist_id INT NOT NULL REFERENCES artist(id),
    PRIMARY KEY (album_id, artist_id)
);

CREATE TABLE track (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    name TEXT NOT NULL,
    album_id INT NOT NULL REFERENCES album(id),
    length INT NOT NULL
);

CREATE TABLE track_artist (
    track_id INT NOT NULL REFERENCES track(id),
    artist_id INT NOT NULL REFERENCES artist(id),
    PRIMARY KEY (track_id, artist_id)
);

CREATE TABLE scrobble (
    timestamp TIMESTAMPTZ NOT NULL PRIMARY KEY,
    track_id INT NOT NULL REFERENCES track(id),
    duration INT,
    client TEXT NOT NULL,
    raw_data JSON
);

