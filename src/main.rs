use std::env;
mod polynomes;
mod utils;
use polynomes::{Error, Polynome};
use utils::{fmt_number_with_vars, sqrt};

fn print_usage() {
    println!(
        "Compute 2nd degree polynomial equations
Usage:\n\tcomputerv1 [equation]\n
Example:\n\tcomputerv1 \"5 * X^2 + 2 = 6 * X^2 + 1 * X^1\""
    );
}

fn main() {
    let input = if let Some(input) = env::args().nth(1) {
        input
    } else {
        eprintln!("ERROR: Missing parameter\n");
        print_usage();
        return;
    };

    let polynomes: Vec<Result<Polynome, Error>> =
        input.split("=").map(|elem| Polynome::parse(elem)).collect();

    if polynomes.len() != 2 {
        eprintln!("ERROR: Invalid input");
        return;
    }

    if let (Ok(poly1), Ok(poly2)) = (polynomes[0].clone(), polynomes[1].clone()) {
        let simplified = poly1 - poly2;
        println!("Reduced form: {} = 0", simplified);
        println!("Polynomial degree: {}", simplified.get_degree());
        match simplified.get_degree() {
            0 => {
                if simplified.get_multiplier_degree(0) == 0. {
                    println!("Every number is a solution");
                } else {
                    println!("Mmm... Seems kinda false to me...");
                }
            }
            1 => println!(
                "{} is a solution",
                -simplified.get_multiplier_degree(0) / simplified.get_multiplier_degree(1)
            ),
            2 => {
                let a = simplified.get_multiplier_degree(2);
                let b = simplified.get_multiplier_degree(1);
                let c = simplified.get_multiplier_degree(0);

                let discriminant = b * b - (4. * a * c);
                println!("Discriminant: {}", discriminant);
                if discriminant > 0. {
                    println!("Positive discriminant, there are 2 real solutions:");
                    println!("{}", (-b + sqrt(discriminant)) / (2. * a));
                    println!("{}", (-b - sqrt(discriminant)) / (2. * a));
                } else if discriminant < 0. {
                    println!("Negative discriminant, there are 2 complexe solutions:");
                    let solutions = [
                        vec![
                            (-b / (2. * a), "".into()),
                            (sqrt(-discriminant) / (2. * a), "i".into()),
                        ],
                        vec![
                            (-b / (2. * a), "".into()),
                            (-sqrt(-discriminant) / (2. * a), "i".into()),
                        ],
                    ];
                    for solution in solutions.into_iter() {
                        println!("{}", fmt_number_with_vars(solution.to_vec()));
                    }
                } else {
                    println!("Discriminant is 0, there is 1 real solution:");
                    println!("{}", -b / (2. * a));
                }
            }
            _ => println!("The polynomial degree is strictly greater than 2, I can't solve."),
        };
    } else {
        let error = polynomes[0]
            .as_ref()
            .and(polynomes[1].as_ref())
            .unwrap_err();
        eprintln!("ERROR: {}", error);
    };
}
