use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn generate_spiral(n: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; n]; n];
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, 1);
    
    for i in 1..=(n * n) as u32 {
        matrix[x][y] = i;
        let (nx, ny) = (((x as isize + dx) as usize) % n, ((y as isize + dy) as usize) % n);
        if matrix[nx][ny] != 0 {
            let temp = dx;
            dx = dy;
            dy = -temp;
        }
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
    }
    
    matrix
}

fn sum_primes(matrix: &Vec<Vec<u32>>) -> u64 {
    matrix.iter().flatten().filter(|&&n| is_prime(n)).map(|&n| n as u64).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let n = 1000;
    let matrix = generate_spiral(n);
    println!("Matriz Espiral de 1000x1000:");
    
    for row in matrix.iter().take(10) {
        println!("{:?}", row);
    }

    let sum = sum_primes(&matrix);
    println!("Suma de los números primos de la matriz: {}", sum);

    let execution_time = start_time.elapsed();
    println!("Tiempo de ejecución: {} ms", execution_time.as_millis());

    let output_dir = "outputs";
    std::fs::create_dir_all(output_dir)?;

    let file_path = Path::new(output_dir).join("output.csv");
    let file_exists = file_path.exists();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    if !file_exists {
        writeln!(file, "Lenguaje,Tiempo de Ejecución (ms)")?;
    }

    writeln!(file, "Rust,{}", execution_time.as_millis())?;

    Ok(())
}