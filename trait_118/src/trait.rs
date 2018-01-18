//rait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合

pub trait Summarizable {
	fn summary(&self) -> String;
}
pub struct NewsArticle {
	pub headline: String,
	157
		 pub location: String,
		 pub author: String,
		 pub content: String,
}
impl Summarizable for NewsArticle {
	fn summary(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}
pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}
impl Summarizable for Tweet {
	fn summary(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

let tweet = Tweet {
	username: String::from("horse_ebooks"),
	content: String::from("of course, as you probably already know, people"),
	reply: false,
	retweet: false,
};
println!("1 new tweet: {}", tweet.summary());

