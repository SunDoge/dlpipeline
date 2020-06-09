pub trait BatchDataset {
    /// We need an Example type to descript the data output
    type Example;

    // 返回一个样本
    fn get_item(&self, index: usize) -> Option<Self::Example>;

    // 返回多个样本
    // fn get_batch(&self, indices: &[usize]) -> Vec<Self::Example> {
    //     let mut result: Vec<Self::Example> = Vec::with_capacity(indices.len());
    //     for index in indices {
    //         result.push(self.get(*index));
    //     }
    //     result
    // }

    // 有些dataset不知道长度
    fn len(&self) -> Option<usize>;
}
