pub mod utils {

    pub fn get_full_name(first: &str, last: &str)-> String {
        let full_name = format!("{0} {1}", first, last);
        return full_name;
     }
}

pub mod db {
    pub fn query(method: &str)-> String {
        return method.to_string();
    }
}
