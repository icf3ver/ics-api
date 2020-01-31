extern crate reqwest;
extern crate ical;

use std::io::BufReader;
use std::io::copy;
use std::fs::File;
use ical::property::Property;
use std::option::Option::None;
use std::option::Option::Some;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap< String, (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn download(){
    let mut resp = reqwest::get("https://calendar.google.com/calendar/ical/wellesleyps.org_qvqdo2dkb151vorl1f0dkigin0%40group.calendar.google.com/public/basic.ics").expect("request failed");
    let mut out = File::create("schedule.ics").expect("failed to create file");
    copy(&mut resp, &mut out).expect("failed to copy content");
}

pub fn parse(){
    let buf = BufReader::new(File::open("schedule.ics")
        .unwrap());

    let reader = ical::PropertyParser::from_reader(buf);

    let mut map = HASHMAP.lock().unwrap();

    let date: (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) = (None, None, None, None, None, None);

    let (mut dtstart, mut dtend, mut description, mut location, mut confirmation, mut summary) = date;
    for line in reader{
        if line.is_ok(){
            let prop = line.unwrap();
            //println!("{:?}", prop);
            let (name, params, value) = (prop.name, prop.params, prop.value);
            if name == "BEGIN" && dtstart != None{
                let date: String = dtstart.clone()
                    .unwrap()[..8].to_string();
                if &date[..4] == "2020" || &date[..4] == "2019" {
                    //println!("{:?}", date);
                    map.insert(date, (dtstart.clone(), dtend.clone(), description.clone(), location.clone(), confirmation.clone(), summary.clone()));
                }
                dtstart = None;
                dtend = None;
                description = None;
                location = None;
                confirmation = None;
                summary = None;
            }else if name == "DTSTART"{
                dtstart = value;
            }else if name == "DTEND"{
                dtend = value;
            }else if name == "DESCRIPTION"{
                description = value;
            }else if name == "LOCATION"{
                location = value;
            }else if name == "CONFIRMATION"{
                confirmation = value;
            }else if name == "SUMMARY"{
                summary = value;
            }
        }
    }
}

pub fn date_event(month: u8, day: u8, year: u16) -> String{
    let mut map = HASHMAP.lock().unwrap();
    let time: String = vec![format!("{:0>4}", year).to_string(), format!("{:0>2}", month).to_string(), format!("{:0>2}", day).to_string()]
        .connect("")
        .to_string();
    println!("{:?}", time);
    if map.contains_key(&time){
        let result = map.get(&time)
            .unwrap().clone().5
            .unwrap();
        return result;
    }
    return "No Entry Found".to_string();
}