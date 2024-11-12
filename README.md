# my-gym-data-parser

A simple parser for my gym data for each set of exercises written using [Pest](https://pest.rs/) library.

## Parsing Process

Each line in the log file represents a group of sets performed in one go for a single exercise and is divided into four sections, separated by the `/` symbol:

1. **Date**: The date the exercise was performed, in `DD.MM.YYYY` format.
2. **Exercise Name**: The name of the exercise (e.g., "bench press" or "lat pulldown").
3. **Target Reps**: The target for the exercise set, set initially when starting to complete an exercise, represented as the number of sets and a min-max range for reps (e.g., `3 x 10-15`).
4. **Sets Completed**: A list of attempts for each set, where each attempt specifies the weight and reps performed (e.g., `35-15`, `25-15`). Each set is divided into multiple **Attempts**. Normally, a set would contain only one attempt, but multiple attempts are present if the weight were modified mid-set because it was too heavy/too light.

The parsed Data Structure looks as follows:

- `ExerciseRecord`: Contains the date, exercise name, target reps, and a list of sets.
- `TargetReps`: Describes the number of sets and min-max range of reps per set.
- `Set`: Represents one set, which may contain multiple attempts (e.g., if the weight or reps changed during the set).
- `Attempt`: Specifies the weight and reps for a single attempt within a set.

## Grammar

The full **Pest** grammar for the parser is defined as follows:

```pest
file = { SOI ~ (record ~ ("\r\n" | "\n")?)+ ~ EOI }
record = { date ~ "/" ~ exercise_name ~ "/" ~ target ~ "/" ~ set_group }

date  = { day ~ "." ~ month ~ "." ~ year }
day = { ASCII_DIGIT{2} }
month = { ASCII_DIGIT{2} }
year = { ASCII_DIGIT{4} }

exercise_name = { (!"/" ~ ANY)+ }

target = { "(" ~ target_sets ~ "x" ~ target_min_reps ~ "-" ~ target_max_reps ~ ")" }
target_sets = { ASCII_DIGIT+ }
target_min_reps = { ASCII_DIGIT+ }
target_max_reps = { ASCII_DIGIT+ }

set_group = { set ~ (";" ~ set)* }
set = { attempt ~ ("," ~ attempt)* }

attempt = { weight ~ "-" ~ reps }
weight = { ASCII_DIGIT+ }
reps = { ASCII_DIGIT+ }

WHITESPACE = _{ " " }
```

## Example Usage

Given an input file (`input.txt`) with the following line:

05.08.2024 / reverse grip lat pulldown / (3 x 10-15) / 35-15 25-15 25-15

This example will be parsed into a `ExerciseRecord` where:
- The date is `05.08.2024`
- The exercise name is `"reverse grip lat pulldown"`
- The target reps are `3 sets of 10-15 reps`
- The sets contain multiple attempts, such as `35kg for 15 reps` and `25kg for 15 reps`.

### Installation

Either clone this repository:

```sh
git clone <link>
cd <local-repo>
cargo build
```

Or add this crate to your `Cargo.toml`:

[dependencies]
exercise_log_parser = "0.1.0"

### Usage

Given `input.txt` file, you can launch this file with the command:

```sh
cargo run -- parse --file input.txt
```

### Purpose

This crate is ideal for fitness applications or data processing tools that need to parse and analyze structured exercise logs. By converting logs into structured data, developers can create applications that track workout progress, calculate workout volume, or analyze performance trends over time.

### License

This project is licensed under the MIT License.
