use core::cmp::min;

// Алгоритм нахождения наибольшего / наименьшего (на ваш выбор) числа в статическом массиве из 10 элементов
fn get_min(array: &[i32; 10]) -> i32 {
    let mut mn = array[0];
    for elem in *array {
        mn = min(mn, elem);
    }
    mn
}

fn is_prime(x : i128) -> bool {
    for i in 2..((x as f64).sqrt() as i128) + 1 {
        if x % i == 0 {
            return false
        }
    }
    
    true
}

// Алгоритм вычисления N-ого простого числа. (N <= 10000)
fn get_Ns_prime(n : i64) -> i128 {
    let mut found = 0;
    let mut x = 2;

    loop {
        if is_prime(x) {
            found += 1
        }

        if found == n {
            break x;
        }

        x += 1;
    }
}

// Алгоритм бинарного поиска числа в отсортированном статическом массиве из 10 элементов, находит первое большее переданного на вход числа
fn bin_search(array: &[i32; 10], n: i32) -> Option<i32> {
    let mut l = 0;
    let mut r = 10;
    while l + 1 < r {
        let m = (l + r) / 2;
        if array[m] <= n {
            l = m;
        }
        else {
            r = m;
        }
    }

    if r == 10 {
        None
    }
    else {
        Some(array[r])
    }
}

fn main() {
    let array: [i32; 10] = [1, 2, 3, 4, 42, -423, 12, -423, 2, 42];
    let sorted_array: [i32; 10] = [1, 2, 3, 4, 5, 65, 74, 81, 943, 1000];

    println!("My simple Array:");
    for elem in array {
        print!("{} ", elem);
    }
    println!();

    println!("My sorted Array:");
    for elem in sorted_array {
        print!("{} ", elem);
    }
    println!();

    println!("Min in simple Array: {}", get_min(&array));
    println!("Min in sorted Array: {}", get_min(&sorted_array));

    println!("The 1st prime number is {}", get_Ns_prime(1));
    println!("The 2nd prime number is {}", get_Ns_prime(2));
    println!("The 3rd prime number is {}", get_Ns_prime(3));
    println!("The 32nd prime number is {}", get_Ns_prime(32));
    println!("The 100th prime number is {}", get_Ns_prime(10000));

    println!("The first number in sorted Array greater than -5 is {:?}", bin_search(&sorted_array, -5));
    println!("The first number in sorted Array greater than 98 is {:?}", bin_search(&sorted_array, 98));
    println!("The first number in sorted Array greater than 321371 is {:?}", bin_search(&sorted_array, 321371));
}
