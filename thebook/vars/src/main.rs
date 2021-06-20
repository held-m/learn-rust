fn main() {

    let i: i64 = i64::max_value()+1;

    println!("{}", i);

    // let x:i128 = 170141183460469231731687303715884105727;
    let x: i8 = 10;
    let y: i8 = 8;
    let y: f32 = f32::from(y);
    dbg!(y);
    // let x = x + 8;
    // let x = x - 3;
    dbg!(x);

    let tup = (500, "fdfdf", 3.5);
    dbg!(tup);
    let u = tup.1;
    dbg!(tup.1);
    dbg!(u);

    let month: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    dbg!(month[4]);

    let h = [3; 5];
    dbg!(h);
}
