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
    println!("{}, {}", text, f64::EPSILON);
}

pub fn compare_floats(number_1: &f64, number_2: &f64) -> bool {
    let difference = number_1 - number_2;
    let absolute = f64::abs(difference);

    absolute < f64::EPSILON
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
    if compare_floats(&tuple_1.x, &tuple_2.x) == false {
        return false;
    }
    if compare_floats(&tuple_1.y, &tuple_2.y) == false {
        return false;
    }
    if compare_floats(&tuple_1.z, &tuple_2.z) == false {
        return false;
    }
    if compare_floats(&tuple_1.w, &tuple_2.w) == false {
        return false;
    }

    return true;
}

pub fn add_tuples(tuple_1: &Tuple, tuple_2: &Tuple) -> Tuple {
    Tuple {
        x: tuple_1.x + tuple_2.x,
        y: tuple_1.y + tuple_2.y,
        z: tuple_1.z + tuple_2.z,
        w: tuple_1.w + tuple_2.w,
    }
}

pub fn subtract_tuples(tuple_1: &Tuple, tuple_2: &Tuple) -> Tuple {
    Tuple {
        x: tuple_1.x - tuple_2.x,
        y: tuple_1.y - tuple_2.y,
        z: tuple_1.z - tuple_2.z,
        w: tuple_1.w - tuple_2.w,
    }
}

pub fn negate_tuple(tuple: &Tuple) -> Tuple {
    Tuple {
        x: -tuple.x,
        y: -tuple.y,
        z: -tuple.z,
        w: -tuple.w,
    }
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
    fn expect_floating_point_numbers_to_be_equal() {
        let number_1 = 1.0000000000000002220446049250313;
        let number_2 = 1.0000000000000002220446049250313767857;

        let equal = compare_floats(&number_1, &number_2);

        assert!(equal);
    }

    #[test]
    fn expect_floating_point_numbers_to_be_not_equal() {
        let number_1 = 1.00011;
        let number_2 = 1.00012;

        let equal = compare_floats(&number_1, &number_2);

        assert!(equal == false);
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

    #[test]
    fn expect_to_add_point_and_vector() {
        let point = create_point(3.0, -2.0, 5.0);
        let vector = create_vector(-2.0, 3.0, 1.0);

        let new_point = add_tuples(&point, &vector);

        assert_eq!(new_point.x, 1.0);
        assert_eq!(new_point.y, 1.0);
        assert_eq!(new_point.z, 6.0);
        assert_eq!(new_point.w, 1.0);
    }

    #[test]
    fn expect_to_add_two_vectors() {
        let vector_one = create_vector(3.0, -2.0, 5.0);
        let vector_two = create_vector(-2.0, 3.0, 1.0);

        let new_vector = add_tuples(&vector_one, &vector_two);

        assert_eq!(new_vector.x, 1.0);
        assert_eq!(new_vector.y, 1.0);
        assert_eq!(new_vector.z, 6.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn expect_to_add_two_points() {
        let point_one = create_point(3.0, -2.0, 5.0);
        let point_two = create_point(-2.0, 3.0, 1.0);

        let new_point = add_tuples(&point_one, &point_two);

        assert_eq!(new_point.x, 1.0);
        assert_eq!(new_point.y, 1.0);
        assert_eq!(new_point.z, 6.0);
        assert_eq!(new_point.w, 2.0);
    }

    #[test]
    fn expect_to_subtract_two_points() {
        let point_one = create_point(3.0, 2.0, 1.0);
        let point_two = create_point(5.0, 6.0, 7.0);

        let new_point = subtract_tuples(&point_one, &point_two);

        assert_eq!(new_point.x, -2.0);
        assert_eq!(new_point.y, -4.0);
        assert_eq!(new_point.z, -6.0);
        assert_eq!(new_point.w, 0.0);
    }

    #[test]
    fn expect_to_subtract_point_and_vector() {
        let point = create_point(3.0, 2.0, 1.0);
        let vector = create_vector(5.0, 6.0, 7.0);

        let new_point = subtract_tuples(&point, &vector);

        assert_eq!(new_point.x, -2.0);
        assert_eq!(new_point.y, -4.0);
        assert_eq!(new_point.z, -6.0);
        assert_eq!(new_point.w, 1.0);
    }

    #[test]
    fn expect_to_subtract_two_vectors() {
        let vector_one = create_vector(3.0, 2.0, 1.0);
        let vector_two = create_vector(5.0, 6.0, 7.0);

        let new_vector = subtract_tuples(&vector_one, &vector_two);

        assert_eq!(new_vector.x, -2.0);
        assert_eq!(new_vector.y, -4.0);
        assert_eq!(new_vector.z, -6.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn expect_to_subtract_vector_from_zero_vector() {
        let zero_vector = create_vector(0.0, 0.0, 0.0);
        let vector = create_vector(1.0, -2.0, 3.0);

        let new_vector = subtract_tuples(&zero_vector, &vector);

        assert_eq!(new_vector.x, -1.0);
        assert_eq!(new_vector.y, 2.0);
        assert_eq!(new_vector.z, -3.0);
        assert_eq!(new_vector.w, 0.0);
    }

    #[test]
    fn expect_to_negate_given_tuple() {
        let tuple = create_tuple(1.0, -2.0, 3.0, -4.0);
        let negated_tuple = negate_tuple(&tuple);

        assert_eq!(negated_tuple.x, -1.0);
        assert_eq!(negated_tuple.y, 2.0);
        assert_eq!(negated_tuple.z, -3.0);
        assert_eq!(negated_tuple.w, 4.0);
    }
}
