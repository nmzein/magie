
INSERT OR IGNORE INTO images
(id, parent_id, name, upl_img_ext, enc_img_ext, upl_img_fmt, enc_img_fmt, upl_anno_ext) VALUES
(1, 2, 'hyperplastic1', 'tiff', 'zarr', 'tiff', 'omezarr', 'db');

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(1, 1, 0, 56, 106, 57152, 107952);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(2, 1, 1, 28, 53, 28576, 53976);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(3, 1, 2, 14, 27, 14288, 26988);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(4, 1, 3, 7, 14, 7144, 13494);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(5, 1, 4, 4, 7, 3572, 6747);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(6, 1, 5, 2, 4, 1786, 3373);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(7, 1, 6, 1, 2, 893, 1686);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(8, 1, 7, 1, 1, 446, 843);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(9, 1, 8, 1, 1, 223, 421);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(10, 1, 9, 1, 1, 111, 210);

INSERT OR IGNORE INTO metadata_layer
(id, image_id, level, cols, rows, width, height) VALUES
(11, 1, 10, 1, 1, 55, 105);

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(1, 1, 'Gland');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(2, 1, 'Lumen');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(3, 1, 'Epithelial Cell');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(4, 1, 'Lymphocyte');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(5, 1, 'Neutrophil');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(6, 1, 'Connective Cell');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(7, 1, 'Plasma Cell');

INSERT OR IGNORE INTO annotation_layer
(id, image_id, tag) VALUES
(8, 1, 'Eosinophil');

