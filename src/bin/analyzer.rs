use csv::{Error, Reader};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, hash::Hash, io};

const TOTAL_COUNT: f32 = 1600000.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WriteRecord {
    word: String,
    df: u32,
    tf_idf_good: f32,
    tf_idf_sad: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Details {
    pub word: String,
    pub tf: f32,
    pub df: f32,
    pub tf_idf: f32,
}

fn main() -> Result<(), Error> {
    let set = RegexSet::new(&[
        r"(?i)^iam$",
        r"(?i)^im$",
        r"(?i)^either$",
        r"(?i)^ourselves$",
        r"(?i)^yourself$",
        r"(?i)^but$",
        r"(?i)^again$",
        r"(?i)^there$",
        r"(?i)^about$",
        r"(?i)^once$",
        r"(?i)^during$",
        r"(?i)^out$",
        r"(?i)^very$",
        r"(?i)^having$",
        r"(?i)^with$",
        r"(?i)^they$",
        r"(?i)^own$",
        r"(?i)^an$",
        r"(?i)^be$",
        r"(?i)^some$",
        r"(?i)^for$",
        r"(?i)^do$",
        r"(?i)^its$",
        r"(?i)^yours$",
        r"(?i)^such$",
        r"(?i)^into$",
        r"(?i)^of$",
        r"(?i)^most$",
        r"(?i)^itself$",
        r"(?i)^other$",
        r"(?i)^off$",
        r"(?i)^is$",
        r"(?i)^s$",
        r"(?i)^am$",
        r"(?i)^or$",
        r"(?i)^who$",
        r"(?i)^as$",
        r"(?i)^from$",
        r"(?i)^him$",
        r"(?i)^each$",
        r"(?i)^the$",
        r"(?i)^themselves$",
        r"(?i)^until$",
        r"(?i)^below$",
        r"(?i)^are$",
        r"(?i)^we$",
        r"(?i)^these$",
        r"(?i)^your$",
        r"(?i)^his$",
        r"(?i)^through$",
        r"(?i)^dont$",
        r"(?i)^nor$",
        r"(?i)^me$",
        r"(?i)^were$",
        r"(?i)^her$",
        r"(?i)^more$",
        r"(?i)^himself$",
        r"(?i)^this$",
        r"(?i)^down$",
        r"(?i)^should$",
        r"(?i)^our$",
        r"(?i)^their$",
        r"(?i)^while$",
        r"(?i)^above$",
        r"(?i)^both$",
        r"(?i)^up$",
        r"(?i)^to$",
        r"(?i)^ours$",
        r"(?i)^had$",
        r"(?i)^she$",
        r"(?i)^all$",
        r"(?i)^no$",
        r"(?i)^when$",
        r"(?i)^at$",
        r"(?i)^any$",
        r"(?i)^before$",
        r"(?i)^them$",
        r"(?i)^same$",
        r"(?i)^and$",
        r"(?i)^been$",
        r"(?i)^have$",
        r"(?i)^in$",
        r"(?i)^will$",
        r"(?i)^on$",
        r"(?i)^does$",
        r"(?i)^yourselves$",
        r"(?i)^then$",
        r"(?i)^that$",
        r"(?i)^because$",
        r"(?i)^what$",
        r"(?i)^over$",
        r"(?i)^why$",
        r"(?i)^so$",
        r"(?i)^can$",
        r"(?i)^did$",
        r"(?i)^not$",
        r"(?i)^now$",
        r"(?i)^under$",
        r"(?i)^he$",
        r"(?i)^you$",
        r"(?i)^herself$",
        r"(?i)^has$",
        r"(?i)^just$",
        r"(?i)^where$",
        r"(?i)^too$",
        r"(?i)^only$",
        r"(?i)^myself$",
        r"(?i)^which$",
        r"(?i)^those$",
        r"(?i)^i$",
        r"(?i)^after$",
        r"(?i)^few$",
        r"(?i)^whom$",
        r"(?i)^t$",
        r"(?i)^being$",
        r"(?i)^if$",
        r"(?i)^theirs$",
        r"(?i)^my$",
        r"(?i)^against$",
        r"(?i)^a$",
        r"(?i)^by$",
        r"(?i)^doing$",
        r"(?i)^it$",
        r"(?i)^how$",
        r"(?i)^further$",
        r"(?i)^was$",
        r"(?i)^here$",
        r"(?i)^than$",
        r"\d",
        r"(?i)^r$",
    ])
    .unwrap();
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Please pass");
    let mut hash_input: HashMap<String, u32> = HashMap::new();
    let mut total_count = 0;
    let sentence = sentence
        .replace(
            &[
                '.', '!', '#', '$', '%', ',', '\'', '^', '&', '*', '(', ')', '_', '-', '{', '}',
                '+', '=', '[', ']', '|', '`', '~', '"', '\\', ':', ';', '<', '>', '?', '/',
            ][..],
            "",
        )
        .replace("would not", "wouldnt")
        .replace("could not", "couldnt")
        .replace("will not", "wont")
        .replace("shall not", "shant")
        .replace("should not", "shouldnt")
        .replace("cannot", "cant")
        .replace("can not", "cant")
        .replace("I am", "Im");
    let cleaned_sentence: String = sentence
        .split_whitespace()
        .filter(|x| if set.is_match(x) { false } else { true })
        .map(|x| {
            total_count += 1;
            *hash_input.entry(x.to_string()).or_insert(0) += 1;
            format!("{} ", x)
        })
        .collect();
    let content = fs::read_to_string("final.csv").expect("Please run");
    let mut reader = Reader::from_reader(content.as_bytes());
    let mut magnitude_good = 0.0;
    let mut magnitude_sad = 0.0;
    let mut hash_words: HashMap<String, WriteRecord> = HashMap::new();
    let records: Vec<WriteRecord> = reader
        .deserialize()
        .map(|x| {
            let record: WriteRecord = x.expect("Please pass");
            magnitude_good += record.tf_idf_good * record.tf_idf_good;
            magnitude_sad += record.tf_idf_sad * record.tf_idf_sad;
            hash_words.insert(record.word.clone(), record.clone());
            record
        })
        .collect();
    magnitude_good = f32::sqrt(magnitude_good);
    magnitude_sad = magnitude_sad.sqrt();
    let mut magnitude_input: f32 = 0.0;
    let mut input_model: Vec<Details> = vec![];
    for (word, count) in hash_input {
        let word = word.as_str().to_lowercase();
        let tf = count as f32 / total_count as f32;
        let df = match hash_words.get(&word) {
            None => 1,
            Some(x) => x.df + 1,
        };
        let idf = f32::log10(TOTAL_COUNT / df as f32);
        input_model.push(Details {
            word,
            tf,
            df: idf,
            tf_idf: tf * idf,
        });
        magnitude_input += tf * idf * tf * idf;
    }
    magnitude_input = magnitude_input.sqrt();
    let mut dot_good: f32 = 0.0;
    let mut dot_bad: f32 = 0.0;
    for model in input_model {
        let (good_value, bad_value) = match hash_words.get(&model.word) {
            None => (0.0, 0.0),
            Some(x) => (model.tf_idf * x.tf_idf_good, model.tf_idf * x.tf_idf_sad),
        };
        dot_good += good_value;
        dot_bad += bad_value;
    }
    println!(
        "The good score cosine is {}",
        dot_good / (&magnitude_good * &magnitude_input)
    );
    println!(
        "The sad score cosine is {}",
        dot_bad / (&magnitude_sad * &magnitude_input)
    );
    Ok(())
}
