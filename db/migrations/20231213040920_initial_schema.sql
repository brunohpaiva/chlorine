-- Create "album" table
CREATE TABLE "public"."album" ("id" integer NOT NULL GENERATED ALWAYS AS IDENTITY, "name" text NOT NULL, PRIMARY KEY ("id"));
-- Create "artist" table
CREATE TABLE "public"."artist" ("id" integer NOT NULL GENERATED ALWAYS AS IDENTITY, "name" text NOT NULL, PRIMARY KEY ("id"));
-- Create "album_artist" table
CREATE TABLE "public"."album_artist" ("album_id" integer NOT NULL, "artist_id" integer NOT NULL, PRIMARY KEY ("album_id", "artist_id"), CONSTRAINT "album_artist_album_id_fkey" FOREIGN KEY ("album_id") REFERENCES "public"."album" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT "album_artist_artist_id_fkey" FOREIGN KEY ("artist_id") REFERENCES "public"."artist" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "track" table
CREATE TABLE "public"."track" ("id" integer NOT NULL GENERATED ALWAYS AS IDENTITY, "name" text NOT NULL, "album_id" integer NOT NULL, "length" integer NOT NULL, PRIMARY KEY ("id"), CONSTRAINT "track_album_id_fkey" FOREIGN KEY ("album_id") REFERENCES "public"."album" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "scrobble" table
CREATE TABLE "public"."scrobble" ("timestamp" timestamptz NOT NULL, "track_id" integer NOT NULL, "duration" integer NULL, "client" text NOT NULL, "raw_data" json NULL, PRIMARY KEY ("timestamp"), CONSTRAINT "scrobble_track_id_fkey" FOREIGN KEY ("track_id") REFERENCES "public"."track" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "track_artist" table
CREATE TABLE "public"."track_artist" ("track_id" integer NOT NULL, "artist_id" integer NOT NULL, PRIMARY KEY ("track_id", "artist_id"), CONSTRAINT "track_artist_artist_id_fkey" FOREIGN KEY ("artist_id") REFERENCES "public"."artist" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT "track_artist_track_id_fkey" FOREIGN KEY ("track_id") REFERENCES "public"."track" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION);
