use std::io;

fn main() {
    let mut fib1 = 1;
    let mut fib2 = 1;

    let mut n = String::new();
    
    println!("Введите номер элемента ряда Фибоначчи:");
    io::stdin()
        .read_line(&mut n)
        .expect("Ошибка чтения ввода");

    let n: i32 = n.trim().parse().expect("Введите число");

    let mut i = 0;

    while i < n - 2 {
        let fib_sum = fib1 + fib2;
        fib1 = fib2;
        fib2 = fib_sum;

        i = i+1;
    }
    println!("Значение n-го числа Фибоначчи: {fib2}");
}
