pub struct SqlIterator<'a> {
    sql: &'a str,
}

impl<'a> SqlIterator<'a> {
    pub fn new(sql: &'a str) -> Self {
        SqlIterator { sql: sql.trim() }
    }
}

impl<'a> Iterator for SqlIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut in_dollar = false;
        let mut ic = self.sql.char_indices();
        let mut num_hyphens = 0;
        let mut num_dollars = 0;

        while let Some((idx, c)) = ic.next() {
            if c == '-' {
                num_hyphens += 1;

                // If we have two hyphens in a row, we're in a line comment
                // so we should ignore everything until the next newline.
                if num_hyphens == 2 {
                    num_hyphens = 0;

                    for (_, c) in ic.by_ref() {
                        if c == '\n' {
                            break;
                        }
                    }
                }

                continue;
            }

            if c == '$' {
                num_dollars += 1;

                // If we have two dollars in a row, we're in a dollar-delimited block
                // and we should ignore semi-colons until we see two more dollars.
                if num_dollars == 2 {
                    num_dollars = 0;
                    in_dollar = !in_dollar;
                }

                continue;
            }

            num_hyphens = 0;
            num_dollars = 0;

            if c == ';' && !in_dollar {
                let res = Some(&self.sql[..idx]);
                self.sql = &self.sql[idx + 1..];
                return res;
            }
        }

        None
    }
}
