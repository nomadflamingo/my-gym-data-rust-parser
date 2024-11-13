# my-gym-data-parser
## Brief Overview
This is an implementation for a simple parser designed to process weight training gym workout logs, manually recorded for each exercise session. It is written using [Pest](https://pest.rs/) library. The parser takes a `txt` file as an input, where each line represents an exercise performed, and produces a structured record for each line. Each record includes details such as the date, exercise type, target number of sets and repetitions, and the real number of sets and repetitions performed, divided into separate attempts (including any variations in weight or repetitions within sets).

## Links
* [Crate](https://crates.io/crates/my-gym-data-rust-parser)
* [Documentation](https://docs.rs/my-gym-data-rust-parser/latest/my_gym_data_rust_parser/index.html)

## Input data format

Each line in the log file represents a group of sets performed in one go for a single exercise and is divided into four sections, separated by the `/` symbol:

1. **Date**: The date the exercise was performed, in `DD.MM.YYYY` format.
2. **Exercise Name**: The name of the exercise (e.g., "bench press" or "lat pulldown").
3. **Target Reps**: The target for the exercise set, set initially when starting to complete an exercise, represented as the number of sets and a min-max range for reps (e.g., `3 x 10-15`).
4. **Sets Completed**: A list of attempts for each set, where each attempt specifies the weight and reps performed (e.g., `35-15`, `25-15`). The weight is specified in kilograms.
    * Each set is divided into multiple **Attempts**. Normally, a set would contain only one attempt, but multiple attempts are present if the weight were modified mid-set because it was too heavy/too light.

## Output data format

The program will parse every line in the file into a `Record` Data Structure contains information as follows:

- `ExerciseRecord`: Contains the date, exercise name, target reps, and a list of sets.
- `TargetReps`: Describes the number of sets and min-max range of reps per set.
- `Set`: Represents one set, which may contain multiple attempts (e.g., if the weight or reps changed during the set).
- `Attempt`: Specifies the weight and reps for a single attempt within a set.


## Example

Given an input file (`input.txt`) with the following line:

```text
05.08.2024 / reverse grip lat pulldown / (3 x 10-15) / 35-15,30-15;25-15;25-15
```

The parser generates an `ExerciseRecord` with the following components:

- **Date**: `"05.08.2024"`  
  Represents the date on which the exercise was performed, allowing the parser to associate each session with a specific day.

- **Exercise Name**: `"reverse grip lat pulldown"`  
  Identifies the name of the exercise performed, enabling tracking of specific exercises over time.

- **Target Reps**:  
  Indicates the initial goal for this exercise in terms of sets and repetitions:
  - **Sets**: `3`  
    The planned number of sets for the exercise.
  - **Rep Range**: `10-15`  
    The intended range of repetitions for each set, indicating a target range rather than an exact number.

- **Sets Completed**:  
  Contains details about each set performed, which may include one or multiple attempts (if weights or reps were adjusted within the set). In this example, the parser interprets the sets as follows:

  - **Set 1**:  
    - **Attempt 1**: `35kg` for `15` reps  
      The initial attempt for this set, beginning with a heavier weight.
    - **Attempt 2**: `30kg` for `15` reps  
      Due to a weight adjustment, the second attempt for this set is completed with a lighter weight.
  
  - **Set 2**:  
    - A single attempt with `25kg` for `15` reps, meeting the target without any weight changes.
  
  - **Set 3**:  
    - Another single attempt with `25kg` for `15` reps, again completed without adjustments.
    

## Grammar

The full **Pest** grammar for the parser is defined as follows:

```rust
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

## Installation

Either clone this repository:

```sh
git clone <link>
cd <local-repo>
cargo build
```

Or add this crate to your `Cargo.toml`:

```toml
[dependencies]
exercise_log_parser = "0.3.0"
```

You can then launch this parser with the command:

```sh
cargo run -- parse --file input.txt
```

## License

This project is licensed under the MIT License.
