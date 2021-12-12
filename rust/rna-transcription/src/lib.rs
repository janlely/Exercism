#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (idx, item) in dna.chars().enumerate() {
            match item {
                'A'|'C'|'G'|'T' => {},
                _ => return Err(idx)
            }
        }
        return Ok(DNA{
            strand: dna.to_string()
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            strand: self.strand.chars().fold(String::new(), |mut acc, ch| match ch {
                'A' => {acc.push('U');acc},
                'T' => {acc.push('A');acc},
                'C' => {acc.push('G');acc},
                'G' => {acc.push('C');acc},
                _   => panic!("wrong nucleotide")
            })
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (idx, item) in rna.chars().enumerate() {
            match item {
                'A'|'C'|'G'|'U' => {},
                _ => return Err(idx)
            }
        }
        return Ok(RNA{
            strand: rna.to_string()
        })
    }
}
