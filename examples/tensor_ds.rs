use dlpipeline::{data_loader::DataLoaderBuilder, dataset::BatchDataset};
use tch::{kind::FLOAT_CPU, Tensor};

#[derive(Debug)]
pub struct TensorListDataset {
    samples: Vec<Tensor>,
}

impl TensorListDataset {
    pub fn new() -> TensorListDataset {
        let samples: Vec<Tensor> = (0..100)
            .map(|_i| Tensor::rand(&[2, 3, 16, 16], FLOAT_CPU))
            .collect();

        TensorListDataset { samples }
    }
}

impl BatchDataset for TensorListDataset {
    type Example = Tensor;

    fn get_item(&self, index: usize) -> Option<Tensor> {
        None
    }

    fn len(&self) -> Option<usize> {
        Some(self.samples.len())
    }
}

fn main() {
    let tensor_list_ds = TensorListDataset::new();

    // println!("{:?}", tensor_list_ds);

    let loader = DataLoaderBuilder::new(tensor_list_ds)
        .batch_size(16)
        .build();
}
