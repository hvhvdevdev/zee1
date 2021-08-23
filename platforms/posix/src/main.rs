use platform_posix::config_reader::CmdLineConfigReader;

#[cfg(not(tarpaulin_include))]
fn main() {
    let config = CmdLineConfigReader::new(std::env::args().collect())
        .read()
        .unwrap();
    println!("{:#?}", config);
}
