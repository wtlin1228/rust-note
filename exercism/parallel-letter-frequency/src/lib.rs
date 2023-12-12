use std::collections::HashMap;
use std::thread;

fn counter(input: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for line in input {
        for c in line
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            *map.entry(c).or_default() += 1;
        }
    }
    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match worker_count {
        1 => counter(input),
        _ => match input.len() {
            // unnecessary spawn threads for tiny data set
            n if n < 500 => counter(input),
            _ => thread::scope(|s| {
                let mut handles = Vec::with_capacity(worker_count);
                let chunk_size = match input.len() % worker_count {
                    0 => input.len() / worker_count,
                    _ => input.len() / worker_count + 1,
                };
                input.chunks(chunk_size).for_each(|chunk| {
                    let handle = s.spawn(|| counter(chunk));
                    handles.push(handle);
                });
                let mut res = handles.pop().unwrap().join().unwrap();
                for handle in handles {
                    handle
                        .join()
                        .unwrap()
                        .iter()
                        .for_each(|(letter, frequency)| {
                            *res.entry(*letter).or_default() += frequency;
                        })
                }
                return res;
            }),
        },
    }
}
