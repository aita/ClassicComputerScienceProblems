use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl TryFrom<char> for Nucleotide {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Nucleotide::A),
            'C' => Ok(Nucleotide::C),
            'G' => Ok(Nucleotide::G),
            'T' => Ok(Nucleotide::T),
            _ => Err("Invalid nucleotide"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Codon(Nucleotide, Nucleotide, Nucleotide);

#[derive(Debug, Clone)]
struct Gene {
    codons: Vec<Codon>,
}

impl FromStr for Gene {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 3 != 0 {
            return Err("Invalid gene length");
        }
        let mut codons = Vec::new();
        for mut chunk in &s.chars().chunks(3) {
            let a = chunk.next().unwrap().try_into()?;
            let b = chunk.next().unwrap().try_into()?;
            let c = chunk.next().unwrap().try_into()?;
            codons.push(Codon(a, b, c));
        }
        Ok(Gene { codons })
    }
}

impl Gene {
    fn linear_contains(&self, key_codon: &Codon) -> bool {
        self.codons.contains(key_codon)
    }

    fn binary_contains(&self, key_codon: &Codon) -> bool {
        self.codons.binary_search(key_codon).is_ok()
    }
}

fn main() {
    let gene = Gene::from_str("ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT").unwrap();
    let acg = Codon(Nucleotide::A, Nucleotide::C, Nucleotide::G);
    let gat = Codon(Nucleotide::G, Nucleotide::A, Nucleotide::T);

    println!("GENE: {:?}", gene);
    println!(
        "Find {:?} by linear search: {:?}",
        acg,
        gene.linear_contains(&acg)
    );
    println!(
        "Find {:?} by binary search: {:?}",
        acg,
        gene.binary_contains(&acg)
    );
    println!(
        "Find {:?} by Linear search: {:?}",
        gat,
        gene.linear_contains(&gat)
    );
    println!(
        "Find {:?} by binary search: {:?}",
        gat,
        gene.binary_contains(&gat)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENE_STR: &str = "ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT";

    #[test]
    fn test_gene_from_str() {
        let gene = Gene::from_str(GENE_STR).unwrap();
        assert_eq!(gene.codons.len(), GENE_STR.len() / 3);
    }

    #[test]
    fn test_gene_linear_contains() {
        let gene = Gene::from_str(GENE_STR).unwrap();

        let acg = super::Codon(
            super::Nucleotide::A,
            super::Nucleotide::C,
            super::Nucleotide::G,
        );
        assert!(gene.linear_contains(&acg));

        let gat = super::Codon(
            super::Nucleotide::G,
            super::Nucleotide::A,
            super::Nucleotide::T,
        );
        assert!(!gene.linear_contains(&gat));
    }

    #[test]
    fn test_gene_binary_contains() {
        let gene = Gene::from_str(GENE_STR).unwrap();

        let acg = super::Codon(
            super::Nucleotide::A,
            super::Nucleotide::C,
            super::Nucleotide::G,
        );
        assert!(gene.binary_contains(&acg));

        let gat = super::Codon(
            super::Nucleotide::G,
            super::Nucleotide::A,
            super::Nucleotide::T,
        );
        assert!(!gene.binary_contains(&gat));
    }
}
