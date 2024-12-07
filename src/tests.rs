fn test_addition() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_addition();
    }
}