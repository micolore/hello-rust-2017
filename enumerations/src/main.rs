fn main() {



}

enum IpAddrKind {
	V4,
	V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


fn route(ip_type: IpAddrKind) { }

route(IpAddrKind::V4);
route(IpAddrKind::V6);

enum IpAddrKind {
	V4,
	V6,
}
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}
let home = IpAddr {
	kind: IpAddrKind::V4,
	address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
	kind: IpAddrKind::V6,
	address: String::from("::1"),
};

enum IpAddr {
	V4(String),
	V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));


//每个成员可以处理不同类型和数量的数据
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);


//它正有着跟我们定义和使用的一样的枚举和成员，不过它将成员中的地址数据嵌入到了两个不同形式的结构体中，他们对不同的成员的定义是不同的：

struct Ipv4Addr {
	// details elided
}
struct Ipv6Addr {
	// details elided
}
enum IpAddr {
	V4(Ipv4Addr),
	V6(Ipv6Addr),
}


enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

Quit 没有关联任何数据。
Move 包含一个匿名结构体
Write 包含单独一个 String 。
ChangeColor 包含三个 i32 

struct QuitMessage; // unit struct
struct MoveMessage {
	x: i32,
	y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

# enum Message {
# Quit,
# Move { x: i32, y: i32 },
# Write(String),
# ChangeColor(i32, i32, i32),
# }
#
impl Message {
fn call(&self) {
	// method body would be defined here
}
}
let m = Message::Write(String::from("hello"));
m.call();


//Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T> 

enum Option<T> {
	Some(T),
	None,
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;






























