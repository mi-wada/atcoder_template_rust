use std::collections::HashMap;

fn cin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

fn cin_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    cin()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn main() {
    let hwn = cin_vec::<usize>();
    let (_h, _w, n) = (hwn[0], hwn[1], hwn[2]);
    let mut row = vec![0; n];
    let mut column = vec![0; n];
    for i in 0..n {
        let ab = cin_vec::<i32>();
        row[i] = ab[0];
        column[i] = ab[1];
    }

    let mut sorted_row = row.clone();
    sorted_row.sort();
    let row_compress: HashMap<i32, i32> = sorted_row
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, (i+1) as i32))
        .collect();

    let mut sorted_column = column.clone();
    sorted_column.sort();
    let column_compress: HashMap<i32, i32> = sorted_column
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, (i+1) as i32))
        .collect();

    println!("{:#?}", row_compress);

    for i in 0..n {
        println!(
            "{} {}",
            row_compress.get(&row[i]).unwrap(),
            column_compress.get(&column[i]).unwrap()
        );
    }
}
