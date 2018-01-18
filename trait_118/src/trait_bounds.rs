//我们可以限制泛型不再适用于任何类型，编译器会确保其被限制为那些实现了特定 trait 的类型，
//由此泛型就会拥有我们希望其类型所拥有的功能。这被称为指定泛型的 trait bounds

pub fn notify<T: Summarizable>(item: T) {
	println!("Breaking news! {}", item.summary());
}

//T: Summarizable + Display

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

fn some_function<T, U>(t: T, u: U) -> i32
where T: Display + Clone,
      U: Clone + Debug
{}

//使用 trait bounds 来修复 largest 函数

use std::cmp::PartialOrd;
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

//使用 trait bound 有条件的实现方法


use std::fmt::Display;
struct Pair<T> {
	x: T,
	y: T,
}
impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}
}
impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

//trait 和 trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器明确指定泛型类型需要拥有哪些行
//为。因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类型是否提供了正确的行为。
//在动态类型语言中，如果我们尝试调用一个类型并没有实现的方法，会在运行时出现错误。Rust 将这些错误移动
//到了编译时，甚至在代码能够运行之前就强迫我们修复错误。另外，我们也无需编写运行时检查行为的代码，因为
//在编译时就已经检查过了，这样相比其他那些不愿放弃泛型灵活性的语言有更好的性能。
//这里还有一种泛型，我们一直在使用它甚至都没有察觉它的存在，这就是 生命周期（lifetimes）
