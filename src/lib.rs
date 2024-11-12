use chrono::NaiveDate;
use pest::Parser;
use pest_derive::Parser;
use std::io;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum GymDataParserError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),

    #[error("Date parsing error: {0}")]
    DateParseError(#[from] chrono::ParseError),

    #[error("File content parse error")]
    FileContentParseError,

    #[error("Exercise name parse error")]
    ExerciseNameParseError,

    #[error("Target parse error")]
    TargetParseError,

    #[error("Set group parse error")]
    SetGroupParseError,

    #[error("Missing data error")]
    MissingDateError,

    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(#[from] std::num::ParseIntError),
}



/// Represents a record of a single exercise session, including the date, exercise name,
/// target repetition range, and the sets completed.
#[derive(Debug)]
pub struct ExerciseRecord {
    /// The date the exercise was completed, formatted as `DD.MM.YYYY`.
    pub date: NaiveDate,

    /// The name of the exercise performed (e.g., "bench press").
    pub exercise_name: String,

    /// The target repetitions, including the total sets and the min-max rep range.
    pub target: TargetReps,

    /// A vector of `Set` instances, each containing the details of completed sets.
    pub sets: Vec<Set>,
}

/// Represents the target repetition range for an exercise, specifying the number of sets
/// and the min-max range of repetitions for each set.
///
/// An example of a valid target token: "3 x 10-15 reps", where `sets_count` is 3,
/// `min_reps` is 10, and `max_reps` is 15.
#[derive(Debug)]
pub struct TargetReps {
    /// The number of sets targeted for the exercise.
    pub sets_count: u32,

    /// The minimum number of repetitions targeted for each set.
    pub min_reps: u32,

    /// The maximum number of repetitions targeted for each set. 
    /// Can be set to the same value as `min_reps`.
    pub max_reps: u32,
}

/// Represents a single set performed within an exercise session, which may contain
/// multiple attempts if the weight or reps were modified mid-set.
#[derive(Debug)]
pub struct Set {
    /// A vector of `Attempt` instances, representing individual attempts within the set.
    ///
    /// Multiple attempts can occur if adjustments are made within the set due to difficulty.
    pub attempts: Vec<Attempt>,
}

/// Represents an attempt within a set, specifying the weight used and the repetitions completed.
/// 
/// Part of a `Set` instance.
#[derive(Debug)]
pub struct Attempt {
    /// The weight lifted in this attempt, in kilograms.
    pub weight: u32,

    /// The number of repetitions completed in this attempt.
    pub reps: u32,
}


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

pub fn parse_exercise_log(input: &str) -> Result<Vec<ExerciseRecord>, GymDataParserError> {
    let mut records = Vec::new();

    let mut parsed = Grammar::parse(Rule::file, input)
        .map_err(|e| GymDataParserError::ParseError(Box::from(e)))?;

    let file_rule = parsed
        .next()
        .ok_or(GymDataParserError::FileContentParseError)?;

    for record in file_rule.into_inner() {
        if record.as_rule() == Rule::record {
            let mut date: Option<NaiveDate> = None;
            let mut exercise_name: Option<String> = None;
            let mut target: Option<TargetReps> = None;
            let mut sets: Vec<Set> = Vec::new();

            for field in record.into_inner() {
                match field.as_rule() {
                    Rule::date => {
                        let date_str = field.as_str();
                        let parsed_date = NaiveDate::parse_from_str(date_str, "%d.%m.%Y")?;
                        date = Some(parsed_date);
                    }
                    Rule::exercise_name => {
                        let name = field.as_str().trim().to_string();
                        exercise_name = Some(name);
                    }
                    Rule::target => {
                        let mut parts = field.into_inner();
                        let sets_count_str = parts
                            .next()
                            .ok_or(GymDataParserError::TargetParseError)?
                            .as_str()
                            .trim();
                        let min_reps_str = parts
                            .next()
                            .ok_or(GymDataParserError::TargetParseError)?
                            .as_str()
                            .trim();
                        let max_reps_str = parts
                            .next()
                            .ok_or(GymDataParserError::TargetParseError)?
                            .as_str()
                            .trim();

                        let sets_count = sets_count_str.parse::<u32>()?;
                        let min_reps = min_reps_str.parse::<u32>()?;
                        let max_reps = max_reps_str.parse::<u32>()?;

                        target = Some(TargetReps {
                            sets_count,
                            min_reps,
                            max_reps,
                        });
                    }
                    Rule::set_group => {
                        for set_group in field.into_inner() {
                            let mut attempts = Vec::new();
                            for set in set_group.into_inner() {
                                let mut set_parts = set.into_inner();
                                let weight_str = set_parts
                                    .next()
                                    .ok_or(GymDataParserError::SetGroupParseError)?
                                    .as_str()
                                    .trim();
                                let reps_str = set_parts
                                    .next()
                                    .ok_or(GymDataParserError::SetGroupParseError)?
                                    .as_str()
                                    .trim();

                                let weight = weight_str.parse::<u32>()?;
                                let reps = reps_str.parse::<u32>()?;

                                attempts.push(Attempt { weight, reps });
                            }
                            sets.push(Set { attempts });
                        }
                    }
                    _ => {}
                }
            }

            let date = date.ok_or(GymDataParserError::MissingDateError)?;
            let exercise_name = exercise_name.ok_or(GymDataParserError::ExerciseNameParseError)?;
            let target = target.ok_or(GymDataParserError::TargetParseError)?;

            records.push(ExerciseRecord {
                date,
                exercise_name,
                target,
                sets,
            });
        }
    }

    Ok(records)
}
