use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct TyrDialect;

// 创建Dialect 方言 要identifier支持url
impl Dialect for TyrDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..'Z').contains(&ch) || '_' == ch
    }

    // identifier 可以有 ':' '/' '?' '&' '='
    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}


pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/yogjun/rust_learn/main/queryer/csv/co.csv";
    let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5", url );
    sql
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&TyrDialect::default(), &example_sql()).is_ok());
    }
}