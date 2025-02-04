use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::{BTreeMap, HashMap};
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct ParsedJourneyPattern {
    order: BTreeMap<i32, String>,
    points: HashMap<String, String>,
}

macro_rules! netex_frames {
    // taken from vec! macro
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(
            std::boxed::Box::new(["PublicationDelivery", "dataObjects", "CompositeFrame", "frames", $($x),+])
        )
    );
}

pub fn parse_netex<R: BufRead>(mut reader: Reader<R>) -> anyhow::Result<Vec<Vec<String>>> {
    let mut path = Vec::with_capacity(64);
    let mut buffer = Vec::new();

    let mut id = None;
    let mut id_pattern = None;

    let mut stop_place2name_type: HashMap<String, Option<String>> = HashMap::new();
    let mut passenger_stops: Vec<(Option<String>, Option<String>)> = Vec::new();

    let mut journey_patterns: HashMap<String, ParsedJourneyPattern> = HashMap::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(ref e)) => {
                path.push(String::from_utf8(Vec::from(e.name().0))?);
                if path_vec_eq(
                    &path,
                    netex_frames!["ServiceFrame", "scheduledStopPoints", "ScheduledStopPoint"],
                ) {
                    id = Some(
                        e.try_get_attribute("id")?
                            .unwrap()
                            .unescape_value()?
                            .to_string(),
                    );
                } else if path_vec_eq(
                    &path,
                    netex_frames!["ServiceFrame", "stopAssignments", "PassengerStopAssignment"],
                ) {
                    passenger_stops.push((None, None));
                } else if path_vec_eq(
                    &path,
                    netex_frames!["ServiceFrame", "journeyPatterns", "ServiceJourneyPattern"],
                ) {
                    id_pattern = Some(
                        e.try_get_attribute("id")?
                            .unwrap()
                            .unescape_value()?
                            .to_string(),
                    );
                    journey_patterns.insert(
                        id_pattern.clone().unwrap().clone(),
                        ParsedJourneyPattern {
                            order: BTreeMap::new(),
                            points: HashMap::new(),
                        },
                    );
                } else if path_vec_eq(
                    &path,
                    netex_frames![
                        "ServiceFrame",
                        "journeyPatterns",
                        "ServiceJourneyPattern",
                        "pointsInSequence",
                        "StopPointInJourneyPattern"
                    ],
                ) {
                    id = Some(
                        e.try_get_attribute("id")?
                            .unwrap()
                            .unescape_value()?
                            .to_string(),
                    );
                    let order =
                        i32::from_str(&*e.try_get_attribute("order")?.unwrap().unescape_value()?)?;
                    journey_patterns
                        .get_mut(&id_pattern.clone().unwrap())
                        .unwrap()
                        .order
                        .insert(order, id.clone().unwrap().clone());
                } else if path_vec_eq(&path, netex_frames!["SiteFrame", "stopPlaces", "StopPlace"])
                {
                    id = Some(
                        e.try_get_attribute("id")?
                            .unwrap()
                            .unescape_value()?
                            .to_string(),
                    );
                    stop_place2name_type.insert(id.clone().unwrap().clone(), None);
                }
            }
            Ok(Event::Empty(e)) => {
                path.push(String::from_utf8(Vec::from(e.name().0))?);
                if path_vec_eq(
                    &path,
                    netex_frames![
                        "ServiceFrame",
                        "journeyPatterns",
                        "ServiceJourneyPattern",
                        "pointsInSequence",
                        "StopPointInJourneyPattern",
                        "ScheduledStopPointRef"
                    ],
                ) {
                    journey_patterns
                        .get_mut(&id_pattern.clone().unwrap())
                        .unwrap()
                        .points
                        .insert(
                            id.clone().unwrap().clone(),
                            e.try_get_attribute("ref")?
                                .unwrap()
                                .unescape_value()?
                                .to_string(),
                        );
                } else if path_vec_eq(
                    &path,
                    netex_frames![
                        "ServiceFrame",
                        "stopAssignments",
                        "PassengerStopAssignment",
                        "ScheduledStopPointRef"
                    ],
                ) {
                    passenger_stops.last_mut().unwrap().0 = Some(
                        e.try_get_attribute("ref")?
                            .unwrap()
                            .unescape_value()?
                            .to_string(),
                    );
                } else if path_vec_eq(
                    &path,
                    netex_frames![
                        "ServiceFrame",
                        "stopAssignments",
                        "PassengerStopAssignment",
                        "StopPlaceRef"
                    ],
                ) {
                    passenger_stops.last_mut().unwrap().1 = Some(
                        e.try_get_attribute("ref")?
                            .unwrap()
                            .unescape_value()?
                            .to_string(),
                    );
                }
                path.pop();
            }
            Ok(Event::End(_)) => {
                path.pop();
            }
            Ok(Event::Text(e)) => {
                if path_vec_eq(
                    &path,
                    netex_frames!["SiteFrame", "stopPlaces", "StopPlace", "Name"],
                ) {
                    stop_place2name_type
                        .insert(id.clone().unwrap(), Some(e.unescape()?.to_string()));
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(_) => {}
        }
    }

    let mut new_stops = Vec::new();
    let mut idx_stops = HashMap::new();
    for (sched_stop_ref, stop_place_ref) in &passenger_stops {
        idx_stops.insert(sched_stop_ref.clone().unwrap(), new_stops.len());
        new_stops.push(
            stop_place2name_type[&stop_place_ref.clone().unwrap()]
                .clone()
                .unwrap()
                .clone(),
        );
    }

    let mut stop_chains = Vec::new();
    for (_, pattern) in journey_patterns {
        let mut stop_chain = Vec::new();
        for (_, stop_point) in pattern.order {
            stop_chain.push(new_stops[idx_stops[&pattern.points[&stop_point]]].clone());
        }
        stop_chains.push(stop_chain);
    }

    Ok(stop_chains)
}

fn path_vec_eq(left_path: &Vec<String>, rigth_path: Vec<&str>) -> bool {
    if left_path.len() != rigth_path.len() {
        return false;
    }
    left_path.iter().zip(rigth_path.iter()).all(|(a, b)| a == b)
}
