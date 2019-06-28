use std::cmp;

use time::{Duration, PreciseTime};

pub fn bench<T, F>(trials: usize, mut f: F) -> Duration
where
    F: FnMut() -> T,
{
    let mut benchmark = Benchmark::new();
    for _ in 0..trials {
        let mut timer = benchmark.start();
        let _keep = f();
        timer.stop();
    }
    benchmark.min_elapsed()
}

pub fn bench_with_buf<T, F>(trials: usize, len: usize, f: F) -> Duration
where
    F: Fn(&mut Vec<u8>) -> T,
{
    let mut benchmark = Benchmark::new();
    for _ in 0..trials {
        let mut buf = Vec::with_capacity(len);
        let mut timer = benchmark.start();
        let _keep = f(&mut buf);
        timer.stop();
    }
    benchmark.min_elapsed()
}

pub struct Benchmark {
    min_elapsed: Option<Duration>,
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark { min_elapsed: None }
    }

    pub fn start<'a>(&'a mut self) -> Timer<'a> {
        Timer {
            source: self,
            start: PreciseTime::now(),
            stopped: false,
        }
    }

    pub fn min_elapsed(self) -> Duration {
        self.min_elapsed.unwrap()
    }
}

pub struct Timer<'a> {
    source: &'a mut Benchmark,
    start: PreciseTime,
    stopped: bool,
}

impl<'a> Timer<'a> {
    pub fn stop(&mut self) {
        let elapsed = self.start.to(PreciseTime::now());
        if self.stopped {
            panic!("already stopped");
        }
        self.stopped = true;
        self.source.min_elapsed = Some(match self.source.min_elapsed {
            None => elapsed,
            Some(prev) => cmp::min(prev, elapsed),
        })
    }
}
