//default 
pub trait Summarizable {
	fn summary(&self) -> String {
		String::from("(Read more...)")
	}
}

//重载
impl Summarizable for NewsArticle {}

let article = NewsArticle {
	headline: String::from("Penguins win the Stanley Cup Championship!"),
	location: String::from("Pittsburgh, PA, USA"),
	author: String::from("Iceburgh"),
	content: String::from("The Pittsburgh Penguins once again are the best
	hockey team in the NHL."),
};
println!("New article available! {}", article.summary());

pub trait Summarizable {
	fn author_summary(&self) -> String;
	fn summary(&self) -> String {
		format!("(Read more from {}...)", self.author_summary())
	}
}

impl Summarizable for Tweet {
	fn author_summary(&self) -> String {
		format!("@{}", self.username)
	}
}

let tweet = Tweet {
	username: String::from("horse_ebooks"),
	content: String::from("of course, as you probably already know, people"),
	reply: false,
	retweet: false,
};
println!("1 new tweet: {}", tweet.summary());

//注意在重载过的实现中调用默认实现是不可能的



