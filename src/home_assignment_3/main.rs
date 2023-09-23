struct UserAccount {
    name: String,
    age: Option<u32>,
}

trait Balance {
    fn get_balance(&self) -> u32 {
        10
    }
}

impl Balance for UserAccount {
    fn get_balance(&self) -> u32 {
        20
    }
}

fn increase_balance<T: Balance>(balance: &T, amount: u32) -> Result<u32, String> {
    if amount <= 10 {
        Ok(balance.get_balance() + amount)
    } else {
        Err("Increase must be less than 10!".to_owned())
    }
}

fn main() {
    let user_account = UserAccount{name: "John".to_owned(), age: None};
    match increase_balance(&user_account, 11) {
        Ok(new_balance) => println!("UserAccount balance increased to {}", new_balance),
        Err(err) => println!("{}", err),
    }
    if let Some(age) = user_account.age {
        println!("{}'s age is {}", user_account.name, age)
    }

}
