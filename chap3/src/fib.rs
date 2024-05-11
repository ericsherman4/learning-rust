
pub fn fib_sequence(n: u32) -> u32 {
    if n == 0
    {
        0
    }
    else if n == 1 {
        1
    }
    else {
        // fib_sequence(n-1) + fib_sequence(n-2)
        let mut seq = [0,1,1];
        for _ in 2..(n+1) {
            seq[2] = seq[0] + seq[1];
            seq[0] = seq[1];
            seq[1] = seq[2];
        }
        seq[2]
    }
}