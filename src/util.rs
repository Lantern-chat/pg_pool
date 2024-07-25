struct SqlIterator<'a> {
    sql: &'a str,
}

impl<'a> SqlIterator<'a> {
    pub fn new(sql: &'a str) -> Self {
        SqlIterator { sql }
    }
}

impl<'a> Iterator for SqlIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut in_dollar = false;
        let mut ic = self.sql.char_indices().peekable();

        loop {
            if let Some((idx, c)) = ic.next() {
                if c == '$' && ic.peek().map(|(_, c)| *c == '$') == Some(true) {
                    in_dollar = !in_dollar;
                }

                if c == ';' && !in_dollar {
                    let res = Some(&self.sql[..idx]);
                    self.sql = &self.sql[idx + 1..];
                    return res;
                }
            } else {
                return None;
            }
        }
    }
}
