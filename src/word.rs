use std::vec;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug)]
pub struct Word {
    pub origin: String,
    pub pro: String,
    pub hw: String,
    pub def:  Vec<String>,
}

pub fn new_word(origin: String,json_str: String) -> Result<Word> {
    let r: Vec<Root> = serde_json::from_str(&json_str).expect("correct format string");
    let r = r[0].clone();
    let mut meaning = vec![];

    for def_item in r.def.unwrap()[0].sseq.iter() {
      let sense = &def_item[0].1.dt;
      for item in sense.iter(){
         meaning.push(item.1.to_string().replace("\"", ""));
      }        
    }

    let word = Word {
        origin: origin,
        pro: r.hwi.prs.unwrap()[0].mw.clone(),
        hw: r.hwi.hw,
        def: meaning,
    };
    Ok(word)
}


#[test]
fn test_new_word() {
   let input: &str = r#"
[{"meta":{"id":"tonic:1","uuid":"f54f3399-6355-4748-b86a-2e7894e445ae","sort":"200272800","src":"collegiate","section":"alpha","stems":["tonic","tonics"],"offensive":false},"hom":1,"hwi":{"hw":"ton*ic","prs":[{"mw":"\u02c8t\u00e4-nik","sound":{"audio":"tonic001","ref":"c","stat":"1"}}]},"fl":"noun","def":[{"sseq":[[["sense",{"sn":"1 a","dt":[["text","{bc}{sx|tonic water||}"]]}],["sense",{"sn":"b","dt":[["text","{bc}an agent (such as a drug) that increases body tone"]]}],["sense",{"sn":"c","dt":[["text","{bc}one that invigorates, restores, refreshes, or stimulates "],["vis",[{"t":"a day in the country was a {wi}tonic{\/wi} for him"}]]]}],["sense",{"sn":"d","dt":[["text","{bc}a liquid preparation for the scalp or hair"]]}],["sense",{"sn":"e","sls":["chiefly New England"],"dt":[["text","{bc}a carbonated flavored beverage"]]}]],[["sense",{"sn":"2","dt":[["text","{bc}the first tone of a major or minor scale {bc}{sx|keynote||}"]]}]],[["sense",{"sn":"3","dt":[["text","{bc}a voiced sound"]]}]]]}],"date":"1797{ds||2||}","shortdef":["tonic water","an agent (such as a drug) that increases body tone","one that invigorates, restores, refreshes, or stimulates"]},{"meta":{"id":"tonic:2","uuid":"ff53f216-1532-45a1-96b9-add0c8fad391","sort":"200272900","src":"collegiate","section":"alpha","stems":["tonic","tonically"],"offensive":false},"hom":2,"hwi":{"hw":"tonic"},"fl":"adjective","def":[{"sseq":[[["sense",{"sn":"1 a","dt":[["text","{bc}characterized by tonus "],["vis",[{"t":"{wi}tonic{\/wi} contraction of muscle"}]]],"sdsense":{"sd":"also","dt":[["text","{bc}marked by prolonged muscular contraction "],["vis",[{"t":"{wi}tonic{\/wi} convulsions"}]]]}}],["sense",{"sn":"b","dt":[["text","{bc}producing or adapted to produce healthy muscular condition and reaction of organs (such as muscles)"]]}]],[["sense",{"sn":"2 a","dt":[["text","{bc}increasing or restoring physical or mental tone {bc}{sx|refreshing||}"]]}],["sense",{"sn":"b","dt":[["text","{bc}yielding a tonic substance"]]}]],[["sense",{"sn":"3","dt":[["text","{bc}relating to or based on the first tone of a scale "],["vis",[{"t":"{wi}tonic{\/wi} harmony"}]]]}]],[["sense",{"sn":"4","sls":["of a syllable"],"dt":[["text","{bc}bearing a principal stress or accent"]]}]],[["sense",{"sn":"5","dt":[["text","{bc}of or relating to speech tones or to languages using them to distinguish words otherwise identical"]]}]]]}],"uros":[{"ure":"ton*i*cal*ly","prs":[{"mw":"\u02c8t\u00e4-ni-k(\u0259-)l\u0113","sound":{"audio":"tonic002","ref":"c","stat":"1"}}],"fl":"adverb"}],"et":[["text","Greek {it}tonikos{\/it}, from {it}tonos{\/it} tension, tone"]],"date":"1649{ds||1|a|}","shortdef":["characterized by tonus; also : marked by prolonged muscular contraction","producing or adapted to produce healthy muscular condition and reaction of organs (such as muscles)","increasing or restoring physical or mental tone : refreshing"]},{"meta":{"id":"tonic accent","uuid":"0ce54b11-8142-495e-89d1-bada3c996f03","sort":"200273000","src":"collegiate","section":"alpha","stems":["tonic accent","tonic accents"],"offensive":false},"hwi":{"hw":"tonic accent"},"fl":"noun","def":[{"sseq":[[["sense",{"sn":"1","dt":[["text","{bc}relative phonetic prominence (as from greater stress or higher pitch) of a spoken syllable or word"]]}]],[["sense",{"sn":"2","dt":[["text","{bc}accent depending on pitch rather than stress"]]}]]]}],"date":"1867{ds||1||}","shortdef":["relative phonetic prominence (as from greater stress or higher pitch) of a spoken syllable or word","accent depending on pitch rather than stress"]},{"meta":{"id":"tonic sol-fa","uuid":"f5d09b6a-0130-4e41-90a5-73615510094b","sort":"200273200","src":"collegiate","section":"alpha","stems":["tonic sol-fa","tonic sol-fas"],"offensive":false},"hwi":{"hw":"tonic sol-fa"},"fl":"noun","def":[{"sseq":[[["sense",{"dt":[["text","{bc}a system of solmization based on key relationships that replaces the normal notation with sol-fa syllables or their initials"]]}]]]}],"date":"1852","shortdef":["a system of solmization based on key relationships that replaces the normal notation with sol-fa syllables or their initials"]},{"meta":{"id":"tonic water","uuid":"ab0de1c4-ae67-4fa8-9803-5a839e650e41","sort":"200273300","src":"collegiate","section":"alpha","stems":["tonic water"],"offensive":false},"hwi":{"hw":"tonic water"},"fl":"noun","def":[{"sseq":[[["sense",{"dt":[["text","{bc}a carbonated beverage flavored with a small amount of quinine, lemon, and lime"]]}]]]}],"date":"1903","shortdef":["a carbonated beverage flavored with a small amount of quinine, lemon, and lime"]}]
"#;
   let w = new_word("".to_string(), input.to_string()).unwrap();
   assert_eq!(w.pro, "ˈtä-nik");
   assert_eq!(w.hw, "ton*ic");
   assert_eq!(
      w.def,
      vec![
         "{bc}{sx|tonic water||}",
         "{bc}the first tone of a major or minor scale {bc}{sx|keynote||}",
         "{bc}a voiced sound",
      ]
   );
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
   pub hwi: Hwi,
   pub def: Option<Vec<Def>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hwi {
    pub hw: String,
    pub prs: Option<Vec<Pr>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pr {
    pub mw: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Def {
    pub sseq: Vec<Vec<(String, Sseq)>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sseq {
    pub dt: Vec<(String, Value)>,
    pub sdsense: Option<Sdsense>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sdsense {
    pub sd: String,
    pub dt: Vec<(String, Value)>,
}

