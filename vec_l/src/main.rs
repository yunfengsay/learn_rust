fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(3);
    println!("{:?}", v);
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    let third: &i32 = &v[2];
    println!("The Third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => eprintln!("Get None")
    }
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);
    
    let v = vec![100, 20, 30];
    for i in v {
        println!("{}", i);
    }
    
    let mut v = vec![1,2,3,100,200];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    
    enum Persion{
        Age(i32),
        Name(String),
    }
    let row = vec![
        Persion::Age(27),
        Persion::Age(18),
        Persion::Name(String::from("yunfeng")),
    ];
}
