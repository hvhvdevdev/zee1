use platform_posix::config_reader::CmdLineConfigReader;

#[cfg(not(tarpaulin_include))]
fn main() {
    //----------------------------------------------------------------------------------------------
    // Read the run configuration from command line arguments.
    //----------------------------------------------------------------------------------------------
    let config = CmdLineConfigReader::new(std::env::args().collect())
        .read()
        .unwrap();

    //----------------------------------------------------------------------------------------------
    // Print the run configuration for debugging purposes.
    //----------------------------------------------------------------------------------------------
    println!("{:#?}", config);
}
