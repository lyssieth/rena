use crate::{run, Arguments};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use regex::Regex;
use tempfile::tempdir;

fn setup_files_for_test(path: &Path, filenames: &[&str]) {
    for x in filenames {
        File::create(path.join(x)).unwrap_or_else(|_| panic!("failed to create file {}", x));
    }
}

fn check_filenames(path: &Path, filenames: &[&str]) {
    for x in path.read_dir().unwrap() {
        dbg!(x.unwrap());
    }

    for x in filenames {
        assert!(path.join(x).exists(), "file {} does not exist", x);
    }
}

// rena images/
#[test]
fn simple_rename() {
    const SIMPLE_FILENAMES: &[&str] =
        &["image.jpg", "image3.jpg", "12746uju21.jpg", "17f29a002.jpg"];
    const SIMPLE_EXPECTED: &[&str] = &[
        "item_0000000000.jpg",
        "item_0000000001.jpg",
        "item_0000000002.jpg",
        "item_0000000003.jpg",
    ];

    let path = tempdir().expect("failed to obtain temporary directory");
    setup_files_for_test(path.path(), SIMPLE_FILENAMES);

    let args = Arguments {
        folder: PathBuf::from(path.path()),
        prefix: "item".to_owned(),
        padding: 10,
        ..Arguments::default()
    };

    let res = run(args);

    assert!(res.is_ok());

    check_filenames(path.path(), SIMPLE_EXPECTED);
}

// rena --match "\.[jpgn]+" images/
#[test]
fn regex_filtering() {
    const REGEX_FILENAMES: &[&str] = &[
        "image.jpg",
        "image3.mp4",
        "12746uju21.jpg",
        "17f29a002.jpg",
        "17f2121wss.png",
        "ffe_image_breaker.webm",
        "potential_effort.jpg",
    ];
    const REGEX_EXPECTED: &[&str] = &[
        "item_0000000000.jpg",
        "item_0000000001.png",
        "item_0000000002.jpg",
        "item_0000000003.jpg",
        "item_0000000004.jpg",
        "ffe_image_breaker.webm",
        "image3.mp4",
    ];

    let path = tempdir().expect("failed to obtain temporary directory");
    setup_files_for_test(path.path(), REGEX_FILENAMES);

    let args = Arguments {
        folder: PathBuf::from(path.path()),
        prefix: "item".to_owned(),
        padding: 10,
        match_regex: Some(Regex::new(r"\.[jpgn]+").expect("failed to compile regex")),
        ..Arguments::default()
    };

    let res = run(args);

    assert!(res.is_ok());

    check_filenames(path.path(), REGEX_EXPECTED);
}

// rena --match "Show\.S(\d+)E(\d+)\.1080p\.mkv" --match-rename "Show S${1} E${2} (1080p).mkv" Show/
#[test]
fn usage_for_shows() {
    const SHOWS_FILENAMES: &[&str] = &[
        "Show.S01E01.1080p.mkv",
        "Show.S01E02.1080p.mkv",
        "Show.S01E03.1080p.mkv",
        "Show.S02E01.1080p.mkv",
        "Show.S02E02.1080p.mkv",
        "Show.S02E03.1080p.mkv",
    ];
    const SHOWS_EXPECTED: &[&str] = &[
        "Show S01 E01 (1080p).mkv",
        "Show S01 E02 (1080p).mkv",
        "Show S01 E03 (1080p).mkv",
        "Show S02 E01 (1080p).mkv",
        "Show S02 E02 (1080p).mkv",
        "Show S02 E03 (1080p).mkv",
    ];

    let path = tempdir().expect("failed to obtain temporary directory");
    setup_files_for_test(path.path(), SHOWS_FILENAMES);

    let args = Arguments {
        folder: PathBuf::from(path.path()),
        match_regex: Some(
            Regex::new(r"Show\.S(\d+)E(\d+)\.1080p\.mkv").expect("failed to compile regex"),
        ),
        match_rename: Some(r"Show S${1} E${2} (1080p).mkv".to_owned()),
        ..Arguments::default()
    };

    let res = run(args);

    assert!(res.is_ok());

    check_filenames(path.path(), SHOWS_EXPECTED);
}
