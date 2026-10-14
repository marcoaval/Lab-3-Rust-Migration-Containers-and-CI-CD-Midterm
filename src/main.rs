// PerfReport reads system resource usage and computes an efficiency score.

use sysinfo::{Disks, System};

// Convert byte measurements to binary gigabytes for the displayed report.
const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// Memory and disk thresholds reproduce the original C++ scoring rules.
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");

    // Refresh system measurements before reading resource usage.
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Logical processors : {}", sys.cpus().len());

    let total = sys.total_memory();
    let used = sys.used_memory();
    // An unavailable total yields zero load instead of division by zero.
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB)  : {:.2}", total as f64 / GB);
    println!("Memory free  (GB)  : {:.2}", (total - used) as f64 / GB);
    println!("Memory in use (%)  : {}", mem_load);

    println!("Uptime (minutes)   : {}", System::uptime() / 60);

    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct = 0.0;
    // The first available disk supplies the report's disk measurements.
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        if dtotal > 0 {
            disk_used_pct = 100.0 * (dtotal - dfree) as f64 / dtotal as f64;
        }
        println!("Disk total (GB)    : {:.2}", dtotal as f64 / GB);
        println!("Disk free  (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used  (%)     : {:.2}", disk_used_pct);
    }

    // Combine memory and disk load into the efficiency score.
    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score   : {} / 100 ({})", score, rating(score));
    println!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_thresholds_are_strict() {
        for (mem_load, expected) in [(0, 100), (70, 100), (71, 90), (85, 90), (86, 75)] {
            assert_eq!(
                compute_score(&Findings {
                    mem_load,
                    disk_used_pct: 0.0
                }),
                expected
            );
        }
    }

    #[test]
    fn disk_thresholds_are_strict() {
        for (disk_used_pct, expected) in [
            (0.0, 100),
            (75.0, 100),
            (75.01, 90),
            (90.0, 90),
            (90.01, 75),
        ] {
            assert_eq!(
                compute_score(&Findings {
                    mem_load: 0,
                    disk_used_pct
                }),
                expected
            );
        }
    }

    #[test]
    fn memory_and_disk_penalties_combine() {
        for (mem_load, memory_penalty) in [(70, 0), (85, 10), (86, 25)] {
            for (disk_used_pct, disk_penalty) in [(75.0, 0), (90.0, 10), (91.0, 25)] {
                let findings = Findings {
                    mem_load,
                    disk_used_pct,
                };
                assert_eq!(
                    compute_score(&findings),
                    100 - memory_penalty - disk_penalty
                );
            }
        }
    }

    #[test]
    fn ratings_match_score_boundaries() {
        for (score, expected) in [
            (100, "GOOD"),
            (85, "GOOD"),
            (84, "FAIR"),
            (60, "FAIR"),
            (59, "POOR"),
            (0, "POOR"),
        ] {
            assert_eq!(rating(score), expected);
        }
    }
}
