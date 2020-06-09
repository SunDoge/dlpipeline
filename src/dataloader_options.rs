use std::time::Duration;

pub struct DataLoaderOptionsBuilder {
    pub batch_size: usize,
    pub num_workers: usize,
    pub enforce_ordering: bool,
    pub max_jobs: Option<usize>,
    pub timeout: Option<Duration>,
    pub drop_last: bool,
}

impl Default for DataLoaderOptionsBuilder {
    fn default() -> Self {
        DataLoaderOptionsBuilder {
            batch_size: 1,
            num_workers: 0,
            enforce_ordering: true,
            max_jobs: None,
            timeout: None,
            drop_last: false,
        }
    }
}

impl DataLoaderOptionsBuilder {
    pub fn batch_size<'a>(&'a mut self, bs: usize) -> &'a mut Self {
        self.batch_size = bs;
        self
    }

    pub fn num_workers<'a>(&'a mut self, n: usize) -> &'a mut Self {
        self.num_workers = n;
        self
    }

    // TODO: more functions

    pub fn build(&self) -> DataLoaderOptions {
        DataLoaderOptions::from(self)
    }
}

pub struct DataLoaderOptions {
    pub batch_size: usize,
    pub num_workers: usize,
    pub enforce_ordering: bool,
    pub max_jobs: usize,
    pub timeout: Option<Duration>,
    pub drop_last: bool,
}

impl From<&DataLoaderOptionsBuilder> for DataLoaderOptions {
    fn from(options: &DataLoaderOptionsBuilder) -> DataLoaderOptions {
        DataLoaderOptions {
            batch_size: options.batch_size,
            num_workers: options.num_workers,
            max_jobs: options.max_jobs.unwrap_or(options.num_workers * 2),
            timeout: options.timeout,
            enforce_ordering: options.enforce_ordering,
            drop_last: options.drop_last,
        }
    }
}
