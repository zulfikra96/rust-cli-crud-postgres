use std::io;
use std::mem;

mod core;
// struct Users {
//     username:String,
//     fullname:String,
//     password:String
// }

fn main(){
    let mut input = String::new();
    let mut is_loop:bool = true;
    while is_loop {
        println!("pilih menu !");
        println!("======================");
        println!("1.tampilkan data");
        println!("2.input data diri");
        println!("3.hapus data diri");
        println!("4.ubah data diri");
        println!("5.Keluar");
        println!("=======================");
        io::stdin().read_line(&mut input)
            .expect("maaf ada yang salah");  
        // choose = input.parse::<i32>().unwrap();
        let mut choose = input.trim();
        
        let choose_int : i32 = match choose.parse::<i32>() {
            Ok(i) =>  i,
            Err(e) => 0,
        };   

        if choose_int == 1{
            core::index::show_data();
        }
    }
    
    // let me = Users {
    //     username:"zulfikra".to_string(),
    //     fullname:"zulfikra96".to_string(),
    //     password:"zulfikra".to_string()
    // };

    // connection::conn().execute("INSERT INTO users (username, fullname, password) VALUES($1,$2,$3)",
    //     &[&me.username, &me.fullname, &me.password])
    //     .unwrap();
    // println!("Hello, world!");
}
