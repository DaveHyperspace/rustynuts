fn main() {
    let _x = 5;

    // default is f64
    let _x = 2.0;
    let _y: f32 = 3.0;

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // char type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    //------------------------
    // Compound Types
    //------------------------

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (_x, y, _z) = tup;

    println!("The value of _y is: {}", y);

    // access via index
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");

    // Array
    let _a = [1, 2, 3, 4, 5];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // explicit type annotation
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize array with same value
    let _a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    // access via index
    let _first = _a[0];
    let _second = _a[1];
}
