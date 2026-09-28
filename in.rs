struct In(std::str::SplitWhitespace<'static>);

impl In {
    fn new() -> Self {
        let mut buf = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();

        Self(Box::leak(buf.into_boxed_str()).split_whitespace())
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.0.next().unwrap().parse().ok().unwrap()
    }
}
