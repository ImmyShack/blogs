use candid::CandidType;
use ic_cdk::api::time;

#[derive(Clone, CandidType)]
pub struct Blog {
    title: String,
    // date: u64,
    content: String,
    tags: Vec<String>
}
//unsigned: u8 0-254 2^8 liczba nieujemna która jest potegą 2
impl Blog {
     pub fn new(title: String, /*date: u64,*/ content: String, tags: Vec<String>) -> Self{
        Self {
            title,
            //date: time(),
            content,
            tags
        }
    }
}