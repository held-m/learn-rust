use part17_1::AveragedCollection;

fn main() {
    let _my_vec: Vec<i32> = vec![5, 6, 23, 44];

    let mut v1 = Vec::new();
    v1.push(5);

    let mut my_average = AveragedCollection::new();

    my_average.add(20);
    my_average.add(10);
    my_average.add(20);
    my_average.add(20);
    println!("my average: {}", my_average.average());
    println!("my average: {:?}", v1);
    println!("my average: {:?}", _my_vec);

}
