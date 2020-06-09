pub trait Sampler: Iterator {
    // 默认啥也不做
    fn reset(&mut self, new_size: Option<usize>) {}

    
}
