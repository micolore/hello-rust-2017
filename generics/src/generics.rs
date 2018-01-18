//提取函数来减少重复
fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let mut largest = number_list[0];
	for number in number_list {
		if number > largest {
			largest = number;
		}
	}
	println!("The largest number is {}", largest);
	# assert_eq!(largest, 100);
}

//2

fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let mut largest = number_list[0];
	for number in number_list {
		if number > largest {
			largest = number;
		}
	}
	println!("The largest number is {}", largest);
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let mut largest = number_list[0];
	for number in number_list {
		if number > largest {
			largest = number;
		}
	}
	println!("The largest number is {}", largest);
}

//3
fn largest(list: &[i32]) -> i32 {
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}
fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("The largest number is {}", result);
	# assert_eq!(result, 100);
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let result = largest(&number_list);
	println!("The largest number is {}", result);
	# assert_eq!(result, 6000);
}
//过程
//1. 我们注意到了重复代码。
//2. 我们将重复代码提取到了一个函数中，并在函数签名中指定了代码中的输入和返回值。
//3. 我们将两个具体的存在重复代码的位置替换为了函数调用。

//泛型数据类型

//在函数定义中使用泛型
fn largest_i32(list: &[i32]) -> i32 {
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}
fn largest_char(list: &[char]) -> char {
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}
fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest_i32(&number_list);
	println!("The largest number is {}", result);
	# assert_eq!(result, 100);
	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest_char(&char_list);
	println!("The largest char is {}", result);
	# assert_eq!(result, 'y');
}

//fn largest<T>(list: &[T]) -> T {}

fn largest<T>(list: &[T]) -> T {
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}
fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("The largest number is {}", result);
	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);
	println!("The largest char is {}", result);
}

//结构体定义中的泛型

struct Point<T> {
	x: T,
	y: T,
}
fn main() {
	let integer = Point { x: 5, y: 10 };
	let float = Point { x: 1.0, y: 4.0 };
}

struct Point<T> {
	x: T,
	y: T,
}
fn main() {
	let wont_work = Point { x: 5, y: 4.0 };
}

struct Point<T, U> {
	x: T,
	y: U,
}
fn main() {
	let both_integer = Point { x: 5, y: 10 };
	let both_float = Point { x: 1.0, y: 4.0 };
	let integer_and_float = Point { x: 5, y: 4.0 };
}

//枚举定义中的泛型数据类型

enum Option<T> {
	Some(T),
	None,
}

enum Result<T, E> {
	Ok(T),
	Err(E),
}

//方法定义中的枚举数据类型

struct Point<T> {
	x: T,
	y: T,
}
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}
fn main() {
	let p = Point { x: 5, y: 10 };
	println!("p.x = {}", p.x());
}


impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}


struct Point<T, U> {
	x: T,
	y: U,
}

impl<T, U> Point<T, U> {
	fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}
fn main() {
	let p1 = Point { x: 5, y: 10.4 };
	let p2 = Point { x: "Hello", y: 'c'};
	let p3 = p1.mixup(p2);
	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个将泛型代码转变为实
//际放入的具体类型的特定代码的过程
























































