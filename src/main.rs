#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(deprecated)]

use scroll_strategist::{
    dfs::solve_p, graph::ItemState, scroll::Scroll, stats::Stats,
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!(
        "=== Welcome to scroll_strategist_cli! ===
Just answer some questions about what you’re scrolling, what you’re scrolling
it with, and what you want your scrolling strategy to optimise for, and
scroll_strategist can work its magic~\n",
    );

    let stdin = io::stdin();
    let mut input_buf = String::new();

    let stats = {
        let mut stats: Option<Stats> = None;
        'stats: while stats.is_none() {
            input_buf.clear();

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
                    continue 'stats;
                }
            }
            stats = Some(Stats::from_vec(stats_vec));
        }

        stats.unwrap_or_else(|| unreachable!())
    };

    let slots = {
        let mut slots: Option<u8> = None;
        while slots.is_none() {
            input_buf.clear();

            print!("Slots remaining: ");
            io::stdout().flush()?;
            stdin.read_line(&mut input_buf)?;

            slots = input_buf.trim().parse().ok();
        }

        slots.unwrap_or_else(|| unreachable!())
    };

    let mut scrolls = Vec::new();
    'scrolls: loop {
        let mut scroll =
            Scroll::new(0.0, false, 0.0, Stats::from_vec(Vec::new()));

        loop {
            input_buf.clear();

            print!("Scroll {} %: ", scrolls.len() + 1);
            io::stdout().flush()?;
            stdin.read_line(&mut input_buf)?;

            let trimmed = input_buf.trim();
            if trimmed.is_empty() && !scrolls.is_empty() {
                break 'scrolls;
            }
            if let Ok(percentage_points) = trimmed.parse::<f64>() {
                scroll.p_suc = percentage_points / 100.0;

                break;
            }
        }

        loop {
            input_buf.clear();

            print!("Scroll {} dark?: ", scrolls.len() + 1);
            io::stdout().flush()?;
            stdin.read_line(&mut input_buf)?;

            input_buf.make_ascii_lowercase();
            match input_buf.trim() {
                "yes" | "y" | "true" => {
                    scroll.dark = true;

                    break;
                }
                "no" | "n" | "false" => {
                    scroll.dark = false;

                    break;
                }
                _ => (),
            }
        }

        loop {
            input_buf.clear();

            print!("Scroll {} cost: ", scrolls.len() + 1);
            io::stdout().flush()?;
            stdin.read_line(&mut input_buf)?;

            if let Ok(cost) = input_buf.trim().parse() {
                scroll.cost = cost;

                break;
            }
        }

        'scroll_stats: loop {
            input_buf.clear();

            print!("Scroll {} stats: ", scrolls.len() + 1);
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
                    continue 'scroll_stats;
                }
            }

            if stats_vec.len() == stats.len() {
                scroll.stats = Stats::from_vec(stats_vec);

                break;
            }
        }

        scrolls.push(scroll);
    }

    let goal = {
        let mut goal: Option<Stats> = None;
        'goal_stats: while goal.is_none() {
            input_buf.clear();

            print!("Goal: ");
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
                    continue 'goal_stats;
                }
            }

            if stats_vec.len() == stats.len() {
                goal = Some(Stats::from_vec(stats_vec));
            }
        }

        goal.unwrap_or_else(|| unreachable!())
    };

    let mut init_state = ItemState::new_exists(slots, stats);

    solve_p(&mut init_state, &scrolls, &goal);

    if let ItemState::Exists {
        slots: _,
        stats: _,
        child,
    } = init_state
    {
        println!("\n=== Results ===");

        if let Some(child) = child {
            println!("Probability of success: {:.3}%", child.p_goal * 100.0);
            println!("Expected cost (scrolls only): {:.1}", child.exp_cost);
        } else {
            println!("Impossible!");
        }
    }

    Ok(())
}
