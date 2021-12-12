use itertools::Itertools;

pub fn count(lines: &[&str]) -> u32 {
    let mut res = 0;
    let src_vec: Vec<Vec<char>> = lines.iter().map(|&s| {
        s.chars().collect::<Vec<char>>()
    }).collect();
    let mut idx1 = 0;
    while idx1 < src_vec.len() {
        let mut idx2 = 0;
        let mut line_idxs = Vec::new();
        while idx2 < src_vec[idx1].len() {
            if src_vec[idx1][idx2] == '+' {
                line_idxs.push(idx2);
            }
            idx2 += 1;
        }
        if line_idxs.len() > 1 {
            line_idxs.iter().combinations(2).for_each(|v| {
                if correct_connected(idx1, v[0], v[1], &src_vec) {
                    res += do_count_rectangle(idx1 + 1, v[0], v[1], &src_vec);
                }
            })
        }
        idx1 += 1;
    }
    res
}

fn do_count_rectangle(mut idx1: usize, idx21: &usize, idx22: &usize, src: &Vec<Vec<char>>) -> u32{
    let mut res = 0;
    while idx1 < src.len() {
        if src[idx1][*idx21] == '+' && src[idx1][*idx22] == '+' {
            if correct_connected(idx1, idx21, idx22, src) {
                res += 1;
            }
        }else if (src[idx1][*idx21] != '|' && src[idx1][*idx21] != '+') ||
            (src[idx1][*idx22] != '|' && src[idx1][*idx22] != '+') {
            break;
        }
        idx1 += 1;
    }
    res
}

fn correct_connected(idx1: usize, idx21: &usize, idx22: &usize, src: &Vec<Vec<char>>) -> bool {
    for i in *idx21+1..*idx22 {
        if src[idx1][i] != '+' && src[idx1][i] != '-' {
            return false;
        }
    }
    true
}

