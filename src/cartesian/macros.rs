#[macro_export]
macro_rules! vec_concat {
    ($($s:expr),+ $(,)?) => {{
        const LEN: usize = $( $s.len() + )* 0;
        & {
            let mut arr = [Vector::new(0, 0); LEN];
            let mut base: usize = 0;
            $({
                let mut i = 0;
                while i < $s.len() {
                    arr[base + i] = $s[i];
                    i += 1;
                }
                base += $s.len();
            })*
            if base != LEN { panic!("invalid length"); }
            arr
        }
    }}
}
#[macro_export]
macro_rules! vecs_scale {
    ($vectors:expr, $scalar:expr) => {
        & {
            let mut arr = [Vector::new(0, 0); vectors.len()];
            for i in 0..vectors.len() {
                arr[i] = vectors[i] * scalar;
            }
            arr
        }
    };
}