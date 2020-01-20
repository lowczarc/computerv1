pub fn abs(x: f64) -> f64 {
    if x < 0. {
        -x
    } else {
        x
    }
}

pub fn sqrt(x: f64) -> f64 {
    let mut guess = 1.;

    for _ in 0..100 {
        guess -= (guess * guess - x) / (2. * guess);
    }

    guess
}

pub fn fmt_number_with_vars(tab: Vec<(f64, String)>) -> String {
    let mut ret = String::from("0");

    for (value, var) in tab.into_iter() {
        if value != 0. {
            let printed_value = if abs(value) == 1. && var != "" {
                String::new()
            } else {
                abs(value).to_string()
            };
            if ret == "0" {
                let sign = if value < 0. { "-" } else { "" };
                ret = format!("{}{}{}", sign, printed_value, var);
            } else {
                let sign = if value < 0. { '-' } else { '+' };
                ret = format!("{} {} {}{}", ret, sign, printed_value, var);
            }
        }
    }

    ret
}
