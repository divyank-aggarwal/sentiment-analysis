use csv::{Error, Reader, StringRecord, Writer};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::{fs, str::SplitAsciiWhitespace};
#[derive(Deserialize, Debug, Serialize)]
struct Record {
    target: Option<String>,
    id: Option<String>,
    date: Option<String>,
    flag: Option<String>,
    user: Option<String>,
    text: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct NewRecord {
    text: Option<String>,
    target: Option<u8>,
    count: Option<u16>,
}

struct DataSet {
    headers: StringRecord,
    records: Vec<StringRecord>,
}

impl DataSet {
    fn new(headers: StringRecord, records: Vec<StringRecord>) -> Self {
        DataSet { headers, records }
    }
}
fn main() -> Result<(), Error> {
    let content = fs::read_to_string("training.csv").expect("Please run");
    // println!("The file contents are \n{}", content);
    let mut reader = Reader::from_reader(content.as_bytes());
    let mut writer = Writer::from_writer(vec![]);
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
    // for record in reader.records() {
    //     let record = record?;
    // }
    println!("{:?}", reader.headers());
    let mut x = 0;
    for record in reader.deserialize() {
        let record: Record = record?;
        let mut cleaned_string = String::from("Hello");
        writer.serialize(NewRecord {
            text: match record.text {
                None => None,
                Some(str) => {
                    cleaned_string = str
                        .replace(
                            &[
                                '.', '!', '#', '$', '%', ',', '\'', '^', '&', '*', '(', ')', '_',
                                '-', '{', '}', '+', '=', '[', ']', '|', '`', '~', '"', '\\', ':',
                                ';', '<', '>', '?', '/',
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
                        .replace("I am", "Im")
                        .split_ascii_whitespace()
                        .filter(|x| {
                            if x.starts_with('@') {
                                false
                            } else if set.is_match(x) {
                                false
                            } else {
                                true
                            }
                        })
                        .map(|x| format!("{} ", x.to_lowercase()))
                        .collect::<String>();
                    Some(cleaned_string.clone())
                }
            },
            target: match record.target {
                None => None,
                Some(x) => match x.parse::<u8>() {
                    Ok(y) => Some(y),
                    _ => None,
                },
            },
            count: match cleaned_string {
                x => {
                    let mut count: u16 = 0;
                    let mut iter = x.split_ascii_whitespace();
                    while let Some(_) = iter.next() {
                        count += 1;
                    }
                    Some(count)
                }
            },
        })?;
        x += 1;
    }
    println!("{}", x);
    let contents =
        String::from_utf8(writer.into_inner().expect("Please pass")).expect("Please pass");
    fs::write("cleaned.csv", contents).expect("Please pass");
    Ok(())
}
