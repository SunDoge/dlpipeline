use std::time::Duration;

pub struct DataLoaderOptions {
    pub batch_size: usize,
    pub num_workers: usize,
    pub enforce_ordering: bool,
    pub max_jobs: Option<usize>,
    pub timeout: Option<Duration>,
    pub drop_last: bool,
}

impl Default for DataLoaderOptions {
    fn default() -> Self {
        DataLoaderOptions {
            batch_size: 1,
            num_workers: 0,
            enforce_ordering: true,
            max_jobs: None,
            timeout: None,
            drop_last: false,
        }
    }
}

pub struct FullDataLoaderOptions {
    pub batch_size: usize,
    pub num_workers: usize,
    pub enforce_ordering: bool,
    pub max_jobs: usize,
    pub timeout: Option<Duration>,
    pub drop_last: bool,
}

impl From<DataLoaderOptions> for FullDataLoaderOptions {
    fn from(options: DataLoaderOptions) -> FullDataLoaderOptions {
        FullDataLoaderOptions {
            batch_size: options.batch_size,
            num_workers: options.num_workers,
            max_jobs: options.max_jobs.unwrap_or(options.num_workers * 2),
            timeout: options.timeout,
            enforce_ordering: options.enforce_ordering,
            drop_last: options.drop_last,
        }
    }
}
