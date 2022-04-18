use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "select a a1,b,123,myfun(b),* FROM datasource WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20 \
    order by a desc,b limit 50 offset 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}",ast)
}
