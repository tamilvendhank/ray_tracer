#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub fn get_hello() -> &'static str {
    "Hello, Mr. Tamil Vendhan Kanagarasu!"
}

pub fn display_text(text: &str) {
    println!("{}", text);
}

pub fn create_tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn create_point(x: f64, y: f64, z: f64) -> Tuple {
    create_tuple(x, y, z, 1.0)
}

pub fn create_vector(x: f64, y: f64, z: f64) -> Tuple {
    create_tuple(x, y, z, 0.0)
}

pub fn tuple_equal(tuple_1: &Tuple, tuple_2: &Tuple) -> bool {
    if tuple_1.x != tuple_2.x {
        return false;
    }
    if tuple_1.y != tuple_2.y {
        return false;
    }
    if tuple_1.z != tuple_2.z {
        return false;
    }
    if tuple_1.w != tuple_2.w {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_return_written_text() {
        assert_eq!("Hello, Mr. Tamil Vendhan Kanagarasu!", get_hello());
    }

    #[test]
    fn create_point_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w = 1.0;

        let tuple = create_tuple(x, y, z, w);

        assert_eq!(tuple.x, x);
        assert_eq!(tuple.y, y);
        assert_eq!(tuple.z, z);
        assert_eq!(tuple.w, w);
    }

    #[test]
    fn expect_to_create_a_vector_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w = 0.0;

        let tuple = create_tuple(x, y, z, w);

        assert_eq!(tuple.x, x);
        assert_eq!(tuple.y, y);
        assert_eq!(tuple.z, z);
        assert_eq!(tuple.w, w);
    }

    #[test]
    fn expect_to_create_a_point() {
        let x = 4.0;
        let y = -4.0;
        let z = 3.0;

        let tuple = create_point(x, y, z);

        assert_eq!(tuple.x, x);
        assert_eq!(tuple.y, y);
        assert_eq!(tuple.z, z);
        assert_eq!(tuple.w, 1.0);
    }

    #[test]
    fn expect_to_create_a_vector() {
        let x = 4.0;
        let y = -4.0;
        let z = 3.0;

        let tuple = create_vector(x, y, z);

        assert_eq!(tuple.x, x);
        assert_eq!(tuple.y, y);
        assert_eq!(tuple.z, z);
        assert_eq!(tuple.w, 0.0);
    }

    #[test]
    fn expect_tuples_to_be_equal() {
        let tuple_1 = create_tuple(4.0, -4.0, 3.0, 1.0);
        let tuple_2 = create_tuple(4.0, -4.0, 3.0, 1.0);

        let equal = tuple_equal(&tuple_1, &tuple_2);

        assert_eq!(equal, true);
    }

    #[test]
    fn expect_tuples_not_to_be_equal() {
        let tuple_1 = create_tuple(4.0, -4.0, 3.0, 1.0);
        let tuple_2 = create_tuple(4.0, -4.0, 3.0, 0.0);

        let equal = tuple_equal(&tuple_1, &tuple_2);

        assert_eq!(equal, false);
    }
}
