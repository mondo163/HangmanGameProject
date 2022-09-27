use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};

//trait for multiple forms of phrase retrieval - strategy pattern
pub trait IPhraseRetrieval {
    fn ReturnPhraseList(&self) -> Vec<String>;
}

//WordNik api specfic struct for phrase retrieval 
pub struct WordNikPhraseRetrieval { 
    apiKey: String,
    hasDictDef: String,
    maxCorpusCount: i64,
    minDictCount: i64,
    maxDictCount: i64,
    minLength: i64,
    maxLength: i64,
    limIt: i64
}

//implementing the trait function on the WordNik struct
impl IPhraseRetrieval for WordNikPhraseRetrieval{
    fn ReturnPhraseList(&self) -> Vec<String>{
        let res = WordNikPhraseRetrieval::get_word_list(self);
        let wordinfo_col = res.unwrap();
        let mut only_words: Vec<String> = Vec::new();
        for i in wordinfo_col{
            only_words.push(i.word)
        }
        
        return only_words
    }
}

//
impl WordNikPhraseRetrieval{
    pub fn new()->WordNikPhraseRetrieval{
        return WordNikPhraseRetrieval{
                apiKey: String::from("raakvjl6oj1zwel88g9x7ia3plyhqd15o4acbav68x8tihxul"),
                hasDictDef: String::from("true"),
                maxCorpusCount: -1,
                minDictCount: 1,
                maxDictCount: 10,
                minLength: 7,
                maxLength: 20,
                limIt: 200  
        };
    }

    #[tokio::main]
    async fn get_word_list(&self) -> Result<Vec<WordInfo>,ExitFailure>{
        let url = format!(
            "https://api.wordnik.com/v4/words.json/randomWords?hasDictionaryDef={}&maxCorpusCount={}&minDictionaryCount={}&maxDictionaryCount={}&minLength={}&maxLength={}&limit={}&api_key={}",
             self.hasDictDef, self.maxCorpusCount.to_string(), self.minDictCount.to_string(),
             self.maxDictCount.to_string(), self.minLength.to_string(), self.maxLength.to_string(),
             self.limIt.to_string(),self.apiKey
        );

        let response = reqwest::get(&url).await?;

        let wordlist: Vec<WordInfo> = response.json().await?;

        Ok(wordlist)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct WordInfo{
    id: i64,
    word: String,
}