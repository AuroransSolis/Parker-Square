extern crate num_cpus;
extern crate time;

use std::thread;
use std::sync::mpsc;


const GOOD_MASK: u64 = 0xC840C04048404040;

fn is_square(mut x: u64) -> bool {
    if (GOOD_MASK << x) as i64 >= 0 {
        return false;
    }
    let zeros = x.trailing_zeros();
    if zeros & 1 != 0 {
        return false;
    }
    x >>= zeros;
    if x & 7 != 1 {
        return x == 0;
    }
    ((x as f64).sqrt() as u64).pow(2) == x
}

fn main() {
    let allstart = time::precise_time_s();
    let threads = num_cpus::get() as u64;
    let (solution_tx, solution_rx) = mpsc::channel();
    let (status_tx, status_rx) = mpsc::channel();
    let mut parkers = vec![];
    let mut update_channels = vec![];
    for no in 0..threads {
        println!("Spawned thread {}", no);
        let t_solution_tx = solution_tx.clone();
        let (getupdate_tx, getupdate_rx) = mpsc::channel();
        update_channels.push(getupdate_tx);
        let t_status_tx = status_tx.clone();
        let parker = thread::spawn(move || {
            println!("Thread {} starting computation", no);
            for x in (2..65536u64).map(|x| x * x).filter(|x| *x % 24 == 1) {
                for y in (2..65535u64).map(|y| y * y).filter(|y| *y % 24 == 1).take_while(|y| *y < x) {
                    for z in (2u64..).map(|z| (z * threads + no).pow(2u32)).take_while(|z| *z < x - y) {
                        while let Ok(msg) = getupdate_rx.try_recv() {
                            if msg {
                                println!("Thread {} | x: {}, y: {}, z: {}", no, x, y, z);
                            }
                        }
                        if is_square(x + y) && is_square(x - y - z) && is_square(x + z)
                            && is_square(x - y + z) && is_square(x + y - z)
                            && is_square(x - z) && is_square(x + y + z) && is_square(x - y) {
                            t_solution_tx.send((x + y, x - y - z, x + z, x - y + z, x, x + y - z, x - z, x + y + z, x - y)).unwrap();
                        }
                    }
                }
            }
            t_status_tx.send(false).unwrap();
        });
        parkers.push(parker);
    }
    drop(solution_tx);
    drop(status_tx);
    let mut start_countdown = time::precise_time_s();
    let mut working = true;
    loop {
        if time::precise_time_s() as u64 == start_countdown as u64 + 300 {
            for sender in update_channels.iter() {
                (*sender).send(true).unwrap();
            }
            start_countdown = time::precise_time_s();
        }
        while let Ok((a, b, c, d, e, f, g, h, i)) = solution_rx.try_recv() {
            println!("Solution: {}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h, i);
        }
        while let Ok(work) = status_rx.try_recv() {
            if !work {
                working = false;
            }
        }
        if !working {
            break;
        }
    }
    for parker in parkers {
        parker.join().unwrap();
    }
    let allend = time::precise_time_s();
    println!("All threads completed");
    println!("Time taken: {}", allend - allstart);
}
