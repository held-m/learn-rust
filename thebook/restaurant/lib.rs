#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod my_hh;

// pub fn get_hh(){
//     hh::hh::jj()
// }

pub struct Home {
    pub doors: String,
    pub windows: String
}

impl Home {
    pub fn my_home(doors: String) -> Home {
        Home {
            doors,
            windows: String::from("hohohoh"),
        }
    }

    pub fn get_window(&self) -> &String {
        return &self.windows;
    }


}