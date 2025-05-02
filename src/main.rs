use chrono::{Duration as ChronoDuration, Local};
use clap::Parser;
use colored::*;
use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

#[derive(Parser)]
#[command(name = "pomodoro-cli")]
#[command(
    author = "Tomoyuki Kamimura",
    version = "1.0",
    about = "A simple Pomodoro timer"
)]
struct Args {
    /// 作業時間（分）
    #[arg(short, long = "work")]
    work_time: u64,

    /// 休憩時間（分）
    #[arg(short, long = "break")]
    break_time: u64,

    /// セット回数
    #[arg(short, long, default_value_t = 1)]
    repeat: u32,
}

fn countdown(minutes: u64, label: &str, color: colored::Color) {
    let duration = Duration::from_secs(minutes * 60);
    let start = Instant::now();
    let end = start + duration;

    while Instant::now() < end {
        let remaining = end - Instant::now();
        let mins = remaining.as_secs() / 60;
        let secs = remaining.as_secs() % 60;
        let time_str = format!("{:02}:{:02}", mins, secs).color(color).bold();
        print!("\r    {}: {}", label, time_str);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n    {}", format!("{} 完了！", label).color(color).bold());
}

fn print_separator(set_index: u32, total_sets: u32) {
    println!();
    println!("{}", "=".repeat(40).dimmed());
    println!(
        "    {}",
        format!("✅\u{3000}セット {}/{} 完了", set_index, total_sets)
            .green()
            .bold()
    );
    println!("{}", "=".repeat(40).dimmed());
    println!();
}

fn main() {
    let args = Args::parse();

    let total_minutes = args.repeat as i64 * (args.work_time + args.break_time) as i64;
    let overall_start = Local::now();
    let overall_end = overall_start + ChronoDuration::minutes(total_minutes);

    println!(
        "{}",
        format!(
            "🎯\u{3000}ポモドーロ開始：「作業 {}分 → 休憩 {}分」 × {}セット",
            args.work_time, args.break_time, args.repeat
        )
        .blue()
        .bold()
    );

    println!(
        "{}",
        format!(
            "🕒\u{3000}全体：{} ～ {}",
            overall_start.format("%H:%M"),
            overall_end.format("%H:%M")
        )
        .cyan()
        .bold()
    );
    println!();

    let mut current_time = overall_start;

    for i in 1..=args.repeat {
        let set_start = current_time;
        let set_end =
            set_start + ChronoDuration::minutes(args.work_time as i64 + args.break_time as i64);

        println!(
            "{}",
            format!(
                "▶️\u{3000}セット {} 開始（{} ～ {}）",
                i,
                set_start.format("%H:%M"),
                set_end.format("%H:%M")
            )
            .cyan()
            .bold()
        );

        countdown(args.work_time, "🛠\u{3000}作業中", colored::Color::Green);
        countdown(args.break_time, "☕\u{3000}休憩中", colored::Color::Yellow);
        print_separator(i, args.repeat);

        current_time = set_end;
    }

    println!(
        "{}",
        "🎉\u{3000}全セット終了！お疲れさまでした！"
            .magenta()
            .bold()
    );
}
