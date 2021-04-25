extern crate crossbeam;
extern crate crossbeam_channel;

use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;

fn main() {
    // There is a data source and a data sink, with data being 
    // processed by two worker threads in parallel on its way from the source
    // to the sink

    // We use bounded channels with a capacity of one

    // The producer must be on its own thread because it produces messages
    // faster than the workers can process them - this means the producer blocks
    // on the call to [crossbeam_channel::Sender::send] for half a second
    // until one of the workers processes the data in the channel is consumed
    // by whichever worker calls receive first, so each message is delivered
    // to a single worker rather than both workers

    // Reading from the channels via the iterator crossbeam_channel::Receiver::item
    // method will block, either waiting for new messages or until the channel is closed

    // Because the channels were created within the crossbeam::scope, we must manually
    // close them via drop to prevent the entire program from blocking on the worker for-loops
    // The calls to drop can be thought of as signalling that no more messages will be sent

    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // Close the channel - this is necessary to exit
            // the for-loop in the worker
            drop(snd1);
        });

        // Parallel processing by 2 threads
        for _ in 0..n_workers {
            // Send to sink, receive from source
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // Spawn workers in separate threads
            s.spawn(move |_| {
            thread::sleep(Duration::from_millis(500));
                // Receive until channel closes
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // Close the channel, otherwise sink will never
        // exit the for-loop
        drop(snd2);

        // Sink
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    }).unwrap();
}
                