use super::base::Sampler;

pub struct SequentialSampler {
    size: usize,
    index: usize,
}

impl SequentialSampler {
    pub fn new(size: usize) -> SequentialSampler {
        SequentialSampler { size, index: 0 }
    }
}

impl Sampler for SequentialSampler {
    type Item = Vec<usize>;

    fn reset(&mut self, new_size: Option<usize>) {
        if let Some(size) = new_size {
            self.size = size;
        }
        self.index = 0;
    }

    fn next(&mut self, batch_size: usize) -> Option<Self::Item> {
        let remaining_indices = self.size - self.index;
        if remaining_indices == 0 {
            return None;
        }
        let mut index_batch: Self::Item = Vec::with_capacity(batch_size.min(remaining_indices));

        for i in &mut index_batch {
            *i = self.index;
            self.index += 1;
        }

        Some(index_batch)
    }
}
