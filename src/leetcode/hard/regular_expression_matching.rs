crate::leetcode::solution!();

struct RegExpToken(u8);

impl RegExpToken {
    fn is_star(&self) -> bool {
        self.0 == b'*'
    }
}

impl std::cmp::PartialEq<u8> for RegExpToken {
    fn eq(&self, other: &u8) -> bool {
        self.0 == b'.' || self.0 == *other
    }
}

struct StrPtr(*const u8);

impl StrPtr {
    fn build(str: &String) -> Self {
        Self(str.as_ptr())
    }

    fn get(&self) -> RegExpToken {
        RegExpToken(unsafe { *self.0 })
    }

    fn next(&self) -> Self {
        Self(self.0.wrapping_offset(1))
    }
}

impl Solution {
    pub fn is_match(word: String, pattern: String) -> bool {
        let space = word.len() + 1;
        let word = word.as_bytes();

        let mut partial_match = vec![false; space];
        let mut level_match = vec![false; space];
        level_match[0] = true;

        let mut pattern_ptr = StrPtr::build(&pattern);
        let mut i = 0;
        while i < pattern.len() {
            std::mem::swap(&mut partial_match, &mut level_match);

            let match_token = pattern_ptr.get();

            if i < pattern.len() - 1 && pattern_ptr.next().get().is_star() {
                let mut j = 0;
                while j < partial_match.len() {
                    while j < partial_match.len() && partial_match[j] == false {
                        j += 1;
                    }

                    if j == partial_match.len() {
                        break;
                    }

                    level_match[j] = true;
                    partial_match[j] = false;
                    j += 1;

                    while j < partial_match.len() && match_token == word[j - 1] {
                        level_match[j] = true;
                        partial_match[j] = false;

                        j += 1;
                    }
                }

                pattern_ptr = pattern_ptr.next();
                i += 1;
            } else {
                for j in 1..space {
                    if partial_match[j - 1] == true && match_token == word[j - 1] {
                        level_match[j] = true;
                    }
                    partial_match[j - 1] = false;
                }
                partial_match[space - 1] = false;
            }

            pattern_ptr = pattern_ptr.next();
            i += 1;
        }

        level_match[word.len()]
    }
}
