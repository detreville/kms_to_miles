#![warn(clippy::all)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![warn(clippy::restriction)]
#![allow(
    clippy::arithmetic_side_effects,
    clippy::default_numeric_fallback,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::missing_assert_message,
    clippy::missing_docs_in_private_items,
    clippy::mod_module_files,
    clippy::non_ascii_literal,
    clippy::print_stdout,
    clippy::pub_use,
    clippy::question_mark_used,
    clippy::std_instead_of_core,
    clippy::unwrap_used
)]

use min_max::max;
use num::{Integer, One, ToPrimitive};
use std::ops::RangeInclusive;

mod distance;
use distance::{Answer, Approximation, Int, Real, Rounding, UnitIn, UnitOut};
use Rounding::{Truncated, WillRound};
use UnitIn::{Kms, TenthKms};

// Intervals are treated as half-open here. This is safe since we care only about their sizes.

// Iteration phases over inputs, with additional info.
// TODO: Use RangeBounds once it works.
struct Phase(
    RangeInclusive<Int>, // Closed range on input counts...
    UnitIn,              // ... against this UnitIn.
    fn(&Real) -> String, // Input formatter.
    Vec<UnitOut>,        // Possible output units for each input.
);

fn main() {
    for Phase(input_counts, unit_in, format_input, units_out) in [
        Phase(1..=9, TenthKms(), format_meters, all_units_out()),
        Phase(10..=29, TenthKms(), format_tenth_kms, no_less_than((1, 8))),
        Phase(30..=99, TenthKms(), format_tenth_kms, no_less_than((1, 4))),
        Phase(10..=20, Kms(), format_kms, no_less_than((1, 2))),
    ] {
        for input in input_counts.map(|n: i64| Approximation::new(n, &unit_in.unit(), Truncated)) {
            let kms = input.distance(); // Input distance in km.
            let Some(best) = units_out
                .iter()
                .flat_map(|unit_out| {
                    unit_out.unit().count_range(&input.interval(), WillRound).map(|ref j| {
                        let output = Approximation::new(*j, &unit_out.unit(), WillRound);
                        output.unit().count().denom().gcd(j).is_one()
                        .then_some(Answer::new(&input, &output))
                    })
                })
                .fold(None, better_of) else {
                    println!("{} => NO ANSWER", format_input(&kms));
                    continue;
                };
            println!(
                "{} => {}",
                format_input(&kms),
                format_miles(&best.output.many())
            );
        }
    }
}

fn format_meters(kms: &Real) -> String {
    format!("{}m", kms.to_owned() * Real::from(1000))
}

fn format_tenth_kms(kms: &Real) -> String {
    if let Some(kms_f32) = kms.to_f32() {
        format!("{:.1}km", kms_f32)
    } else {
        format!("{}km", kms)
    }
}

fn format_kms(kms: &Real) -> String {
    format!("{kms}km")
}

fn format_miles(miles: &Real) -> String {
    format!(
        "{} {}",
        miles,
        if miles.numer().is_one() {
            "mile"
        } else {
            "miles"
        },
    )
}

fn all_units_out() -> Vec<UnitOut> {
    enum_iterator::all()
        .filter(|unit_out: &UnitOut| unit_out.unit().count().numer().is_one())
        .collect()
}

fn no_less_than(fraction: (Int, Int)) -> Vec<UnitOut> {
    let fraction_ = Real::from(fraction.0) / Real::from(fraction.1);
    all_units_out()
        .iter()
        .filter(|out| out.unit().count() >= &fraction_)
        .copied()
        .collect::<Vec<_>>()
}

// The better of two (optional) answers, based on the overlap fractions of each input and output.
// Each interval most be non-empty.
fn better_of(x: Option<Answer>, y: Option<Answer>) -> Option<Answer> {
    match (x.to_owned(), y.to_owned()) {
        (None, _) => y,
        (_, None) => x,
        (Some(x_), Some(y_)) => {
            if overlap_fraction(&x_) > overlap_fraction(&y_) {
                x
            } else {
                y
            }
        }
    }
}

// The fractional overlap of an answer's input and output.
fn overlap_fraction(answer: &Answer) -> Real {
    let input_interval = answer.input.interval();
    let output_interval = answer.output.interval();
    // Divide by largest to get smallest fraction.
    input_interval.intersection_with(&output_interval).size()
        / max(input_interval.size(), output_interval.size())
}
