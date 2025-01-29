/*
1. buka terminal
2. jalankan perintah "cargo new nama-folder"
3. buka folder di code editor */

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("hello test")
}

#[test]
fn test_varable(){
    let name = "audiansyah";
    println!("helllo {}", name);
}

#[test]
fn test_mutable(){
    let mut name = "audiansyah";
    println!("hello {}", name);

    name = "audi";
    println!("hello {}", name);
}

#[test]
fn static_typing(){
    let mut name = "audiansyah";
    println!("hello {}", name);

    name = "audi";
    println!("hello {}", name);
}

#[test]
/*bisa biking nama yang sama untuk variabel yang berbeda, 
dengan catatan variabel yang sebelumnya akan tertutupi oleh yang baru */
fn shadowing(){ 
    let nama = "audiansyah";
    println!("hello {}", nama);

    let nama = 10;
    println!("hello {}", nama);
}

/*data type
seap nilai di rust memiliki tipe data. secara garis besar
rust membagi tipe data menjadi dua subsets; scalar dan compound
- scalar adalah type yang mempresentasikan single value(nilai tunggal), yaitu integer, float, boolean dan char
- compound adalah type yang mempresentasikan beberapa value(bisa lebih darti satu) dalam satu type, yaitu tuple dan array */


/*explicit type
saat kita membuat variable, secara default kita tdk tau perlu menyebutkan tipe data secara explicit,
karena rust bisa otomatis mendeteksi tipe data apa yang kita gunakan
namun, kita juga bisa menyebutkan tipe data sebuah variable secara explicit dengan menggunakan tanda : (titik dua) */
#[test]
fn explicit(){
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn number(){
    let a: i64 = 20;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

/*rust bisa melakukan konversi tipe dat dari tipe data number yang ukurannya kecil ke yg ukurannya lebih besar dan sebaliknya.
namun perlu diperhatikan jika melakukan konversi data number dari ukuran lebih besar ke ukuan lebih kecil, maka bisa terjadi yang namabya integer overflow,
yaitu kondisi dimana nilai number tidak bisa ditampung oleh tipe data tujuan konversi.
untuk melakukan konversi kita bisa mengunakan kata kunci as*/
#[test]
fn number_convertion(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);
}

//1:06:31