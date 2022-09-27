mod WordnikRetrieval;
use crate::WordnikRetrieval::IPhraseRetrieval;


fn main() {
    let users = WordnikRetrieval::WordNikPhraseRetrieval::new();
    let list = users.ReturnPhraseList();
    for i in list{
        println!("{}", i);
    }
}
