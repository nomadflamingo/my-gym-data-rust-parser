#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use my_gym_data_rust_parser::*;

    // Helper function to parse a single record from a string and retrieve the first ExerciseRecord
    fn parse_single_record(input: &str) -> ExerciseRecord {
        let records = parse_exercise_log(input).expect("Expected successful parse");
        assert_eq!(records.len(), 1);
        records.into_iter().next().unwrap()
    }

    // Test the date rule
    #[test]
    fn test_parse_date() {
        let record = parse_single_record("05.08.2024 / bench press / (3 x 10-15) / 100-10,90-10");
        assert_eq!(record.date, NaiveDate::from_ymd_opt(2024, 8, 5).unwrap());
    }

    // Test the exercise_name rule
    #[test]
    fn test_parse_exercise_name() {
        let record = parse_single_record("05.08.2024 / bench press / (3 x 10-15) / 100-10,90-10");
        assert_eq!(record.exercise_name, "bench press");
    }

    // Test the target rule, specifically target sets, min reps, and max reps
    #[test]
    fn test_parse_target() {
        let record = parse_single_record("05.08.2024 / bench press / (3 x 10-15) / 100-10,90-10");
        assert_eq!(record.target.sets_count, 3);
        assert_eq!(record.target.min_reps, 10);
        assert_eq!(record.target.max_reps, 15);
    }

    // Test the set_group rule, parsing multiple sets
    #[test]
    fn test_parse_set_group() {
        let record =
            parse_single_record("05.08.2024 / bench press / (3 x 10-15) / 100-10,90-10;80-12");
        assert_eq!(record.sets.len(), 2);

        // Check first set
        assert_eq!(record.sets[0].attempts.len(), 2);
        assert_eq!(record.sets[0].attempts[0].weight, 100);
        assert_eq!(record.sets[0].attempts[0].reps, 10);
        assert_eq!(record.sets[0].attempts[1].weight, 90);
        assert_eq!(record.sets[0].attempts[1].reps, 10);

        // Check second set
        assert_eq!(record.sets[1].attempts.len(), 1);
        assert_eq!(record.sets[1].attempts[0].weight, 80);
        assert_eq!(record.sets[1].attempts[0].reps, 12);
    }

    // Test the attempt rule by parsing weights and reps within a set
    #[test]
    fn test_parse_attempt() {
        let record = parse_single_record("05.08.2024 / bench press / (3 x 10-15) / 120-8,110-8");
        assert_eq!(record.sets.len(), 1);
        assert_eq!(record.sets[0].attempts.len(), 2);

        // Check first attempt
        assert_eq!(record.sets[0].attempts[0].weight, 120);
        assert_eq!(record.sets[0].attempts[0].reps, 8);

        // Check second attempt
        assert_eq!(record.sets[0].attempts[1].weight, 110);
        assert_eq!(record.sets[0].attempts[1].reps, 8);
    }

    // Test parsing multiple records in a file
    #[test]
    fn test_parse_multiple_records() {
        let input = "\
            05.08.2024 / bench press / (3 x 10-15) / 100-10,90-10;80-12\n\
            06.08.2024 / squat / (4 x 8-12) / 140-10,130-10";

        let records = parse_exercise_log(input).expect("Expected successful parse");
        assert_eq!(records.len(), 2);

        // Check first record
        assert_eq!(
            records[0].date,
            NaiveDate::from_ymd_opt(2024, 8, 5).unwrap()
        );
        assert_eq!(records[0].exercise_name, "bench press");
        assert_eq!(records[0].target.sets_count, 3);
        assert_eq!(records[0].target.min_reps, 10);
        assert_eq!(records[0].target.max_reps, 15);

        // Check second record
        assert_eq!(
            records[1].date,
            NaiveDate::from_ymd_opt(2024, 8, 6).unwrap()
        );
        assert_eq!(records[1].exercise_name, "squat");
        assert_eq!(records[1].target.sets_count, 4);
        assert_eq!(records[1].target.min_reps, 8);
        assert_eq!(records[1].target.max_reps, 12);
    }

    // Test the file rule with empty input to ensure it fails gracefully
    #[test]
    fn test_parse_empty_file() {
        let result = parse_exercise_log("");
        assert!(result.is_err());
    }

    // Test invalid date format to check error handling
    #[test]
    fn test_invalid_date_format() {
        let result = parse_exercise_log("2024-08-05 / bench press / (3 x 10-15) / 100-10,90-10");
        assert!(result.is_err());
    }

    // Test invalid target format to ensure it fails correctly
    #[test]
    fn test_invalid_target_format() {
        let result = parse_exercise_log("05.08.2024 / bench press / 3x10-15 / 100-10,90-10");
        assert!(result.is_err());
    }

    // Test invalid set format to check if parsing fails
    #[test]
    fn test_invalid_set_format() {
        let result = parse_exercise_log("05.08.2024 / bench press / (3 x 10-15) / 100-10-5");
        assert!(result.is_err());
    }

    // Test for WHITESPACE handling in between parts
    #[test]
    fn test_whitespace_handling() {
        let record =
            parse_single_record("05.08.2024 / bench press / (3 x 10-15) / 100-10 , 90-10 ; 80-12");
        assert_eq!(record.sets.len(), 2);

        // Check the first set for whitespace
        assert_eq!(record.sets[0].attempts[0].weight, 100);
        assert_eq!(record.sets[0].attempts[0].reps, 10);
        assert_eq!(record.sets[0].attempts[1].weight, 90);
        assert_eq!(record.sets[0].attempts[1].reps, 10);

        // Check the second set
        assert_eq!(record.sets[1].attempts[0].weight, 80);
        assert_eq!(record.sets[1].attempts[0].reps, 12);
    }
}
