use crate::core;
struct Users {
    username:String,
    fullname:String,
    password:String
}
pub fn show_data(){
    let me = Users {
        username:"zulfikra".to_string(),
        fullname:"zulfikra96".to_string(),
        password:"zulfikra".to_string()
    };
//    core::connection::conn().execute("INSERT INTO users (username, fullname, password) VALUES($1,$2,$3)",
//         &[&me.username, &me.fullname, &me.password])
//         .unwrap();
    println!("ini show data");
}