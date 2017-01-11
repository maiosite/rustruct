// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

static ITEMIZED_BULLET: &'static str = "*+-•‣⁃";

fn detect_itemized_bullet(line: &str) -> bool {
    match line.len() > 2 {
        false => false,
        true => {
            let mut chs = line.chars();
            let first_ch = chs.next().unwrap();
            let second_ch = chs.next().unwrap();
            ITEMIZED_BULLET.chars().any(|ch| ch == first_ch) && second_ch.is_whitespace()
        }
    }
}

pub fn is_itemized(line: &str) -> bool {
    detect_itemized_bullet(line)
}
