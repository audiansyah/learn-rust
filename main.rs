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

// operator
#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
}

// augmented assignments
#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

//comparison opertators >, <, <=, >=, ==
#[test]
fn  comparison(){
    let a = 10;
    let b = 20;
    let result = a > b;

    println!("{}", result);
}

// boolean operator &&(dan), ||(or), !(kebalikan)
#[test]
fn boolean_operator(){
    let absen = 75;
    let nilai_akhir = 80;
    
    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

//char
#[test]
fn  char(){
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

//tuple: tipe dara kumpulan lebih dari satu tipe data dan jumlahnya sudah final(tidak bisa berkurang ataupun bertambah)
#[test]
fn  tuple(){
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
    //akses tuple
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    //destructuring
    let(a, b, c) = data;
    println!("{} {} {}", a, b, c);

    //mutable
    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit(){
    println!("hello");
}

//unit(tuple kosong)
#[test]
fn  test_unit(){
    let result: () = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

//aray
#[test]
fn  array(){
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    //akses array
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b, );
}

//mutable aray
#[test]
fn  mut_array(){
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    //akses array
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b, );

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

//two_dimensional_array
#[test]
fn  two_dimensional_array(){
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];


    //akses array
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
}

//constant
#[test]
fn  coonstant(){
    const MAXIMUM: i32 = 100;

    println!("{}", MAXIMUM);
}

//variable scope
#[test]
fn  variable_scope(){
    let audi = 1;

    {
        println!("inner audi: {}", audi);
        let syawal = 2;
        println!("inner syawal: {}", syawal);
    }
}

/*garbage collection, stack and heap
 pada prog-lang biasanya, garbage collection berguna untuk memantau data yang sudah tdak di gunakan di memory dengan menghapusnya secara otomatis.
 
 sedangkan rust, membagi data di memory dalam dua bagian, yaitu stack and heap
 - stack adalah bagian dimana data disimpan dalam struktur tumpukan "last in, first out", semua data di stack harus fixed size
 - heap adalah seperti tempat untuk menyimpann data, dimana untuk tempat menyimpan data di heap kita akan melakukan request ke heap,
   lalu didalam heap terdapat memory allocator yang bertugas untuk menemukan area kosong untuk menyimpan dan mengalokasikan data ke area tersebut.
   seteah itu kita akan digeri pointer ke lokasi dimana data itu berada di heap.

   pointer dari heap berukuran fix sized, oleh karena itu pointer tersebut akan di simpan di stack.
  */ 
#[test]
fn  stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("audi");

    println!("{} {}", a, b);
}
fn function_b(){
    let a = 10;
    let b = String::from("syawal");

    println!("{} {}", a, b);
}

/* drop function
- adalah function untuk menghapus data, sehingga akan di bersihkan dari heap
- saat variable keluar dari scopenya, yang artinya tidak bisa di akses lagi, secara otomatis rust akan  memanggil drop function
- dan jika rust function() sudah selesai di eksekusi, maka function tersebut akan dihapus pula dari stack frame
- oleh karena itu, rust tidak membutuhkan garbage collection ataupun manual memory management */

/* &str dan string
- rust memiliki type data text  yang fixed size, yaitu &str(string slice) dan yang bisa mengembang ukurannya yaitu string
- &str karena ukurannya fixed size, jadi rust akan menyimpan di stack, sefangkan sting karena bisa mengembang, maka disimpan di heap 

Immutable str
- karena ukuran &str adalah fixed size, maka operasi &str adalah tipe data yang immutable, artinya isi data &str tidak bisa diubah
- ketika kita buat mutable, dan mengubah data &str, sebenernya yang dilakukan adalah mengganti isi variablem bukan mengubah isi dari &str

docs: https://doc.rust-lang.org/std/primitive.str.html */
#[test]
fn  string(){
    let name: &str = "   Audiansyah   ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn  string_type(){
    let mut name: String = String::from("audiansyah");
    name.push_str(" Syawalhan");
    println!("{}", name);

    let audi = name.replace("audiansyah", "audi");
    println!("{}", audi);
}

/*ownership
adalah fitur yang digunakan oleh rust untuk menjadikan rust menjadi prog-lang yang aman dalam mengolah data di memory
tanpa harus adanya fitur garbage collection atau manual memory management.
untuk melakukan data management di memory, yang wajib di mengerti karena ini berdampak ke hampir semua fitur

ownership rules:
- sreiap value fi rust harus punya owner
- dalam satu wakrtu, hanya boleh ada satu owner
- ketika owner keluar scope, value akan dihapus
*/

#[test]
fn ownership_rule() {
    // a tidak bisa di akses disini, belum di deklarasikan
    let a = 10; // a bisa mulai di akses

    {// b tidak bisa diakses disini, karena belum di deklarasikan
        let b = 20; // b bisa di akses mulai disini
        println!("{}", b);
    }//scope b selesai disini, b dihapus dan tidak bisa di akses lagi

    println!("{}", a);
}//scope a selesai disini, a dihapus dan tidak bisa di akses lagi

/* data copy
- sesuai aturan di own rule, setiap value harus dimiliki oleh satu ower pada satu waktu.
- ketika kita berhasi dengan data maka akan dimiliki hanya oleh satu own.
- semua data yg bersifat fixed size, ketika ditambahkan ke variable baru, akan memiliki data hasil copy dari variable lama
- oleh karena itu, setiap data akan selalu dimiliki oleh satu own pada satu waktu
*/
#[test]
fn data_copy() {
    let a = 10;
    let b = a; //copy data dari a ke b

    println!("{} {}", a, b);
}

/*own movement
- data copy tidak terjhadi untuk tipe data yang disimpan di heap
- seperti aturan di ownership, dalam waktu satu waktu value hanya dimiliki satu own
- maka kita coba buat variable baru dari variable lama, maka yang terjhadi bukanlan copy, melainkan transfer ownership dari own lama ke own baru
- setelah proses transfer selesai, secara otomats own lama akan dianggap tidak valid lagi digunakan
 */

#[test]
fn ownership_movement() {
    let name1 = String::from("audi");

    //ownership dari name1 dipindahkan ke name1
    let name2 = name1;

    println!("{}", name2);
}

/*clone
- sekarang kita tahu bahwa data di stack akan di copy sedangkan data di heap akan di pindahkan ownershipnya
- lantas bagaimana jika kita juga ingin melaklukan copy untuk data di heap?
- maka kita harus melakukan clone
- clone adalah membuat data tiruan yang sama dari data aslinya
- string memiliki method clone() untuk melakukan ini
- saat kita memanggil method clone() maka method tersebut akan meng-copy data string menjadi data string baru
- semua tipe data yang di simpan di heap di rust memiliki method clone()
 */
#[test]
fn clone() {
    let name1 = String::from("audi");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}