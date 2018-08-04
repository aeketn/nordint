use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;

/// Applies a binary operator to two arguments.
/// The LHS argument must have addition and multiplication defined
/// such that the right-hand side is of type RHS, and the result is of type ANS.
/// The ANS (return type) must have Rem (modulus) defined.
fn apply_operator<OP, LHS, RHS, ANS>(operator: &OP, lhs: LHS, rhs: RHS) -> ANS
where
    OP:  Fn(LHS, RHS) -> ANS,
    LHS: Add<RHS>
       + Mul<RHS>,
    ANS: Rem,
{
    operator(lhs, rhs)
}

/// Prints the header for a table of size n
fn print_header(op_name: &str, n: u64) {
    println!("{{{}}} table for n = {}", op_name, n);
    print!("{:5}|", ' ');
    for col in 0..n {
        print!("{:5}", col); 
    }
    println!();
    print!("-----|");
    for _ in 0..n * 5 {
        print!("-");
    }
    println!();
}

/// Prints the table of size n, given a function to apply to its row/col intersections
fn print_table<F>(op_name: &str, f: &F, n: u64)
where
    F: Fn(u64, u64) -> u64,
{
    print_header(op_name, n);
    for row in 0..n {
        print!("{:5}|", row);
        for col in 0..n {
            print!("{:5}", apply_operator(f, row, col) % n);
        }
        println!();
        println!("{:5}|", "");
    }
    println!("\n");
}

fn main() {
    for i in 1..10 {
        print_table("Add", &<u64>::add, i);
    }
    for i in 1..10 {
        print_table("Mul", &<u64>::mul, i);
    }
}