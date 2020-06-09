use super::dataloader_options::DataLoaderOptions;
use super::dataset::BatchDataset;
use super::samplers::base::Sampler;
use std::sync::Arc;

pub struct DataLoader<D, S>
where
    D: BatchDataset,
    S: Sampler,
{
    options: DataLoaderOptions,
    dataset: Arc<D>,
    sampler: S,
}

impl<D, S> DataLoader<D, S>
where
    D: BatchDataset,
    S: Sampler,
{
    pub fn new(dataset: D, sampler: S, options: DataLoaderOptions) -> Self {
        DataLoader {
            dataset: Arc::new(dataset),
            sampler,
            options,
        }
    }
}
