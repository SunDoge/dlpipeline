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

        let current_batch_size = batch_size.min(remaining_indices);
        let mut index_batch: Self::Item = Vec::with_capacity(current_batch_size);

        for _ in 0..current_batch_size {
            index_batch.push(self.index);
            self.index += 1;
        }

        Some(index_batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let mut sampler = SequentialSampler::new(5);

        assert_eq!(sampler.next(4), Some(vec![0, 1, 2, 3]));
        assert_eq!(sampler.next(4), Some(vec![4]));
        assert_eq!(sampler.next(4), None);
    }

    #[test]
    fn reset() {
        let mut sampler = SequentialSampler::new(3);
        assert_eq!(sampler.next(4), Some(vec![0, 1, 2]));
        assert_eq!(sampler.next(4), None);
        sampler.reset(None);
        assert_eq!(sampler.next(4), Some(vec![0, 1, 2]));
        sampler.reset(Some(2));
        assert_eq!(sampler.next(4), Some(vec![0, 1]));
    }
}
