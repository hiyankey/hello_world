 

 struct Company {
    number_of_workers: i32,
    ceo: String,
 }

 pub fn test_closures(){
let mut  vercel = Company{number_of_workers: 42, ceo: "rauchg".to_string()};
let mut change_ceo = |name_of_ceo| vercel.ceo = name_of_ceo;
change_ceo("Lee Rob".to_string());
change_ceo("Rauno Freiberge".to_string());
println!("{}", vercel.ceo);
 }