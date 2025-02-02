// use crate::types::{Directory, MoveMode};
// use rusqlite::Connection;
// use rusqlite_migration::{Migrations, M};
// use std::{
//     path::Path,
//     sync::{Arc, Mutex},
// };

// static BIN_ID: u32 = 1;

// fn cleanup(path: &str) {
//     if Path::new(path).exists() {
//         // Delete db.sqlite.
//         std::fs::remove_file(path).unwrap();
//     }
// }

// fn connect(path: &str, url: &str) -> Arc<Mutex<Connection>> {
//     std::fs::File::create(path).unwrap();

//     let mut conn = Connection::open(url).unwrap();

//     let migrations = Migrations::new(vec![M::up(include_str!("../../state/schema.sql"))]);

//     migrations.to_latest(&mut conn).unwrap();

//     return Arc::new(Mutex::new(conn));
// }

// #[test]
// fn test_directory_insert() {
//     const DB_PATH: &str = "./test_directory_insert.sqlite";
//     const DB_URL: &str = "sqlite://../test_directory_insert.sqlite";

//     cleanup(DB_PATH);

//     let conn = connect(DB_PATH, DB_URL);

//     // Insert some directories.
//     crate::db::directory::insert(2, "T1").unwrap(); // Gets id 3.
//     crate::db::directory::insert(3, "T1.1").unwrap(); // Gets id 4.
//     crate::db::directory::insert(2, "T2").unwrap(); // Gets id 5.

//     let expected_result = Directory {
//         r#type: "directory".into(),
//         id: 0,
//         name: "../stores".into(),
//         lft: 1,
//         rgt: 12,
//         files: vec![],
//         subdirectories: vec![
//             Directory {
//                 r#type: "directory".into(),
//                 id: 1,
//                 name: "Bin".into(),
//                 lft: 2,
//                 rgt: 3,
//                 files: vec![],
//                 subdirectories: vec![],
//             },
//             Directory {
//                 r#type: "directory".into(),
//                 id: 2,
//                 name: "Local Storage".into(),
//                 lft: 4,
//                 rgt: 11,
//                 files: vec![],
//                 subdirectories: vec![
//                     Directory {
//                         r#type: "directory".into(),
//                         id: 3,
//                         name: "T1".into(),
//                         lft: 5,
//                         rgt: 8,
//                         files: vec![],
//                         subdirectories: vec![Directory {
//                             r#type: "directory".into(),
//                             id: 4,
//                             name: "T1.1".into(),
//                             lft: 6,
//                             rgt: 7,
//                             files: vec![],
//                             subdirectories: vec![],
//                         }],
//                     },
//                     Directory {
//                         r#type: "directory".into(),
//                         id: 5,
//                         name: "T2".into(),
//                         lft: 9,
//                         rgt: 10,
//                         files: vec![],
//                         subdirectories: vec![],
//                     },
//                 ],
//             },
//         ],
//     };

//     let actual_result = crate::db::general::get_registry().unwrap();

//     assert_eq!(expected_result, actual_result);

//     cleanup(DB_PATH);
// }

// #[test]
// fn test_directory_move_1() {
//     const DB_PATH: &str = "./test_directory_move_1.sqlite";
//     const DB_URL: &str = "sqlite://../test_directory_move_1.sqlite";

//     cleanup(DB_PATH);

//     let conn = connect(DB_PATH, DB_URL);

//     // Insert some directories.
//     crate::db::directory::insert(2, "T1").unwrap(); // Gets id 3.
//     crate::db::directory::insert(3, "T1.1").unwrap(); // Gets id 4.
//     crate::db::directory::insert(2, "T2").unwrap(); // Gets id 5.

//     // Move T1 inside of T2.
//     crate::db::directory::r#move(3, 5, MoveMode::Regular).unwrap();

//     let expected_result = Directory {
//         r#type: "directory".into(),
//         id: 0,
//         name: "../stores".into(),
//         lft: 1,
//         rgt: 12,
//         files: vec![],
//         subdirectories: vec![
//             Directory {
//                 r#type: "directory".into(),
//                 id: 1,
//                 name: "Bin".into(),
//                 lft: 2,
//                 rgt: 3,
//                 files: vec![],
//                 subdirectories: vec![],
//             },
//             Directory {
//                 r#type: "directory".into(),
//                 id: 2,
//                 name: "Local Storage".into(),
//                 lft: 4,
//                 rgt: 11,
//                 files: vec![],
//                 subdirectories: vec![Directory {
//                     r#type: "directory".into(),
//                     id: 5,
//                     name: "T2".into(),
//                     lft: 5,
//                     rgt: 10,
//                     files: vec![],
//                     subdirectories: vec![Directory {
//                         r#type: "directory".into(),
//                         id: 3,
//                         name: "T1".into(),
//                         lft: 6,
//                         rgt: 9,
//                         files: vec![],
//                         subdirectories: vec![Directory {
//                             r#type: "directory".into(),
//                             id: 4,
//                             name: "T1.1".into(),
//                             lft: 7,
//                             rgt: 8,
//                             files: vec![],
//                             subdirectories: vec![],
//                         }],
//                     }],
//                 }],
//             },
//         ],
//     };

//     let actual_result = crate::db::general::get_registry().unwrap();

//     assert_eq!(expected_result, actual_result);

//     cleanup(DB_PATH);
// }

// #[test]
// fn test_directory_move_2() {
//     const DB_PATH: &str = "./test_directory_move_2.sqlite";
//     const DB_URL: &str = "sqlite://../test_directory_move_2.sqlite";

//     cleanup(DB_PATH);

//     let conn = connect(DB_PATH, DB_URL);

//     // Insert some directories.
//     crate::db::directory::insert(2, "T1").unwrap(); // Gets id 3.
//     crate::db::directory::insert(2, "T2").unwrap(); // Gets id 4.
//     crate::db::directory::insert(3, "T1.1").unwrap(); // Gets id 5.
//     crate::db::directory::insert(2, "T3").unwrap(); // Gets id 6.
//     crate::db::directory::insert(4, "T2.1").unwrap(); // Gets id 7.
//     crate::db::directory::insert(4, "T2.2").unwrap(); // Gets id 8.
//     crate::db::directory::insert(7, "T2.1.1").unwrap(); // Gets id 9.
//     crate::db::directory::insert(6, "T3.1").unwrap(); // Gets id 10.

//     // Move T2 inside of T3.1.
//     crate::db::directory::r#move(4, 10, MoveMode::Regular).unwrap();

//     let expected_result = Directory {
//         r#type: "directory".into(),
//         id: 0,
//         name: "../stores".into(),
//         lft: 1,
//         rgt: 22,
//         files: vec![],
//         subdirectories: vec![
//             Directory {
//                 r#type: "directory".into(),
//                 id: 1,
//                 name: "Bin".into(),
//                 lft: 2,
//                 rgt: 3,
//                 files: vec![],
//                 subdirectories: vec![],
//             },
//             Directory {
//                 r#type: "directory".into(),
//                 id: 2,
//                 name: "Local Storage".into(),
//                 lft: 4,
//                 rgt: 21,
//                 files: vec![],
//                 subdirectories: vec![
//                     Directory {
//                         r#type: "directory".into(),
//                         id: 3,
//                         name: "T1".into(),
//                         lft: 5,
//                         rgt: 8,
//                         files: vec![],
//                         subdirectories: vec![Directory {
//                             r#type: "directory".into(),
//                             id: 5,
//                             name: "T1.1".into(),
//                             lft: 6,
//                             rgt: 7,
//                             files: vec![],
//                             subdirectories: vec![],
//                         }],
//                     },
//                     Directory {
//                         r#type: "directory".into(),
//                         id: 6,
//                         name: "T3".into(),
//                         lft: 9,
//                         rgt: 20,
//                         files: vec![],
//                         subdirectories: vec![Directory {
//                             r#type: "directory".into(),
//                             id: 10,
//                             name: "T3.1".into(),
//                             lft: 10,
//                             rgt: 19,
//                             files: vec![],
//                             subdirectories: vec![Directory {
//                                 r#type: "directory".into(),
//                                 id: 4,
//                                 name: "T2".into(),
//                                 lft: 11,
//                                 rgt: 18,
//                                 files: vec![],
//                                 subdirectories: vec![
//                                     Directory {
//                                         r#type: "directory".into(),
//                                         id: 7,
//                                         name: "T2.1".into(),
//                                         lft: 12,
//                                         rgt: 15,
//                                         files: vec![],
//                                         subdirectories: vec![Directory {
//                                             r#type: "directory".into(),
//                                             id: 9,
//                                             name: "T2.1.1".into(),
//                                             lft: 13,
//                                             rgt: 14,
//                                             files: vec![],
//                                             subdirectories: vec![],
//                                         }],
//                                     },
//                                     Directory {
//                                         r#type: "directory".into(),
//                                         id: 8,
//                                         name: "T2.2".into(),
//                                         lft: 16,
//                                         rgt: 17,
//                                         files: vec![],
//                                         subdirectories: vec![],
//                                     },
//                                 ],
//                             }],
//                         }],
//                     },
//                 ],
//             },
//         ],
//     };

//     let actual_result = crate::db::general::get_registry().unwrap();

//     assert_eq!(expected_result, actual_result);

//     cleanup(DB_PATH);
// }

// #[test]
// fn test_directory_soft_delete() {
//     const DB_PATH: &str = "./test_directory_soft_delete.sqlite";
//     const DB_URL: &str = "sqlite://../test_directory_soft_delete.sqlite";

//     cleanup(DB_PATH);

//     let conn = connect(DB_PATH, DB_URL);

//     // Insert some directories.
//     crate::db::directory::insert(2, "T1").unwrap(); // Gets id 3.
//     crate::db::directory::insert(3, "T1.1").unwrap(); // Gets id 4.
//     crate::db::directory::insert(2, "T2").unwrap(); // Gets id 5.

//     // Soft delete T1.
//     crate::db::directory::r#move(3, BIN_ID, MoveMode::SoftDelete).unwrap();

//     let expected_result = Directory {
//         r#type: "directory".into(),
//         id: 0,
//         name: "../stores".into(),
//         lft: 1,
//         rgt: 12,
//         files: vec![],
//         subdirectories: vec![
//             Directory {
//                 r#type: "directory".into(),
//                 id: 1,
//                 name: "Bin".into(),
//                 lft: 2,
//                 rgt: 7,
//                 files: vec![],
//                 subdirectories: vec![Directory {
//                     r#type: "directory".into(),
//                     id: 3,
//                     name: "T1".into(),
//                     lft: 3,
//                     rgt: 6,
//                     files: vec![],
//                     subdirectories: vec![Directory {
//                         r#type: "directory".into(),
//                         id: 4,
//                         name: "T1.1".into(),
//                         lft: 4,
//                         rgt: 5,
//                         files: vec![],
//                         subdirectories: vec![],
//                     }],
//                 }],
//             },
//             Directory {
//                 r#type: "directory".into(),
//                 id: 2,
//                 name: "Local Storage".into(),
//                 lft: 8,
//                 rgt: 11,
//                 files: vec![],
//                 subdirectories: vec![Directory {
//                     r#type: "directory".into(),
//                     id: 5,
//                     name: "T2".into(),
//                     lft: 9,
//                     rgt: 10,
//                     files: vec![],
//                     subdirectories: vec![],
//                 }],
//             },
//         ],
//     };

//     let actual_result = crate::db::general::get_registry().unwrap();

//     assert_eq!(expected_result, actual_result);

//     cleanup(DB_PATH);
// }

// #[test]
// fn test_directory_hard_delete() {
//     const DB_PATH: &str = "./test_directory_hard_delete.sqlite";
//     const DB_URL: &str = "sqlite://../test_directory_hard_delete.sqlite";

//     cleanup(DB_PATH);

//     let conn = connect(DB_PATH, DB_URL);

//     // Insert some directories.
//     crate::db::directory::insert(2, "T1").unwrap(); // Gets id 3.
//     crate::db::directory::insert(3, "T1.1").unwrap(); // Gets id 4.
//     crate::db::directory::insert(2, "T2").unwrap(); // Gets id 5.

//     // Delete T1.
//     crate::db::directory::delete(3).unwrap();

//     let expected_result = Directory {
//         r#type: "directory".into(),
//         id: 0,
//         name: "../stores".into(),
//         lft: 1,
//         rgt: 8,
//         files: vec![],
//         subdirectories: vec![
//             Directory {
//                 r#type: "directory".into(),
//                 id: 1,
//                 name: "Bin".into(),
//                 lft: 2,
//                 rgt: 3,
//                 files: vec![],
//                 subdirectories: vec![],
//             },
//             Directory {
//                 r#type: "directory".into(),
//                 id: 2,
//                 name: "Local Storage".into(),
//                 lft: 4,
//                 rgt: 7,
//                 files: vec![],
//                 subdirectories: vec![Directory {
//                     r#type: "directory".into(),
//                     id: 5,
//                     name: "T2".into(),
//                     lft: 5,
//                     rgt: 6,
//                     files: vec![],
//                     subdirectories: vec![],
//                 }],
//             },
//         ],
//     };

//     let actual_result = crate::db::general::get_registry().unwrap();

//     assert_eq!(expected_result, actual_result);

//     cleanup(DB_PATH);
// }
