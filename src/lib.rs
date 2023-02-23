pub mod thicc{
    pub fn char_to_thicc(input: char) -> char{
        match input{
            'a'|'A' => '卂',
            'b'|'B' => '乃',
            'c'|'C' => '匚',
            'd'|'D' => '刀',
            'e'|'E' => '乇',
            'f'|'F' => '下',
            'g'|'G' => '厶',
            'h'|'H' => '卄',
            'i'|'I' => '工',
            'j'|'J' => '丁',
            'k'|'K' => '长',
            'l'|'L' => '乚',
            'm'|'M' => '从',
            'n'|'N' => '𠘨',
            'o'|'O' => '口',
            'p'|'P' => '尸',
            'q'|'Q' => '㔿',
            'r'|'R' => '尺',
            's'|'S' => '丂',
            't'|'T' => '丅',
            'u'|'U' => '凵',
            'v'|'V' => 'リ',
            'w'|'W' => '山',
            'x'|'X' => '乂',
            'y'|'Y' => '丫',
            'z'|'Z' => '乙',
            other => other
        }
    }

    pub fn str_to_thicc(input: &str) -> String{
        input.chars().map(|c|char_to_thicc(c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
