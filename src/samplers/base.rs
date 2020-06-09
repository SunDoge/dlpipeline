pub trait Sampler {
    type Item;
    // 默认啥也不做
    fn reset(&mut self, new_size: Option<usize>) {}

    fn next(&mut self, batch_size: usize) -> Option<Self::Item>;
}
