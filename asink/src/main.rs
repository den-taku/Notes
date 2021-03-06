use asink::*;
use std::sync::mpsc::Sender;

/// This will be serialized into msgpack
#[derive(Serialize)]
struct Doc {
    id: usize,
    data: Vec<f64>,
}

fn experiment(s: Sender<Doc>) {
    for i in 0..5 {
        let doc = Doc {
            id: i,
            data: vec![i as f64],
        };
        s.send(doc).unwrap(); // Send data to sink
    }
}

fn main() {
    let sink = msgpack::MsgpackSink::from_str("test.msg");
    let (s, th) = sink.run(); // Sink start thread to write recieved data into msgpack
    experiment(s);
    th.join().unwrap();
}