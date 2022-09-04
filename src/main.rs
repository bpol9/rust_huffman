use std::collections::HashMap;

struct HuffmanEncoding {
    data: Vec<u64>,
    number_of_bits: u64
}

impl HuffmanEncoding {
    
    fn add_value(&mut self, data: u64, bits_number: u64) {
        let taken_bits = self.number_of_bits & 63;
        self.data.last_mut().unwrap() |= (data >> taken_bits as u64);
        if bits_number > 64 - taken_bits {
            self.data.push(data << (64 - taken_bits) as u64);
        }
        self.number_of_bits += bits_number;
    }
}

fn compute_frequencies(text: &String) -> HashMap<char, u32> {
    let mut result = HashMap::new();
    for c in text.chars() {
        let count = result.entry(c).or_insert(0);
        *count += 1;
    }

    result
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::compute_frequencies;

    #[test]
    fn test_freq_computation() {
        let text = String::from("hello alex");
        let res = compute_frequencies(&text);
        assert_eq!(res.get(&'l'), Some(&3));
        assert_eq!(res.get(&'h'), Some(&1));
        assert_eq!(res.get(&'e'), Some(&2));
        assert_eq!(res.get(&'m'), None);
    }
}
