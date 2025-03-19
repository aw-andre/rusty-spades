use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short, long, required_unless_present("interleaved"))]
    pub forward: Option<String>,
    #[arg(short, long, required_unless_present("interleaved"))]
    pub reverse: Option<String>,
    #[arg(short, long, required_unless_present_all(&["forward", "reverse"]))]
    pub interleaved: Option<String>,
    #[arg(short, long, default_value = "output")]
    pub output: String,
    #[arg(short, long, default_values = ["21", "33", "55"])]
    pub kmers: Vec<u8>,
    #[arg(short, long)]
    pub threads: Option<usize>,
    #[arg(short, long, default_value = "64")]
    pub memory: usize,
}

impl Config {
    pub fn threadlimit(&self) {
        match self.threads {
            Some(n) => rayon::ThreadPoolBuilder::new()
                .num_threads(n)
                .build_global()
                .unwrap(),
            None => (),
        }
    }
}
