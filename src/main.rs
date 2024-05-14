use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Fungsi untuk membaca matriks dari file
fn read_file(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(filename); // Buat path dari nama file
    let file = File::open(&path)?; // Buka file, jika gagal return error
    let mut matrix = Vec::new(); // Inisialisasi matriks sebagai vector

    // Baca file baris demi baris
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();
        matrix.push(row);
    }
    Ok(matrix)
}

// Fungsi untuk menyelesaikan masalah TSP
fn tsp(
    mark: usize,
    position: usize,
    n: usize,
    distan: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<i32>>,
) -> i32 {
    let completed_visit = (1 << n) - 1;
    if mark == completed_visit {
        return distan[position][0]; // Jika semua kota sudah dikunjungi, kembali ke kota awal
    }
    if dp[mark][position] != -1 {
        return dp[mark][position]; // Jika sudah dihitung sebelumnya, gunakan nilai yang sudah ada
    }
    let mut answer = i32::MAX; // Inisialisasi jawaban dengan nilai maksimum
    for city in 0..n {
        // Hitung jarak baru dan rekursif untuk mencari jarak minimal
        if (mark & (1 << city)) == 0 {
            let new_answer = distan[position][city] + tsp(mark | (1 << city), city, n, distan, dp);
            answer = cmp::min(answer, new_answer);
        }
    }
    dp[mark][position] = answer;
    answer
}

fn main() {
    // Meminta input nama file dari user
    println!("Masukkan path file dari graf: ");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Gagal membaca input");
    let filename = filename.trim();

    let distan = read_file(filename).expect("Gagal membaca file");

    let n = distan.len();
    let mut dp = vec![vec![-1; n]; 1 << n];

    let min_distance = tsp(1, 0, n, &distan, &mut dp);
    println!("Jarak minimum: {}", min_distance);
}
