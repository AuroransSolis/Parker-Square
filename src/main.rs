extern crate num_cpus;
extern crate time;

use std::thread;
use std::sync::mpsc;


const GOOD_MASK: u64 = 0xC840C04048404040;

fn is_square(mut x: u32) -> bool {
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
    ((x as f64).sqrt() as u32).pow(2) == x
}

fn main() {
    let allstart = time::precise_time_s();
    let threads = num_cpus::get() as u32;
    let (solution_tx, solution_rx) = mpsc::channel();
    let mut parkers = vec![];
    for no in 0..threads {
        println!("Spawned thread {}", no);
        let t_solution_tx = solution_tx.clone();
        let parker = thread::spawn(move || {
            println!("Thread {} starting computation", no);
            let mut values = vec![0u32; 9];
            let mut e_prog = 0;
            let mut e_start = time::precise_time_s();
            for e in (0..65536).map(|e| e * e) {
                values[4] = e;
                let e64 = e as f64;
                let e_sqrt = e64.sqrt() as u32;
                if (e_sqrt & 127 == 0) && e > 0 {
                    let e_end = time::precise_time_s();
                    e_prog += 1;
                    println!("Thread {} e progress: e = {}, update {} out of 511 | Time taken: {}", no, e, e_prog, e_end - e_start);
                    e_start = time::precise_time_s();
                }
                if (3.0 * e64) % 72.0 != 3.0 {
                    continue;
                }
                for b in (0u32..65536).map(|b| b * b).take_while(|b| *b < e) {
                    values[1] = b as u32;
                    let b64 = b as f64;
                    for a in (0u32..65536).map(|a| a * a).filter(|a| *a > b && *a != e) {
                        values[0] = a;
                        let a64 = a as f64;
                        for c in (0u32..).map(|c| threads * c + no).take_while(|c| *c < 65536).map(|c| c * c).filter(|c| *c > b && *c != a && *c != e) {
                            values[2] = c;
                            let c64 = c as f64;
                            if (a64 + b64 + c64) < 3.0 * e64 {
                                continue;
                            }
                            if (a64 + b64 + c64) > 3.0 * e64 {
                                break;
                            }
                            let mut send = true;
                            while send {
                                let g = a as u64 + b as u64 - e as u64;
                                let h = a as u64 + c as u64 - e as u64;
                                let i = b as u64 + c as u64 - e as u64;
                                if (g > 65535) || (h > 65535) || (i > 65535) {
                                    send = false;
                                    continue;
                                }
                                values[6] = g as u32;
                                values[7] = h as u32;
                                values[8] = i as u32;
                                if !(is_square(values[6]) && is_square(values[7]) && is_square(values[8])) {
                                    send = false;
                                    continue;
                                }
                                let d = c as u64 + e as u64 - a as u64;
                                let f = a as u64 + e as u64 - c as u64;
                                if (d > 65535) || (h > 65535) {
                                    send = false;
                                    continue;
                                }
                                values[3] = d as u32;
                                values[5] = f as u32;
                                if !(is_square(values[3]) && is_square(values[5])) {
                                    send = false;
                                    continue;
                                }
                                let mut solution = true;
                                for n1 in 0..values.len() {
                                    for n2 in (0..values.len()).filter(|n2| *n2 != n1) {
                                        if values[n1] == values[n2] {
                                            solution = false;
                                        }
                                    }
                                }
                                if !solution {
                                    send = false;
                                    continue;
                                }
                                t_solution_tx.send(
                                    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8])
                                ).unwrap();
                            }
                        }
                    }
                }
                //let b_end = time::precise_time_s();
                //println!("Thread {} || Time for all b, a, c at e = {}: {}", no, e, b_end - b_start);
            }
        });
        parkers.push(parker);
    }
    drop(solution_tx);
    while let Ok((a, b, c, d, e, f, g, h, i)) = solution_rx.recv() {
        println!("Solution: {}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h, i);
    }
    for parker in parkers {
        parker.join().unwrap();
    }
    let allend = time::precise_time_s();
    println!("All threads completed");
    println!("Time taken: {}", allend - allstart);
}