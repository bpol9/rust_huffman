use std::collections::HashMap;
use std::rc::Rc;

struct HuffmanEncoding {
    data: Vec<u64>,
    number_of_bits: u64
}

struct HuffmanNode {
    weight: f64,
    symbol: Option<char>,
    left: Option<Rc<HuffmanNode>>,
    right: Option<Rc<HuffmanNode>>
}

struct HuffmanUnitCode {
    code: u64,
    number_of_bits
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

fn get_huffman_leaves(text: &String) -> Vec<HuffmanNode> {
    let freqs = compute_frequencies(text);
    let leaves = Vec::new();
    for (symbol, freq in freqs) {
        leaves.add(HuffmanNode {
            weight: freq,
            symbol: Some(symbol),
            None,
            None
        });
    }
}

fn construct_huffman_tree(text: &String): Rc<HuffmanNode> {
    let leaves = get_huffman_leaves(text);
    quick_sort(&mut leaves, &|x,y| x.weight < y.weight);
    let intermediate_nodes = Vec::new();
    while intermediate_nodes.len() > 1 {
        let first = take_smallest(leaves, intermediate_nodes);
        let second = take_smallest(leaves, intermediate_nodes);
        let new_node = HuffmanNode {
            weight: one.weight + two.weight,
            symbol: None,
            Rc::new(one),
            Rc::new(two)
        }
        intermediate_nodes.push(new_node);
    }

    Rc::new(intermediate_nodes.pop())
}

fn take_smallest(first: Vec<HuffmanNode>, second: Vec<HuffmanNode>) -> HuffmanNode {
    first.is_empty() || first[0].weight > second[0].weight ? second.pop_front().unwrap() : first.pop_front().unwrap()
}

fn get_huffman_encoding(text: &String) -> HashMap<char, HuffmanUnitCode> {
    //...
}


fn main() {
    println!("Hello, world!");
}

fn quick_sort<T,F>(v: &mut [T], f: &F)
    where F: Fn(&T,&T) -> bool
{
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v, f);
        quick_sort(&mut v[0..pivot_index], f);
        quick_sort(&mut v[pivot_index + 1..len], f);
    }
}

fn partition<T,F>(v: &mut [T], f: &F) -> usize
    where F: Fn(&T,&T) -> bool
{
    let len = v.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if f(&v[i], &v[last_index]) {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
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
