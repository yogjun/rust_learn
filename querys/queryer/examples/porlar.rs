
use polars::prelude::*;

fn main() {
//     let lf1 = LazyFrame::scan_parquet("myfile_1.parquet".into(), Default::default())?
//     .groupby([col("ham")])
//     .agg([
//         // expressions can be combined into powerful aggregations
//         col("foo")
//             .sort_by([col("ham").rank(Default::default())], [false])
//             .last()
//             .alias("last_foo_ranked_by_ham"),
//         // every expression runs in parallel
//         col("foo").cummin(false).alias("cumulative_min_per_group"),
//         // every expression runs in parallel
//         col("foo").reverse().list().alias("reverse_group"),
//     ]);

// let lf2 = LazyFrame::scan_parquet("myfile_2.parquet".into(), Default::default())?
//     .select([col("ham"), col("spam")]);

// let df = lf1
//     .join(lf2, [col("reverse_group")], [col("foo")], JoinType::Left)
//     // now we finally materialize the result.
//     .collect()?;
}