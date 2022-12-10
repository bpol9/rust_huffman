use std::collections::HashMap;
use std::rc::Rc;

struct HuffmanEncoding {
    data: Vec<u64>,
    number_of_bits: u64
}

struct HuffmanNode {
    weight: u32,
    symbol: Option<char>,
    left: Option<Rc<HuffmanNode>>,
    right: Option<Rc<HuffmanNode>>
}

struct HuffmanUnitCode {
    code: u64,
    number_of_bits: u8
}

impl HuffmanEncoding {

    fn add_value(&mut self, value: u64, number_of_bits: u64) {
        if self.number_of_bits & 63 == 0 {
            self.data.push(0);
            if self.data.len() == 1 {
                self.number_of_bits = 0;
            }
        }

        let taken_bits = self.number_of_bits & 63;
        let remaining_bits = 64 - taken_bits;
        *self.data.last_mut().unwrap() |= value << taken_bits;
        if number_of_bits > remaining_bits {
            self.data.push(value >> remaining_bits);
        }
        self.number_of_bits += number_of_bits;
    }
}

impl HuffmanUnitCode {
    fn add_bit_zero(&mut self) {
        self.code = self.code << 1;
        self.number_of_bits += 1;
    }

    fn add_bit_one(&mut self) {
        self.code = (self.code << 1) | 1;
        self.number_of_bits += 1;
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
    let mut leaves = Vec::new();
    for (symbol, freq) in freqs {
        leaves.push(HuffmanNode {
            weight: freq,
            symbol: Some(symbol),
            left: None,
            right: None
        });
    }
    leaves
}

fn construct_huffman_tree(text: &String) -> Rc<HuffmanNode> {
    let mut leaves = get_huffman_leaves(text);
    quick_sort(&mut leaves, &|x,y| x.weight < y.weight);
    let mut intermediate_nodes = Vec::new();
    while leaves.len() > 0 || intermediate_nodes.len() > 1 {
        let one = take_smallest(&mut leaves, &mut intermediate_nodes);
        let two = take_smallest(&mut leaves, &mut intermediate_nodes);
        let new_node = HuffmanNode {
            weight: one.weight + two.weight,
            symbol: None,
            left: Some(Rc::new(one)),
            right: Some(Rc::new(two))
        };
        intermediate_nodes.push(new_node);
    }

    Rc::new(intermediate_nodes.pop().unwrap())
}

fn take_smallest(first: &mut Vec<HuffmanNode>, second: &mut Vec<HuffmanNode>) -> HuffmanNode {
    if first.is_empty() || (!second.is_empty() && first[0].weight > second[0].weight) {
        second.remove(0)
    } else {
        first.remove(0)
    }
    //first.is_empty() || first[0].weight > second[0].weight ? second.pop_front().unwrap() : first.pop_front().unwrap()
}

fn dfs_huffman_tree(node: Rc<HuffmanNode>, curr_unit_code: HuffmanUnitCode, result: &mut HashMap<char, HuffmanUnitCode>) {
    if node.left.is_none() && node.right.is_none() {
        result.insert(node.symbol.clone().unwrap(), curr_unit_code);
        return;
    }

    if !node.left.is_none() {
        let mut left_unit_code = HuffmanUnitCode {
            code: curr_unit_code.code,
            number_of_bits: curr_unit_code.number_of_bits
        };
        left_unit_code.add_bit_zero();
        dfs_huffman_tree(node.left.clone().unwrap(), left_unit_code, result);
    }
    if !node.right.is_none() {
        let mut right_unit_code = HuffmanUnitCode {
            code: curr_unit_code.code,
            number_of_bits: curr_unit_code.number_of_bits
        };
        right_unit_code.add_bit_one();
        dfs_huffman_tree(node.right.clone().unwrap(), right_unit_code, result);
    }

}

fn get_encoding_from_huffman_tree(tree: Rc<HuffmanNode>) -> HashMap<char, HuffmanUnitCode> {
    let mut result = HashMap::new();
    let curr_enc = HuffmanUnitCode {
        code: 0,
        number_of_bits: 0
    };
    dfs_huffman_tree(tree, curr_enc, &mut result);
    result
}

fn get_huffman_encoding(text: &String) -> HashMap<char, HuffmanUnitCode> {
    let tree = construct_huffman_tree(text);
    let encoding = get_encoding_from_huffman_tree(tree);
    encoding
}

fn encode_with_huffman(text: &String) -> HuffmanEncoding {
    let map = get_huffman_encoding(text);
    let mut result = HuffmanEncoding {
        data: Vec::new(),
        number_of_bits: 0
    };
    let mut unit_code: &HuffmanUnitCode;
    for c in text.chars() {
        unit_code = map.get(&c).expect("character not found in map!");
        result.add_value(unit_code.code, unit_code.number_of_bits.into());
    }

    result
}



fn main() {
    println!("Hello, world!!!");
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
    use super::get_huffman_leaves;
    use super::HuffmanUnitCode;
    use super::get_huffman_encoding;
    use super::HuffmanEncoding;
    use super::encode_with_huffman;

    #[test]
    fn test_freq_computation() {
        let text = String::from("hello alex");
        let res = compute_frequencies(&text);
        assert_eq!(res.get(&'l'), Some(&3));
        assert_eq!(res.get(&'h'), Some(&1));
        assert_eq!(res.get(&'e'), Some(&2));
        assert_eq!(res.get(&'m'), None);
    }

    #[test]
    fn test_get_huffman_leaves() {
        let text = String::from("mmmmaaarrrthhaa");
        let leaves = get_huffman_leaves(&text);
        assert_eq!(5, leaves.len());
        assert!(leaves.iter().any(|l| l.symbol == Some('m') && l.weight == 4));
        assert!(leaves.iter().any(|l| l.symbol == Some('a') && l.weight == 5));
        assert!(leaves.iter().any(|l| l.symbol == Some('r') && l.weight == 3));
        assert!(leaves.iter().any(|l| l.symbol == Some('t') && l.weight == 1));
        assert!(leaves.iter().any(|l| l.symbol == Some('h') && l.weight == 2));
    }

    #[test]
    fn test_encode_with_huffman() {
        let text = String::from("mmmmaaarrrthhaa");
        let result = encode_with_huffman(&text);
        assert_eq!(1, result.data.len());
        assert_eq!(Some(&8281669546), result.data.get(0));
        assert_eq!(33, result.number_of_bits);
    }


    #[test]
    fn test_huffman_unit_code_add_bit_zero() {
        let mut unit_code = HuffmanUnitCode {
            code: 5,
            number_of_bits: 3
        };
        unit_code.add_bit_zero();
        assert_eq!(10, unit_code.code);
        assert_eq!(4, unit_code.number_of_bits);
    }

    #[test]
    fn test_huffman_unit_code_add_bit_one() {
        let mut unit_code = HuffmanUnitCode {
            code: 0,
            number_of_bits: 0
        };
        unit_code.add_bit_one();
        assert_eq!(1, unit_code.code);
        assert_eq!(1, unit_code.number_of_bits);
    }

    #[test]
    fn test_encoding() {
        let text = String::from("mmmmaaarrrthhaa");
        let result = get_huffman_encoding(&text);
        let m_opt = result.get(&'m');
        assert!(!m_opt.is_none());
        let m_enc = m_opt.unwrap();
        assert_eq!(m_enc.code, 2);
        assert_eq!(m_enc.number_of_bits, 2);
        let a_opt = result.get(&'a');
        assert!(!a_opt.is_none());
        let a_enc = a_opt.unwrap();
        assert_eq!(a_enc.code, 3);
        assert_eq!(a_enc.number_of_bits, 2);
        let t_opt = result.get(&'t');
        assert!(!t_opt.is_none());
        let t_enc = t_opt.unwrap();
        assert_eq!(t_enc.code, 2);
        assert_eq!(t_enc.number_of_bits, 3);
        let h_opt = result.get(&'h');
        assert!(!h_opt.is_none());
        let h_enc = h_opt.unwrap();
        assert_eq!(h_enc.code, 3);
        assert_eq!(h_enc.number_of_bits, 3);
        let r_opt = result.get(&'r');
        assert!(!r_opt.is_none());
        let r_enc = r_opt.unwrap();
        assert_eq!(r_enc.code, 0);
        assert_eq!(r_enc.number_of_bits, 2);
    }

    #[test]
    fn test_add_value_huffman_encoding() {
        let mut enc = HuffmanEncoding {
            data: Vec::new(),
            number_of_bits: 0
        };
        enc.add_value(3, 2);
        assert_eq!(enc.data.len(), 1);
        assert_eq!(enc.data.get(0), Some(&3));
        enc.add_value(1, 2);
        assert_eq!(enc.data.len(), 1);
        assert_eq!(enc.data.get(0), Some(&7));
        let new_value = 1 << 59;
        let expected = (1 << 63) + 7;
        enc.add_value(new_value, 60);
        assert_eq!(enc.data.len(), 1);
        assert_eq!(enc.data.get(0), Some(&expected));
        enc.add_value(2, 2);
        assert_eq!(enc.data.len(), 2);
        assert_eq!(enc.data.get(0), Some(&expected));
        assert_eq!(enc.data.get(1), Some(&2));
    }
}
