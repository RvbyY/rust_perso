include!(mathlib);

fn sum_x(n: usize, array: [[i32; 2]; 2], exp: i32) -> i32
{
    let mut i = 0usize;
    let mut result_x = 0i32;

    while i < n {
        result_x += array[i][0].pow(exp.try_into().unwrap());
        i += 1;
    }
    return result_x;
}

fn sum_y(n: usize, array: [[i32; 2]; 2], exp: i32) -> i32
{
    let mut i = 0usize;
    let mut result_y = 0i32;

    while i < n {
        result_y += array[0][i].pow(exp.try_into().unwrap());
        i += 1;
    }
    return result_y;
}

fn sum_both(n: usize, array: [[i32; 2]; 2]) -> i32
{
    let mut i = 0usize;
    let mut s_result = 0i32;

    while i < n {
        s_result += array[i][0] * array[0][i];
        i += 1;
    }
    return s_result;
}

fn mean_square(n: usize, array: [[i32; 2]; 2],
    a: f32, b: f32, elem: str) -> f32
{
    let mut upper = 0f32;
    let mut i = 0usize;

    while i < n {
        if elem == 'X'
            upper += (array[i][1] - (array[i][0] - b)
                / a).pow(2.try_into().unwrap());
        else if elem == 'Y'
            upper += (array[i][1] - (a * array[i][0]
                + b)).pow(2.try_into().unwrap());
        i += 1;
    }
    return (upper / n).pow(.5.try_into().unwrap());
}

fn main() -> ()
{
    let array = [[0, 2], [9, 2]];

    let result = sum_x(2, array, 1);
    println!("Sum of my array: {}", result);
}