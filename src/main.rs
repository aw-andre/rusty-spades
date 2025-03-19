use clap::Parser;

mod config;
mod inputprocessor;

fn main() {
    let config = config::Config::parse();
    config.threadlimit();
    config.memlimit();
}
