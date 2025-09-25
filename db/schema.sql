-- This is a prototyping schema for now, it will change a lot.
-- TODO: migrations

CREATE TABLE artist (
	id INT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	name TEXT NOT NULL,
	slug TEXT NOT NULL,
	UNIQUE (slug)
);

CREATE TABLE track (
	id INT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	title TEXT NOT NULL,
	slug TEXT NOT NULL,
	length INT, -- allow null values for now
	UNIQUE (slug)
);

CREATE TABLE track_artist (
	track_id INT NOT NULL REFERENCES track,
	artist_id INT NOT NULL REFERENCES artist,
	credited_as TEXT,
	join_phrase TEXT,
	artist_order INT NOT NULL,
	PRIMARY KEY (track_id, artist_id)
);

CREATE TABLE album (
	id INT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	title TEXT NOT NULL,
	slug TEXT NOT NULL,
	UNIQUE (slug)
);

CREATE TABLE album_track (
	album_id INT NOT NULL REFERENCES album,
	track_id INT NOT NULL REFERENCES track,
	track_number INT NOT NULL,
	PRIMARY KEY (album_id, track_id)
);

CREATE TABLE album_artist (
	album_id INT NOT NULL REFERENCES album,
	artist_id INT NOT NULL REFERENCES artist,
	credited_as TEXT,
	join_phrase TEXT,
	artist_order INT NOT NULL,
	PRIMARY KEY (album_id, artist_id)
);

CREATE TABLE scrobble (
	utc_date_time TIMESTAMPTZ NOT NULL PRIMARY KEY,
	track_id INT NOT NULL REFERENCES track,
	album_id INT REFERENCES album,
	FOREIGN KEY (album_id, track_id) REFERENCES album_track (album_id, track_id)
);
