fn main() {
    println!("Hello, world!");
}


# struct User {
# username: String,
# email: String,
# sign_in_count: u64,
# active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
	User {
	     email: email,
 	     username: username,
	     active: true,
	     sign_in_count: 1,
        }
}
 
//two init field 变量与字段同名时的字段初始化语法 


# struct User {
# username: String,
# email: String,
# sign_in_count: u64,
# active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
	User {
	      email,
	      username,
	      active: true,
              sign_in_count: 1,
          }
 }

//使用结构体更新语法从其他对象创建对象

# struct User {
# 	username: String,
# 	email: String,
# 	sign_in_count: u64,
# 	active: bool,
# }
#
# let user1 = User {
# 	email: String::from("someone@example.com"),
# 	username: String::from("someusername123"),
# 	active: true,
# 	sign_in_count: 1,
# };
#
let user2 = User {
	email: String::from("another@example.com"),
	username: String::from("anotherusername567"),
	active: user1.active,
	sign_in_count: user1.sign_in_count,
};



//结构体更新语法利用.. 以指定未显式设置的字段应有与给定实例对应字段相同的值

# struct User {
# username: String,
# email: String,
# sign_in_count: u64,
# active: bool,
# }
#
# let user1 = User {
# email: String::from("someone@example.com"),
# username: String::from("someusername123"),
# active: true,
# sign_in_count: 1,
# };
#
let user2 = User {
 	email: String::from("another@example.com"),
 	username: String::from("anotherusername567"),
	..user1
};


//使用没有命名字段的元组结构体创建不同的类型

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

//没有任何字段的类单元结构体
//他们被称为 类单元结构体（unit-like structs）因为他们类似于 () ,即 unit 类型

struct User {
	username: &str,
	email: &str,
	sign_in_count: u64,
	active: bool,
}
fn main() {
   let user1 = User {
	email: "someone@example.com",
	username: "someusername123",
	active: true,
	sign_in_count: 1,
  };
}












