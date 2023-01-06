use bitvec::prelude::*;

fn compress(gene: &[u8]) -> BitVec {
    let mut bits = BitVec::new();
    for nucleotide in gene.iter().map(|&c| c.to_ascii_uppercase()) {
        match nucleotide {
            b'A' => bits.extend_from_bitslice(bits![0, 0]),
            b'C' => bits.extend_from_bitslice(bits![0, 1]),
            b'G' => bits.extend_from_bitslice(bits![1, 0]),
            b'T' => bits.extend_from_bitslice(bits![1, 1]),
            _ => panic!("Invalid nucleotide: {}", nucleotide as char),
        }
    }
    bits
}

fn decompress(bits: &BitVec) -> Vec<u8> {
    let mut gene = Vec::new();
    for chunk in bits.chunks(2) {
        let nucleotide = match (chunk[0], chunk[1]) {
            (false, false) => b'A',
            (false, true) => b'C',
            (true, false) => b'G',
            (true, true) => b'T',
            _ => panic!("Invalid bit pattern: {:?}", chunk),
        };
        gene.push(nucleotide);
    }
    gene
}

fn main() {
    let str =
        b"TAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATATAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATA"
            .repeat(100);
    println!("original is {} bytes", str.len());
    let compressed = compress(&str);
    println!("{}", String::from_utf8(decompress(&compressed)).unwrap());
    println!(
        "original and decompressed are the same: {}",
        str == decompress(&compressed)
    );
}
