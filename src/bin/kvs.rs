use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("my kvs")
        .version("0.1")
        .author("humings@gmail.com")
        .subcommand(
            SubCommand::with_name("set")
                .about("set the key's value")
                .arg(Arg::with_name("value")
                    .help("Set an value for this key")
                    .index(1)
                )
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get the key's value")
        )
        .subcommand(SubCommand::with_name("rm")
            .about("remove a key's value")
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("get") {
        println!("found get arg");
    } else {
        // no subcommand found
        panic!("no subcommand found");
    }

}
