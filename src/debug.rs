use std::io::Write;

fn write_file(content: &str, file_name: &str) {
    let cwd = std::env::current_dir().unwrap();
    let mut file =
        std::fs::File::create(format!("{}/{}", cwd.to_str().unwrap(), file_name)).unwrap();
    write!(&mut file, "{}", content).unwrap();
}

pub fn debug_val(val: &str) {
    write_file(val, "dump.dbg")
}
