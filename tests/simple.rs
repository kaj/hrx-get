use hrx_get::Archive;

static DATA: &str = "<===> hello.md\
                     \n# Hello world\
                     \nThis is a simple markdown file.\
                     \n\
                     \n<===>\
                     \nThis is just a comment.\
                     \n<===> foo.txt\
                     \nThis is something else.\n";

static OTHER_DATA: &str = "<=====> hello.md\
                           \n# Hello world\
                           \nThis is a simple markdown file.\
                           \n\
                           \n<=====> dir/whatever\
                           \n<=====> foo.txt\
                           \nThis is something else.\n";

#[test]
fn get_names() {
    let archive = Archive::parse(DATA).unwrap();
    assert_eq!(archive.names(), ["foo.txt", "hello.md"])
}

#[test]
fn get_existing_nonlast() {
    let archive = Archive::parse(DATA).unwrap();
    assert_eq!(
        archive.get("hello.md").unwrap(),
        "# Hello world\nThis is a simple markdown file.\n"
    )
}

#[test]
fn get_existing_last() {
    let archive = Archive::parse(DATA).unwrap();
    assert_eq!(archive.get("foo.txt").unwrap(), "This is something else.\n")
}

#[test]
fn get_unexisting() {
    let archive = Archive::parse(DATA).unwrap();
    assert_eq!(archive.get("bar.txt"), None)
}

#[test]
fn get_other_existing() {
    let archive = Archive::parse(OTHER_DATA).unwrap();
    assert_eq!(
        archive.get("hello.md").unwrap(),
        "# Hello world\nThis is a simple markdown file.\n"
    )
}
#[test]
fn get_existing_empty() {
    let archive = Archive::parse(OTHER_DATA).unwrap();
    assert_eq!(archive.get("dir/whatever").unwrap(), "")
}

#[test]
fn load_file() {
    let archive = Archive::load("tests/sample.hrx".as_ref()).unwrap();
    assert_eq!(
        archive.get("file1").unwrap(),
        "This file doesn't have a trailng newline."
    )
}

#[test]
fn load_file_fail() {
    assert_eq!(
        Archive::load("no/such/file.hrx".as_ref())
            .unwrap_err()
            .to_string(),
        "Failed to read \"no/such/file.hrx\": No such file or directory (os error 2)"
    )
}

#[test]
fn load_file_bad_format() {
    assert_eq!(
        Archive::load("Cargo.toml".as_ref())
            .unwrap_err()
            .to_string(),
        "Failed to parse \"Cargo.toml\": No archive boundary found"
    )
}
