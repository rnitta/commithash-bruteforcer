use clap::{App, Arg};
use commithash_bruteforcer::helpers::*;

fn main() {
    let app = App::new("commithash")
        .version("0.0.1")
        .author("rnitta <rnitta@gmail.com>")
        .about("Short commit hash")
        .arg(
            Arg::with_name("PATH")
                .help("path to dir")
                .short("p")
                .long("path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("PREFIX")
                .help("search commit hash starting with PREFIX")
                .short("s")
                .long("search")
                .takes_value(true)
                .required(true),
        );
    let matches = app.get_matches();
    let path = matches.value_of("PATH").unwrap();
    let prefix = matches.value_of("PREFIX").unwrap();
    let mut name: String = "0000000000000000000000000000000000000000".to_owned();

    loop {
        empty_commit(path, prefix, &name);
        let commit_hash = latest_commit_hash(path);
        let cat_file = cat_file(path, &commit_hash);
        let (former, latter) = split_base(&cat_file, &name);

        'a: loop {
            let seed = [former.to_owned(), name.to_owned(), latter.to_owned()].join("");
            let hash = hashize(&seed);
            if hash.starts_with(prefix) {
                filter_branch(path, &commit_hash, &name);
                println!("yay!: {}", hash);
                break 'a;
            }
            name = hash;
        }
    }
}
