use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(PartialEq)]
struct MinFloat(f64);

impl Eq for MinFloat {}

impl PartialOrd for MinFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinFloat {
    fn cmp(&self, other: &MinFloat) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        // N.B. Reversing the order enables a min-heap property.
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord
        }
    }
}

#[derive(PartialEq, PartialOrd)]
struct MaxFloat(f64);

impl Eq for MaxFloat {}

impl Ord for MaxFloat {
    fn cmp(&self, other: &MaxFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let stdin = io::stdin();

    let mut minheap = BinaryHeap::new();
    let mut maxheap = BinaryHeap::new();

    let mut count: i64 = 0;
    let mut sum: f64 = 0.0;
    let mut sq_sum: f64 = 0.0;
    let mut min: f64 = 0.0;
    let mut max: f64 = 0.0;

    for bs in stdin.lock().split(b' ') {
        let svec = &bs.unwrap();
        let arg = std::str::from_utf8(svec);
        match arg.unwrap().trim().parse::<f64>() {
            Ok(f) => {
                if count == 0 || f < min {
                    min = f;
                }

                if count == 0 || f > max {
                    max = f;
                }

                count += 1;
                sum += f;
                sq_sum += f * f;

                if maxheap.len() == 0 || maxheap.peek() > Some(&MaxFloat(f)) {
                    maxheap.push(MaxFloat(f));
                } else {
                    minheap.push(MinFloat(f));
                }

                if maxheap.len() as i32 - minheap.len() as i32 > 1 {
                    if let Some(MaxFloat(root)) = maxheap.pop() {
                        minheap.push(MinFloat(root));
                    }
                } else if minheap.len() as i32 - maxheap.len() as i32 > 1 {
                    if let Some(MinFloat(root)) = minheap.pop() {
                        maxheap.push(MaxFloat(root));
                    }
                }

            },
            Err(e) =>  println!("{:?}", e)
        }
    }

    println!("N      {}", count);

    if count == 0 {
        return;
    }

    println!("min    {}", min);
    println!("max    {}", max);
    println!("sum    {}", sum);

    let mut median: f64 = 0.0;

    if maxheap.len() > minheap.len() {
        if let Some(&MaxFloat(root)) = maxheap.peek() {
            median = root;
        }
    } else if minheap.len() > maxheap.len() {
        if let Some(&MinFloat(root)) = minheap.peek() {
            median = root;
        }
    } else {
        if let Some(&MaxFloat(maxroot)) = maxheap.peek() {
            if let Some(&MinFloat(minroot)) = minheap.peek() {
                median = (maxroot + minroot) / 2.0;
            }
        }
    }

    println!("median {}", median);

    let fcount: f64 = count as f64;
    let mean: f64 = sum / fcount;

    println!("mean   {}", mean);

    if count > 1 {
        let stddev: f64 = (fcount * sq_sum - sum * sum) / (fcount * (fcount - 1.0));
        println!("σ      {}", stddev.sqrt())
    }
}
