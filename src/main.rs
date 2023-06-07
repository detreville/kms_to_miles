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
use num::{One, ToPrimitive};
use std::ops::RangeInclusive;

mod distance;
use distance::{Answer, Approximation, InputUnit, Int, OutputUnit, Real, Rounding};
use InputUnit::{Kms, TenthKms};
use Rounding::{Truncated, WillRound};

// Intervals are treated as half-open here. This is safe since we care only about their sizes.

fn main() {
    let all_output_units = all_output_units(); // Numerators must all equal Real::One().
    let halves_and_up = output_symbols_no_less_than((1, 2));
    let quarters_and_up = output_symbols_no_less_than((1, 4));
    let eighths_and_up = output_symbols_no_less_than((1, 8));
    struct Phase(
        // Iteration phase over inputs.
        RangeInclusive<Int>, // Closed range on input counts...
        InputUnit,           // ... against this InputUnit.
        fn(&Real) -> String, // Input formatter.
        Vec<OutputUnit>,     // Possible output units for each input.::
    );
    let phases = [
        Phase(1..=9, TenthKms(), format_as_meters, all_output_units),
        Phase(10..=29, TenthKms(), format_tenth_kms, eighths_and_up),
        Phase(30..=99, TenthKms(), format_tenth_kms, quarters_and_up),
        Phase(10..=20, Kms(), format_as_kms, halves_and_up),
    ];
    for Phase(input_counts, input_unit, input_formatter, output_units) in phases {
        for input in input_counts.map(|i| Approximation::new(i, &input_unit.unit(), Truncated)) {
            let kms = input.many() * input.unit().of().to_owned();
            let Some(best) = output_units
                .iter()
                .flat_map(|out| {
                    out.unit().count_range(&input.interval(), WillRound).map(|ref j| {
                        let output = Approximation::new(
                            *j,
                            &out.unit(),
                            WillRound,
                        );
                        let multiplier = output.unit().many().to_owned();
                        (multiplier.denom() == (multiplier * output.count().to_owned()).denom()
                            )
                        .then_some(Answer::new(&input, &output))
                    })
                })
                .fold(None, better_of) else {
                    println!("{} => NO ANSWER", input_formatter(&kms));
                    continue;
                };
            let miles = best.output.many();
            println!(
                "{} => {} {}",
                input_formatter(&kms),
                miles,
                if miles.numer().is_one() {
                    "mile"
                } else {
                    "miles"
                },
            );
        }
    }
}

fn format_as_meters(kms: &Real) -> String {
    format!("{}m", kms.to_owned() * Real::from(1000))
}

fn format_tenth_kms(kms: &Real) -> String {
    if let Some(kms_f32) = kms.to_f32() {
        format!("{:.1}km", kms_f32)
    } else {
        format!("{}km", kms)
    }
}

fn format_as_kms(kms: &Real) -> String {
    format!("{kms}km")
}

fn all_output_units() -> Vec<OutputUnit> {
    enum_iterator::all().collect()
}

fn output_symbols_no_less_than(fraction: (Int, Int)) -> Vec<OutputUnit> {
    let fraction_ = Real::from(fraction.0) / Real::from(fraction.1);
    all_output_units()
        .iter()
        .filter(|out| out.unit().many() >= &fraction_)
        .copied()
        .collect::<Vec<_>>()
}

// The better of two (optional) answers, based on the overlap fractions of each input and output.
// Each interval most be non-empty.
fn better_of(x: Option<Answer>, y: Option<Answer>) -> Option<Answer> {
    // The fractional overlap of one answer's input and output.
    fn overlap_fraction(answer: &Answer) -> Real {
        ((answer.input.interval()).intersection_with(&answer.output.interval())).size()
            / max(
                answer.input.interval().size(),
                answer.output.interval().size(),
            )
    }
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
