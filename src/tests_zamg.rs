#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_spatialmos_date() {
        assert_eq!(
            "2021-03-28 23:00:00",
            date_to_spatialmos_date("2021-03-28 23:16:23".to_string())
        );
        assert_eq!(
            "2021-01-01 02:00:00",
            date_to_spatialmos_date("2021-01-01 02:03:01".to_string())
        );
    }
}
