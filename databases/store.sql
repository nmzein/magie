CREATE TABLE IF NOT EXISTS id_counter (next_id INTEGER NOT NULL);

INSERT OR IGNORE INTO id_counter (next_id) VALUES (1);

CREATE TABLE IF NOT EXISTS directories (
    id INTEGER PRIMARY KEY,
    parent_id INTEGER,
    predeletion_parent_id INTEGER,
    name TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES directories (id) ON DELETE CASCADE,
    UNIQUE (parent_id, name) -- Enforce that for each parent directory, there is only one directory with a given name.
);

CREATE TABLE IF NOT EXISTS images (
    id INTEGER PRIMARY KEY,
    parent_id INTEGER NOT NULL,
    predeletion_parent_id INTEGER,
    name TEXT NOT NULL,
    created_at DATETIME,
    updated_at DATETIME,
    decoder TEXT,
    encoder TEXT NOT NULL,
    generator TEXT,
    uploaded_image_extension TEXT NOT NULL,
    uploaded_annotations_extension TEXT,
    FOREIGN KEY (parent_id) REFERENCES directories (id) ON DELETE CASCADE,
    UNIQUE (parent_id, name) -- Enforce that for each parent directory, there is only one file with a given name.
);

CREATE TABLE IF NOT EXISTS metadata_layer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image_id INTEGER NOT NULL,
    level INTEGER NOT NULL,
    cols INTEGER NOT NULL,
    rows INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
    UNIQUE (image_id, level)
);

CREATE TABLE IF NOT EXISTS annotation_layer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    colour TEXT NOT NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
    UNIQUE (image_id, tag)
);
