
pub fn text_to_string_vec(input: &str) -> Vec<&str>{
    return input.lines().collect();
}

#[cfg(test)]
mod tests{
    use crate::text_to_string_vec;

    #[test]
    fn testtext_to_string_vec() {
        let input ="abc\ndef\nghi";

        let result = text_to_string_vec(input);

        assert_eq!(result, vec!["abc", "def", "ghi"]);

    }

}