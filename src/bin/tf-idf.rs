use csv::{Error, Reader, Writer};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

/*  Assumptions
 1. Word occurence is 1 in each document since tweets are char limited by design.
 2. Therefore Document frequency is just the total count of word in the file.
 3. Term frequency would be 1/(total word count) for any document term.
*/

const COUNT: f32 = 1600000.0;
#[derive(Serialize, Deserialize, Debug)]
struct Record {
    text: String,
    target: u8,
    count: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct TfIdf {
    id: u32,
    target: bool,
    details: Vec<Details>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Details {
    pub word: String,
    pub tf: f32,
    pub df: f32,
    pub tf_idf: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct WriteRecord {
    word: String,
    df: u32,
    tf_idf_good: f32,
    tf_idf_sad: f32,
}

impl TfIdf {
    fn new(id: u32, target: u8) -> Self {
        TfIdf {
            id,
            details: vec![],
            target: match target {
                0 => false,
                _ => true,
            },
        }
    }

    fn insert_word(&mut self, word: String, tf: f32, df: f32) {
        self.details.push(Details {
            word,
            tf,
            df,
            tf_idf: tf * df,
        })
    }
}

fn main() -> Result<(), Error> {
    println!("Hello World");
    let re = Regex::new(r"\d").unwrap();
    let content = fs::read_to_string("cleaned.csv").expect("Please run");
    let mut reader = Reader::from_reader(content.as_bytes());
    let mut writer = Writer::from_writer(vec![]);
    println!("{:?}", reader.headers());
    let mut counts: HashMap<String, u32> = HashMap::new();
    for word in content.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    let mut x = 0;
    let mut arr: Vec<TfIdf> = vec![];
    for record in reader.deserialize() {
        let record: Record = record?;
        let mut temp = TfIdf::new(x, record.target);
        let tf = 1.0 / (record.count as f32);
        for word in record.text.split_whitespace() {
            let df = match counts.entry(word.to_string()).or_insert(0) {
                0 => {
                    continue;
                }
                y => f32::log10(COUNT / *y as f32),
            };
            temp.insert_word(word.to_string(), tf, df)
        }
        arr.push(temp);
        x += 1;
    }
    let mut vector_good: HashMap<String, f32> = HashMap::new();
    let mut vector_sad: HashMap<String, f32> = HashMap::new();
    for element in &arr {
        for detail in &element.details {
            match element.target {
                true => {
                    *vector_good.entry(detail.word.to_string()).or_insert(0.0) += detail.tf_idf;
                }
                false => {
                    *vector_sad.entry(detail.word.to_string()).or_insert(0.0) += detail.tf_idf;
                }
            }
        }
    }

    for (word, df) in counts.into_iter() {
        if !re.is_match(&word) {
            let good = *vector_good.entry(word.clone()).or_insert(0.0);
            let bad = *vector_sad.entry(word.clone()).or_insert(0.0);
            writer.serialize(WriteRecord {
                word,
                df,
                tf_idf_good: good,
                tf_idf_sad: bad,
            })?;
        }
    }
    let final_content =
        String::from_utf8(writer.into_inner().expect("Please pass")).expect("Please pass");
    fs::write("final.csv", final_content).expect("Please pass");
    println!("{:?}", arr[100000]);
    Ok(())
}
