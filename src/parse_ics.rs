use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<
        HashMap<
            String,
            Event,
        >,
    > = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct Event {
    pub dtstart: Option<String>,
    pub dtend: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub confirmation: Option<String>,
    pub summary: Option<String>
}

pub fn download() {
    let mut resp = reqwest::get("https://calendar.google.com/calendar/ical/wellesleyps.org_qvqdo2dkb151vorl1f0dkigin0%40group.calendar.google.com/public/basic.ics").expect("request failed");
    let mut out = File::create("schedule.ics").expect("failed to create file");
    copy(&mut resp, &mut out).expect("failed to copy content");
}

pub fn parse() {
    let buf = BufReader::new(File::open("schedule.ics").unwrap());

    let reader = ical::PropertyParser::from_reader(buf);

    let mut map = HASHMAP.lock().unwrap();

    let mut event = Event {
        dtstart: None,
        dtend: None,
        description: None,
        location: None,
        confirmation: None,
        summary: None
    };

    for line in reader{
        if line.is_ok(){
            let prop = line.unwrap();
            let (name, params, value) = (prop.name, prop.params, prop.value);
            if name == "BEGIN" && event.dtstart != None{
                let date: String = event.dtstart.clone()
                    .unwrap()[..8].to_string();
                map.insert(date, event.clone());

                let event = Event {
                    dtstart: None,
                    dtend: None,
                    description: None,
                    location: None,
                    confirmation: None,
                    summary: None
                };
            }else if name == "DTSTART"{
                event.dtstart = value.clone();
            }else if name == "DTEND"{
                event.dtend = value.clone();
            }else if name == "DESCRIPTION"{
                event.description = value.clone();
            }else if name == "LOCATION"{
                event.location = value.clone();
            }else if name == "CONFIRMATION"{
                event.confirmation = value;
            }else if name == "SUMMARY"{
                event.summary = value;
            }
        }
    }
}

pub fn date_event(month: u8, day: u8, year: u16) -> String {
    let mut map = HASHMAP.lock().unwrap();
    let time: String = vec![
        format!("{:0>4}", year).to_string(),
        format!("{:0>2}", month).to_string(),
        format!("{:0>2}", day).to_string(),
    ]
    .connect("")
    .to_string();
    println!("{:?}", time);
    if map.contains_key(&time) {
        let result = map.get(&time).unwrap().clone().summary.unwrap();
        return result;
    }
    return "No Entry Found".to_string();
}
