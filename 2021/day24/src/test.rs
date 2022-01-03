#[cfg(test)]
mod test {
    #[test]
    fn sample_a() {
        let prog = crate::parse(&include_str!("../sample1.txt").to_string());
        let input = vec![4];
        assert_eq!(crate::ALU::run_program(&prog, input), [0,-4,0,0]);
    }

    #[test]
    fn sample_b() {
        let prog = crate::parse(&include_str!("../sample2.txt").to_string());
        let input = vec![2, 6];
        assert!(crate::ALU::run_program(&prog, input)[3] == 1);
    }

    #[test]
    fn sample_c() {
        let prog = crate::parse(&include_str!("../sample3.txt").to_string());
        let input = vec![15];
        assert_eq!(crate::ALU::run_program(&prog, input), [1,1,1,1]);
    }

    #[test]
    fn monad_a() {
        let prog = crate::parse(&include_str!("../monad1.txt").to_string());
        let input = vec![9];
        assert_eq!(crate::ALU::run_program(&prog, input), [9,0,0,0]);
    }

    // test individual MONAD program blocks (18 lines with one inp instruction)
    #[test]
    fn monad_b() {
        let prog = crate::parse(&include_str!("../monad2.txt").to_string());
        
        for val in 1 ..= 9 {
            let input = vec![val];
            assert_eq!(crate::ALU::run_program(&prog, input), [val, 1, val + 16, val + 16]);
        }
    }
}
