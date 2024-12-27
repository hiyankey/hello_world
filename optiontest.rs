
pub fn test_option_type()-> Option<u8>{
let mut opt1: Option<u8> = None;
opt1 = Some(42);
return  opt1;
}
pub fn test_option_string()-> Option<String>{
let mut opt1: Option<String> = None;
opt1 = Some("Hello from option test".to_string());
return  opt1;
}
pub fn test_option_arch()-> Option<ArcheType>{
let mut opt1: Option<ArcheType> = None;
opt1 = Some(ArcheType::Designer);
return  opt1;
}

pub enum ArcheType {
    Designer,
    Developer,

}

impl ToString for ArcheType {
fn to_string(&self) -> String {
    match self {
        ArcheType::Designer => "Designer",
        ArcheType::Developer => "Developer",
    }.to_string()
}
}