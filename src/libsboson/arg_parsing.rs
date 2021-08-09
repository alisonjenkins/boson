use clap::{
    App,
    // SubCommand
    Arg,
    ArgMatches,
};

pub fn get_args<'a>() -> Result<ArgMatches<'a>, String> {
    let matches = App::new("boson")
                          .version("BOSON_VERSION")
                          .author("Alan Jenkins <alan.james.jenkins@gmail.com>")
                          .about("Manages launching games using Valve's Proton Linux compatibility layer and associated options.")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .get_matches();
    Ok(matches)
}
