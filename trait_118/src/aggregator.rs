extern crate aggregator;
use aggregator::Summarizable;
struct WeatherForecast {
	high_temp: f64,
	low_temp: f64,
	chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
		 fn summary(&self) -> String {
			 format!("The high will be {}, and the low will be {}. The chance of
			 precipitation is {}%.", self.high_temp, self.low_temp,
			 self.chance_of_precipitation)
		 }
}
//只能在 trait 或对应类型位于我们 crate 本地的时候为其实现 trait
//不允许对外部类型实现外部 trait

//rait 实现的一个需要注意的限制是：只能在 trait 或对应类型位于我们 crate 本地的时候为其实现 trait。换句话说，
//不允许对外部类型实现外部 trait。例如，不能在 Vec 上实现 Display trait，因为 Display 和 Vec 都定义于标准
//库中。允许在像 Tweet 这样作为我们 aggregator crate 部分功能的自定义类型上实现标准库中的 trait Display 。也
//允许在 aggregator crate 中为 Vec 实现 Summarizable ，因为 Summarizable 定义与此。这个限制是我们称为 孤儿规
//则（orphan rule）的一部分，如果你感兴趣的可以在类型理论中找到它。简单来说，它被称为 orphan rule 是因为
//其父类型不存在。没有这条规则的话，两个 crate 可以分别对相同类型是实现相同的 trait，因而这两个实现会相互
//冲突：Rust 将无从得知应该使用哪一个。因为 Rust 强制执行 orphan rule，其他人编写的代码不会破坏你代码，
//反之亦是如此。
