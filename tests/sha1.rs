use std::fs::{read_dir, File};
use std::io::Error as IoError;

use assert_fs::fixture::FixtureError;
use assert_fs::prelude::{FileTouch, FileWriteBin, PathChild};
use assert_fs::TempDir;
use chksum_sha1::{chksum, Error as ChksumError};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    ChksumError(#[from] ChksumError),
    #[error(transparent)]
    FixtureError(#[from] FixtureError),
    #[error(transparent)]
    IoError(#[from] IoError),
}

#[test]
fn empty_directory_as_path() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;

    let dir = temp_dir.path();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn empty_directory_as_pathbuf() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;

    let dir = temp_dir.to_path_buf();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn empty_directory_as_readdir() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_path() -> Result<(), Error> {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    let dir = temp_dir.path();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_pathbuf() -> Result<(), Error> {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    let dir = temp_dir.to_path_buf();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn non_empty_directory_with_empty_file_as_readdir() -> Result<(), Error> {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        temp_dir.child("file.txt").touch()?;
        temp_dir
    };

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_path() -> Result<(), Error> {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    let dir = temp_dir.path();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_pathbuf() -> Result<(), Error> {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    let dir = temp_dir.to_path_buf();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    let dir = &temp_dir.to_path_buf();
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    Ok(())
}

#[test]
fn non_empty_directory_with_non_empty_file_as_readdir() -> Result<(), Error> {
    let temp_dir = {
        let temp_dir = TempDir::new()?;
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        temp_dir
    };

    let dir = read_dir(temp_dir.path())?;
    let digest = chksum(dir)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    Ok(())
}

#[test]
fn empty_file_as_path() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    let file = child.path();
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn empty_file_as_pathbuf() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    let file = child.to_path_buf();
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    let file = &child.to_path_buf();
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn empty_file_as_file() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file
    };

    let file = File::open(child.path())?;
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    let file = &File::open(child.path())?;
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    Ok(())
}

#[test]
fn non_empty_file_as_path() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    let file = child.path();
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    Ok(())
}

#[test]
fn non_empty_file_as_pathbuf() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    let file = child.to_path_buf();
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    let file = &child.to_path_buf();
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    Ok(())
}

#[test]
fn non_empty_file_as_file() -> Result<(), Error> {
    let temp_dir = TempDir::new()?;
    let child = {
        let file = temp_dir.child("file.txt");
        file.touch()?;
        file.write_binary(b"data")?;
        file
    };

    let file = File::open(child.path())?;
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    let file = &File::open(child.path())?;
    let digest = chksum(file)?.to_hex_lowercase();
    assert_eq!(digest, "a17c9aaa61e80a1bf71d0d850af4e5baa9800bbd");

    Ok(())
}
