use ndarray::{arr1, arr2, Array, Array1};

fn main() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);

    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("{}", a.dot(&b));

    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);


}
