use scroll_strategist::{graph::ItemState, scroll, stats::Stats};
use serde::Deserialize;
use std::{fs::File, io, path::Path};

#[derive(Deserialize, Debug)]
pub struct Input {
    pub stats: Vec<u16>,
    pub slots: u8,
    pub scrolls: Vec<Scroll>,
    pub goal: Vec<u16>,
}

#[derive(Deserialize, Debug)]
pub struct Scroll {
    pub percent: f64,
    pub dark: bool,
    pub cost: f64,
    pub stats: Vec<u16>,
}

pub fn read_from_file<'a, P: AsRef<Path>>(
    json_path: P,
) -> io::Result<(ItemState<'a>, Vec<scroll::Scroll>, Stats)> {
    let mut json_file = File::open(json_path)?;
    let json_input: Input = serde_json::from_reader(&mut json_file)?;

    Ok((
        ItemState::new_exists(
            json_input.slots,
            Stats::from_vec(json_input.stats),
        ),
        json_input
            .scrolls
            .into_iter()
            .map(|s| {
                scroll::Scroll::new(
                    s.percent / 100.0,
                    s.dark,
                    s.cost,
                    Stats::from_vec(s.stats),
                )
            })
            .collect(),
        Stats::from_vec(json_input.goal),
    ))
}
