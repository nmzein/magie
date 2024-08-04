use crate::types::{Directory, MoveMode};
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

fn cleanup(path: &str) {
    if Path::new(path).exists() {
        // Delete db.sqlite.
        std::fs::remove_file(path).unwrap();
    }
}

fn connect(path: &str, url: &str) -> Arc<Mutex<Connection>> {
    std::fs::File::create(path).unwrap();

    let mut conn = Connection::open(url).unwrap();

    let migrations = Migrations::new(vec![M::up(include_str!("../../state/schema.sql"))]);

    migrations.to_latest(&mut conn).unwrap();

    return Arc::new(Mutex::new(conn));
}

#[test]
fn test_directory_insert() {
    const DB_PATH: &str = "./test_directory_insert.sqlite";
    const DB_URL: &str = "sqlite://../test_directory_insert.sqlite";

    cleanup(DB_PATH);

    let conn = connect(DB_PATH, DB_URL);

    // Insert some directories.
    crate::db::directory::insert(2, "T1", Arc::clone(&conn)).unwrap(); // Gets id 3.
    crate::db::directory::insert(3, "T1.1", Arc::clone(&conn)).unwrap(); // Gets id 4.
    crate::db::directory::insert(2, "T2", Arc::clone(&conn)).unwrap(); // Gets id 5.

    let expected_result = Directory {
        id: 0,
        name: "../stores".to_owned(),
        lft: 1,
        rgt: 12,
        files: vec![],
        subdirectories: vec![
            Directory {
                id: 1,
                name: "Bin".to_owned(),
                lft: 2,
                rgt: 3,
                files: vec![],
                subdirectories: vec![],
            },
            Directory {
                id: 2,
                name: "Local Storage".to_owned(),
                lft: 4,
                rgt: 11,
                files: vec![],
                subdirectories: vec![
                    Directory {
                        id: 3,
                        name: "T1".to_owned(),
                        lft: 5,
                        rgt: 8,
                        files: vec![],
                        subdirectories: vec![Directory {
                            id: 4,
                            name: "T1.1".to_owned(),
                            lft: 6,
                            rgt: 7,
                            files: vec![],
                            subdirectories: vec![],
                        }],
                    },
                    Directory {
                        id: 5,
                        name: "T2".to_owned(),
                        lft: 9,
                        rgt: 10,
                        files: vec![],
                        subdirectories: vec![],
                    },
                ],
            },
        ],
    };

    let actual_result = crate::db::general::get_registry(Arc::clone(&conn)).unwrap();

    assert_eq!(expected_result, actual_result);

    cleanup(DB_PATH);
}

#[test]
fn test_directory_move_1() {
    const DB_PATH: &str = "./test_directory_move_1.sqlite";
    const DB_URL: &str = "sqlite://../test_directory_move_1.sqlite";

    cleanup(DB_PATH);

    let conn = connect(DB_PATH, DB_URL);

    // Insert some directories.
    crate::db::directory::insert(2, "T1", Arc::clone(&conn)).unwrap(); // Gets id 3.
    crate::db::directory::insert(3, "T1.1", Arc::clone(&conn)).unwrap(); // Gets id 4.
    crate::db::directory::insert(2, "T2", Arc::clone(&conn)).unwrap(); // Gets id 5.

    // Move T1 inside of T2.
    crate::db::directory::r#move(3, 5, MoveMode::Regular, Arc::clone(&conn)).unwrap();

    let expected_result = Directory {
        id: 0,
        name: "../stores".to_owned(),
        lft: 1,
        rgt: 12,
        files: vec![],
        subdirectories: vec![
            Directory {
                id: 1,
                name: "Bin".to_owned(),
                lft: 2,
                rgt: 3,
                files: vec![],
                subdirectories: vec![],
            },
            Directory {
                id: 2,
                name: "Local Storage".to_owned(),
                lft: 4,
                rgt: 11,
                files: vec![],
                subdirectories: vec![Directory {
                    id: 5,
                    name: "T2".to_owned(),
                    lft: 5,
                    rgt: 10,
                    files: vec![],
                    subdirectories: vec![Directory {
                        id: 3,
                        name: "T1".to_owned(),
                        lft: 6,
                        rgt: 9,
                        files: vec![],
                        subdirectories: vec![Directory {
                            id: 4,
                            name: "T1.1".to_owned(),
                            lft: 7,
                            rgt: 8,
                            files: vec![],
                            subdirectories: vec![],
                        }],
                    }],
                }],
            },
        ],
    };

    let actual_result = crate::db::general::get_registry(Arc::clone(&conn)).unwrap();

    assert_eq!(expected_result, actual_result);

    cleanup(DB_PATH);
}

#[test]
fn test_directory_move_2() {
    const DB_PATH: &str = "./test_directory_move_2.sqlite";
    const DB_URL: &str = "sqlite://../test_directory_move_2.sqlite";

    cleanup(DB_PATH);

    let conn = connect(DB_PATH, DB_URL);

    // Insert some directories.
    crate::db::directory::insert(2, "T1", Arc::clone(&conn)).unwrap(); // Gets id 3.
    crate::db::directory::insert(2, "T2", Arc::clone(&conn)).unwrap(); // Gets id 4.
    crate::db::directory::insert(3, "T1.1", Arc::clone(&conn)).unwrap(); // Gets id 5.
    crate::db::directory::insert(2, "T3", Arc::clone(&conn)).unwrap(); // Gets id 6.
    crate::db::directory::insert(4, "T2.1", Arc::clone(&conn)).unwrap(); // Gets id 7.
    crate::db::directory::insert(4, "T2.2", Arc::clone(&conn)).unwrap(); // Gets id 8.
    crate::db::directory::insert(7, "T2.1.1", Arc::clone(&conn)).unwrap(); // Gets id 9.
    crate::db::directory::insert(6, "T3.1", Arc::clone(&conn)).unwrap(); // Gets id 10.

    // Move T2 inside of T3.1.
    crate::db::directory::r#move(4, 10, MoveMode::Regular, Arc::clone(&conn)).unwrap();

    let expected_result = Directory {
        id: 0,
        name: "../stores".to_owned(),
        lft: 1,
        rgt: 22,
        files: vec![],
        subdirectories: vec![
            Directory {
                id: 1,
                name: "Bin".to_owned(),
                lft: 2,
                rgt: 3,
                files: vec![],
                subdirectories: vec![],
            },
            Directory {
                id: 2,
                name: "Local Storage".to_owned(),
                lft: 4,
                rgt: 21,
                files: vec![],
                subdirectories: vec![
                    Directory {
                        id: 3,
                        name: "T1".to_owned(),
                        lft: 5,
                        rgt: 8,
                        files: vec![],
                        subdirectories: vec![Directory {
                            id: 5,
                            name: "T1.1".to_owned(),
                            lft: 6,
                            rgt: 7,
                            files: vec![],
                            subdirectories: vec![],
                        }],
                    },
                    Directory {
                        id: 6,
                        name: "T3".to_owned(),
                        lft: 9,
                        rgt: 20,
                        files: vec![],
                        subdirectories: vec![Directory {
                            id: 10,
                            name: "T3.1".to_owned(),
                            lft: 10,
                            rgt: 19,
                            files: vec![],
                            subdirectories: vec![Directory {
                                id: 4,
                                name: "T2".to_owned(),
                                lft: 11,
                                rgt: 18,
                                files: vec![],
                                subdirectories: vec![
                                    Directory {
                                        id: 7,
                                        name: "T2.1".to_owned(),
                                        lft: 12,
                                        rgt: 15,
                                        files: vec![],
                                        subdirectories: vec![Directory {
                                            id: 9,
                                            name: "T2.1.1".to_owned(),
                                            lft: 13,
                                            rgt: 14,
                                            files: vec![],
                                            subdirectories: vec![],
                                        }],
                                    },
                                    Directory {
                                        id: 8,
                                        name: "T2.2".to_owned(),
                                        lft: 16,
                                        rgt: 17,
                                        files: vec![],
                                        subdirectories: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                ],
            },
        ],
    };

    let actual_result = crate::db::general::get_registry(Arc::clone(&conn)).unwrap();

    assert_eq!(expected_result, actual_result);

    cleanup(DB_PATH);
}

#[test]
fn test_directory_delete() {
    const DB_PATH: &str = "./test_directory_delete.sqlite";
    const DB_URL: &str = "sqlite://../test_directory_delete.sqlite";

    cleanup(DB_PATH);

    let conn = connect(DB_PATH, DB_URL);

    // Insert some directories.
    crate::db::directory::insert(2, "T1", Arc::clone(&conn)).unwrap(); // Gets id 3.
    crate::db::directory::insert(3, "T1.1", Arc::clone(&conn)).unwrap(); // Gets id 4.
    crate::db::directory::insert(2, "T2", Arc::clone(&conn)).unwrap(); // Gets id 5.

    // Delete T1.
    crate::db::directory::delete(3, Arc::clone(&conn)).unwrap();

    let expected_result = Directory {
        id: 0,
        name: "../stores".to_owned(),
        lft: 1,
        rgt: 8,
        files: vec![],
        subdirectories: vec![
            Directory {
                id: 1,
                name: "Bin".to_owned(),
                lft: 2,
                rgt: 3,
                files: vec![],
                subdirectories: vec![],
            },
            Directory {
                id: 2,
                name: "Local Storage".to_owned(),
                lft: 4,
                rgt: 7,
                files: vec![],
                subdirectories: vec![Directory {
                    id: 5,
                    name: "T2".to_owned(),
                    lft: 5,
                    rgt: 6,
                    files: vec![],
                    subdirectories: vec![],
                }],
            },
        ],
    };

    let actual_result = crate::db::general::get_registry(Arc::clone(&conn)).unwrap();

    assert_eq!(expected_result, actual_result);

    cleanup(DB_PATH);
}
