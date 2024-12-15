pub fn add_tuple<T: std::ops::Add<Output = T>>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 + b.0, a.1 + b.1)
}
