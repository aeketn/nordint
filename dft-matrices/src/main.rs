// author:  Erik Nordin
// created: 08/02/2018
// updated: 08/04/2018
// contact: aeketn@gmail.com

#![feature(range_contains)] // Allows syntax: (0..10).contains(x);
                            // Must compile using a Nightly build

/// Prints the header for a table of size n
fn print_header(op_name: &str, n: i64) {
    println!("{{{}}} table for n = {}", op_name, n);
    print!("{:5}|", ' '); 
    for col in 0..n {
        print!("{:10}", col);
    }
    println!();
    print!("-----|");
    for _ in 0..n * 10 {
        print!("-");
    }
    println!();
}

/// Prints the dft table given:
///   a table name,
///   a format function (clockwise or counter-clockwise)
///   a bound (n) for the table size
fn print_dft_table<F>(table_name: &str, format: &F, n: i64)
where
    F: Fn(i64, i64) -> String,
{
    print_header(table_name, n);
    for row in 0..n {
        print!("{:5}|", row);
        for col in 0..n {
            let omega = row * col % n;
            print!("{:>10}", format(omega, n));
        }
        println!();
        println!("     |");
    }
    println!("\n");
}

/// Formats omega in the form of (i, -1, or -i) multiplied by omega 
/// raised to the power of some exponent. This is uesd only if n
/// is evenly divisible into quarters or halves such that 
/// (i, -1, or -i) are relevant values to refer to in the table.
fn format_omega(multiplier: &str, exponent: i64) -> String {
    if exponent == 1 {
        format!("{}ω", multiplier)
    } else {
        format!("{}ω^{}", multiplier, exponent)
    }
}

/// Formats the ith root of unity such that omega has a positive exponent,
/// meaning that the angles expand counter-clockwise around the unit circle.
fn format_dft_counterclockwise(omega: i64, n: i64) -> String {
    let has_halves = 0 == n % 2;
    let has_quarters = 0 == n % 4;

    let i = n / 4;
    let neg_one = n / 2;
    let neg_i = 3 * n / 4;

    let q2 = i..neg_one;
    let q3 = neg_one..neg_i;
    let q4 = neg_i..n;

    if omega == 0 { " 1".to_string() }
    else if has_quarters && omega == i          { " i".to_string() }
    else if has_halves   && omega == neg_one    { "-1".to_string() } 
    else if has_quarters && omega == neg_i      { "-i".to_string() }
    else if has_quarters && q2.contains(&omega) { format_omega(" i", omega - i)       }
    else if has_halves   && q3.contains(&omega) { format_omega(" -", omega - neg_one) }
    else if has_quarters && q4.contains(&omega) { format_omega("-i", omega - neg_i)   }
    else { format_omega("", omega) }
}

/// Formats the ith root of unity such that omega has a negative exponent,
/// meaning that the angles expand clockwise around the unit circle.
fn format_dft_clockwise(omega: i64, n: i64) -> String {
    let has_halves = 0 == n % 2;
    let has_quarters = 0 == n % 4;

    let neg_i = n / 4;
    let neg_one = n / 2;
    let i = 3 * n / 4;

    let q3 = neg_i..neg_one;
    let q2 = neg_one..i;
    let q1 = i..n;

    if omega == 0 { " 1".to_string() } 
    else if has_quarters && omega == neg_i      { "-i".to_string() } 
    else if has_halves   && omega == neg_one    { "-1".to_string() }
    else if has_quarters && omega == i          { " i".to_string() }
    else if has_quarters && q3.contains(&omega) { format_omega("-i", omega - neg_i)   }
    else if has_halves   && q2.contains(&omega) { format_omega(" -", omega - neg_one) } 
    else if has_quarters && q1.contains(&omega) { format_omega(" i", omega - i)       }
    else { format_omega("", omega) }
}

fn main() {
    for n in 1..14 {
        print_dft_table("DFT:clockwise", &format_dft_clockwise, n);
    }
    for n in 1..14 {
        print_dft_table("IDFT:counter-clockwise", &format_dft_counterclockwise, n);
    }
    let powers_of_two_plus_one: [i64;4] = [2+1, 4+1, 8+1, 16+1];
    for n in &powers_of_two_plus_one {
        print_dft_table("DFT:clockwise", &format_dft_clockwise, *n);
    }
}