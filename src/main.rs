#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(deprecated)]

mod json;
mod opts;

use clap::Parser;
use scroll_strategist::{
    dfs::solve_p, graph::ItemState, scroll::Scroll, stats::Stats,
};
use std::{
    io::{self, Write},
    process,
};

fn main() -> io::Result<()> {
    let cl_opts: opts::Opts = opts::Opts::parse();

    let stdin = io::stdin();
    let mut input_buf = String::new();

    let (mut item_state, scrolls, goal) = if let Some(json_path) = cl_opts.json
    {
        json::read_from_file(json_path)?
    } else {
        if cl_opts.noninteractive {
            eprintln!(
                "`--noninteractive` was specified, but `--json` was not!",
            );

            process::exit(1);
        }

        interactive_input(&stdin, &mut input_buf)?
    };

    solve_p(&mut item_state, &scrolls, &goal);

    if cl_opts.noninteractive {
        if let ItemState::Exists {
            slots: _,
            stats: _,
            child,
        } = item_state
        {
            if let Some(child) = child {
                println!(
                    "Probability of success: {:.3}%",
                    child.p_goal * 100.0,
                );
                println!(
                    "Expected cost (scrolls only): {:.1}",
                    child.exp_cost,
                );
                println!(
                    "Next scroll to use: {:.0}%",
                    child.scroll().p_suc * 100.0,
                );
            }
        }

        Ok(())
    } else {
        print!("=== Results ===");
        interactive_scrolling(&item_state, &stdin, &mut input_buf)
    }
}

fn interactive_input<'a>(
    stdin: &io::Stdin,
    input_buf: &mut String,
) -> io::Result<(ItemState<'a>, Vec<Scroll>, Stats)> {
    println!(
        "=== Welcome to scroll_strategist_cli! ===
Just answer some questions about what you’re scrolling, what you’re scrolling
it with, and what you want your scrolling strategy to optimise for, and
scroll_strategist can work its magic~\n",
    );

    let stats = {
        let mut stats: Option<Stats> = None;
        'stats: while stats.is_none() {
            input_buf.clear();

            print!("Stats: ");
            io::stdout().flush()?;
            stdin.read_line(input_buf)?;

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
            stdin.read_line(input_buf)?;

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
            stdin.read_line(input_buf)?;

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
            stdin.read_line(input_buf)?;

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
            stdin.read_line(input_buf)?;

            if let Ok(cost) = input_buf.trim().parse() {
                scroll.cost = cost;

                break;
            }
        }

        'scroll_stats: loop {
            input_buf.clear();

            print!("Scroll {} stats: ", scrolls.len() + 1);
            io::stdout().flush()?;
            stdin.read_line(input_buf)?;

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
            stdin.read_line(input_buf)?;

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

    println!();

    Ok((ItemState::new_exists(slots, stats), scrolls, goal))
}

fn interactive_scrolling(
    item_state: &ItemState,
    stdin: &io::Stdin,
    input_buf: &mut String,
) -> io::Result<()> {
    println!();

    if let ItemState::Exists {
        slots: _,
        stats,
        child,
    } = item_state
    {
        if let Some(child) = child {
            println!("Probability of success: {:.3}%", child.p_goal * 100.0);
            println!("Expected cost (scrolls only): {:.1}", child.exp_cost);
            println!(
                "Next scroll to use: {:.0}%",
                child.scroll().p_suc * 100.0,
            );

            loop {
                input_buf.clear();

                print!("Did the scroll pass? [y/n/b]: ");
                io::stdout().flush()?;
                stdin.read_line(input_buf)?;

                let outcomes = child.outcomes();
                input_buf.make_ascii_lowercase();
                match input_buf.trim() {
                    "yes" | "y" | "true" => {
                        for outcome in outcomes {
                            if let ItemState::Exists {
                                slots: _,
                                stats: outcome_stats,
                                child: _,
                            } = outcome
                            {
                                if outcome_stats != stats {
                                    return interactive_scrolling(
                                        outcome, stdin, input_buf,
                                    );
                                }
                            }
                        }

                        println!("\nFinal stats: {}", stats);

                        break;
                    }
                    "no" | "n" | "false" => {
                        for outcome in outcomes {
                            if let ItemState::Exists {
                                slots: _,
                                stats: outcome_stats,
                                child: _,
                            } = outcome
                            {
                                if outcome_stats == stats {
                                    return interactive_scrolling(
                                        outcome, stdin, input_buf,
                                    );
                                }
                            }
                        }

                        println!("\nFinal stats: {}", stats);

                        break;
                    }
                    "boom" | "b" | "boomed" => {
                        println!("\nR.I.P. :(");

                        return Ok(());
                    }
                    _ => (),
                }
            }
        } else {
            println!("Final stats: {}", stats);
        }
    } else {
        println!("R.I.P. :(");
    }

    Ok(())
}
