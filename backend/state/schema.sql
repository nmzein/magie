CREATE TABLE IF NOT EXISTS directories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_id INTEGER,
  predeletion_parent_id INTEGER,
  name TEXT NOT NULL,
  lft INTEGER NOT NULL,
  rgt INTEGER NOT NULL,
  FOREIGN KEY(parent_id) REFERENCES directories(id) ON DELETE CASCADE,
  UNIQUE(parent_id, name) -- Enforce that for each parent directory, there is only one directory with a given name.
);

INSERT OR IGNORE INTO directories (id, parent_id, name, lft, rgt) VALUES (0, NULL, '../stores', 1, 6); -- '../stores'
INSERT OR IGNORE INTO directories (id, parent_id, name, lft, rgt) VALUES (1, 0, 'Bin', 2, 3); -- '../stores/Bin'
INSERT OR IGNORE INTO directories (id, parent_id, name, lft, rgt) VALUES (2, 0, 'Local Storage', 4, 5); -- '../stores/Local Storage'

CREATE TABLE IF NOT EXISTS images (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_id INTEGER NOT NULL,
  predeletion_parent_id INTEGER,
  name TEXT NOT NULL,
  upl_img_ext TEXT NOT NULL,
  enc_img_ext TEXT NOT NULL,
  upl_img_fmt TEXT NOT NULL,
  enc_img_fmt TEXT NOT NULL,
  upl_anno_ext TEXT,
  FOREIGN KEY(parent_id) REFERENCES directories(id) ON DELETE CASCADE,
  UNIQUE(parent_id, name) -- Enforce that for each parent directory, there is only one file with a given name.
);

CREATE TABLE IF NOT EXISTS metadata_layer (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  image_id INTEGER NOT NULL,
  level INTEGER NOT NULL,
  cols INTEGER NOT NULL,
  rows INTEGER NOT NULL,
  width INTEGER NOT NULL,
  height INTEGER NOT NULL,
  FOREIGN KEY(image_id) REFERENCES images(id) ON DELETE CASCADE,
  UNIQUE(image_id, level)
);

CREATE TABLE IF NOT EXISTS annotation_layer (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  image_id INTEGER NOT NULL,
  tag TEXT NOT NULL,
  FOREIGN KEY(image_id) REFERENCES images(id) ON DELETE CASCADE,
  UNIQUE(image_id, tag)
);