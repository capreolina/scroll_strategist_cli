#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(deprecated)]

use scroll_strategist::stats::Stats;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!(
        "=== Welcome to scroll_strategist_cli! ===
Just answer some questions about what you're scrolling, what you're scrolling
it with, and what you want your scrolling strategy to optimise for, and
scroll_strategist can work its magic~\n",
    );

    let stdin = io::stdin();
    let mut input_buf = String::new();

    let mut stats: Option<Stats> = None;
    'stats: while stats.is_none() {
        print!("Stats: ");
        io::stdout().flush()?;
        stdin.read_line(&mut input_buf)?;

        let mut stats_vec = Vec::new();
        for maybe_stat in input_buf
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().ok())
        {
            if let Some(stat) = maybe_stat {
                stats_vec.push(stat);
            } else {
                input_buf.clear();

                continue 'stats;
            }
        }
        stats = Some(Stats::from_vec(stats_vec));

        input_buf.clear();
    }

    let mut slots: Option<u8> = None;
    while slots.is_none() {
        print!("Slots remaining: ");
        io::stdout().flush()?;
        stdin.read_line(&mut input_buf)?;

        slots = input_buf.trim().parse().ok();

        input_buf.clear();
    }

    Ok(())
}
