extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate html2text;
extern crate mercury;
extern crate tokio_core;

mod error;

use std::env;

use clap::{App, AppSettings};
use dotenv::dotenv;
use mercury::Mercury;
use tokio_core::reactor::Core;

use error::Error;

quick_main!(run);

fn run() -> Result<i32, Error> {
    dotenv().ok();

    let mut core = Core::new()?;
    let key = env::var("MERCURY_API_KEY")?;
    let client = Mercury::new(&core.handle(), key)?;
    let matches = App::new("Mercury Reader")
        .version("0.1")
        .about("Read articles in your terminal. Powered by the Mercury Parser.")
        .author("Postlight")
        .arg_from_usage("<url> 'The url of the article you would like to read'")
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let url = matches.value_of("url").unwrap_or_else(|| unreachable!());
    let article = core.run(client.parse(url))?;

    println!("");
    println!("{}", article.title);

    if let Some(ref name) = article.author {
        println!("{}", name);
    }

    println!("");
    println!("{}", {
        let data = article.content.as_bytes();
        let width = article.content.len();

        html2text::from_read(data, width)
    });

    Ok(0)
}
