fn main() {}

#[cfg(test)]
mod test {
    use boolish_macro::Boolish;

    #[test]
    fn should_generate_conversion_to_bool() {
        #[derive(Boolish)]
        enum State {
            #[boolval("true")]
            Active,
            #[boolval("false")]
            Inactive,
        }

        let active: State = From::from(true);
        assert_eq!(Into::<bool>::into(active), true);
        let inactive: State = From::from(false);
        assert_eq!(Into::<bool>::into(inactive), false);
    }
}
