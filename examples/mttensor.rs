use std::thread;
use std::time::Duration;
use tch::{Device, Kind, Tensor};

fn main() {
    let handles: Vec<thread::JoinHandle<Tensor>> = (0..10)
        .map(|_i| {
            thread::spawn(|| {
                let x = Tensor::rand(&[2, 3, 16, 16], tch::kind::FLOAT_CPU);
                let y: Tensor = x.upsample_bilinear2d(&[8, 8], true, 1., 1.);
                thread::sleep(Duration::from_secs(1));
                y * 2
            })
        })
        .collect();

    let outputs: Vec<Tensor> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("{:#?}", outputs);
}
