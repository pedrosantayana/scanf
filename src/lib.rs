pub mod scanf;

#[cfg(test)]
mod tests {
    use crate::scanf::scanf;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        scanf("aa");
        assert_eq!(result, 4);
    }

    #[test]
    fn read_int() {
        let int = scanf("%{d}");
        assert
    }
}
