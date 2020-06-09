use super::dataset::BatchDataset;

pub struct DataLoader<T>
where
    T: BatchDataset,
{
    dataset: T,
    batch_size: usize,
    num_workers: usize,
    shuffle: bool,
    order: bool,
}

impl<T> DataLoader<T> where T: BatchDataset {}


pub struct DataLoaderBuilder<T>
where
    T: BatchDataset,
{
    dataset: T,
    batch_size: usize,
    num_workers: usize,
    shuffle: bool,
    order: bool,
}

impl<T> DataLoaderBuilder<T>
where
    T: BatchDataset,
{
    pub fn new(dataset: T) -> Self {
        DataLoaderBuilder {
            dataset,
            batch_size: 1,
            num_workers: 0,
            shuffle: false,
            order: false,
        }
    }

    pub fn batch_size(mut self, bs: usize) -> Self {
        self.batch_size = bs;
        self
    }

    pub fn num_workers(mut self, n: usize) -> Self {
        self.num_workers = n;
        self
    }

    pub fn shuffle(mut self, shuffle: bool) ->Self {
        self.shuffle = shuffle;
        self
    }

    pub fn order(mut self, order: bool) -> Self {
        self.order = order;
        self
    }

    pub fn build(self) -> DataLoader<T> {
        DataLoader {
            dataset: self.dataset,
            batch_size: self.batch_size,
            num_workers: self.num_workers,
            shuffle: self.shuffle,
            order: self.order,
        }
    }
}
