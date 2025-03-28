use cap::Cap;
use clap::Parser;
use std::alloc;

#[global_allocator]
pub static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::MAX);

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short, long, required_unless_present("resume"))]
    pub forward: Option<String>,
    #[arg(short, long, required_unless_present("resume"))]
    pub reverse: Option<String>,
    #[arg(short, long, required_unless_present_all(&["forward", "reverse"]))]
    pub resume: Option<String>,
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
    pub fn threadlimit(&self) -> &Self {
        if let Some(n) = self.threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(n)
                .build_global()
                .unwrap()
        }
        self
    }

    pub fn memlimit(&self) -> &Self {
        ALLOCATOR.set_limit(self.memory).unwrap();
        self
    }
}
