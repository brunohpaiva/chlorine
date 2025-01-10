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
	-- this is a dumb way to store artists I think, need to think more about it
	main_artist_id INT NOT NULL REFERENCES artist,
	length INT, -- allow null values for now
	UNIQUE (slug)
);

CREATE TABLE track_artist (
	track_id INT NOT NULL REFERENCES track,
	artist_id INT NOT NULL REFERENCES artist,
	PRIMARY KEY (track_id, artist_id)
);

CREATE TABLE scrobble (
	utc_date_time TIMESTAMPTZ NOT NULL PRIMARY KEY,
	track_id INT NOT NULL REFERENCES track
);
