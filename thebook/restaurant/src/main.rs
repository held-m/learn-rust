#![allow(dead_code)]
// use mylib::my_hh;

// use mylib::Home;

mod oo;


fn main() {
    // my_hh::jj();
    //my_hh::jj();
    // crate::oo::my_hhh::jj();

    oo::my_hh::jj();
    oo::my_hh2::jj();

    // let h1 = Home::my_home("mmymymy home".to_owned());
    //
    // println!("doors: {} ", h1.doors);
    // println!("windows: {}", h1.get_window());
    // let name_window = String::from("aaaaassss");
    // h1.set_window(&name_window);
    // println!("windows: {}", h1.get_window());

}