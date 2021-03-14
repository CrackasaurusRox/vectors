fn main() {
    
    let mut v: Vec<i32> = Vec::new();
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];

    match v.get(2) {
        Some(third) => println!("Third element in the array is {}", third),
        None => println!("There is no third element."),
    }
 
    println!("{:?}", v);

    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    let vv = vec![1,2,3];
    println!("{:?}", vv);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
         SpreadsheetCell::Int(3),
         SpreadsheetCell::Text(String::from("blue")),
         SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}