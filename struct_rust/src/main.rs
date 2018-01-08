fn main() {
    //使用结构体组织相关联的数据

    //元组&结构体


}

struct User{
   username : String,
   email : String,
   sign_in_count : u64,
   active : bool,
}

let mut user1 = User {
   email : String::from("12@qq.com"),
   username : String::from("tony"),
   active : true,
   sign_in_count : 1,
}

user1.email = String::from("23@qq.com");

