pub fn fivonacci(n: usize) -> usize{

        if n == 0{
            return 0;
        }else if n == 1{
            return 1;
        }

        let mut result = vec![0; n + 1];
        result[0] = 0;
        result[1] = 1;
        for i in 2..=n{
            result[i] = result[i-2] + result[i-1];
        }
        result[n]
}