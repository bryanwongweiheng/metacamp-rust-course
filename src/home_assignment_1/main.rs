struct User {
    name: String,
    balance: (f32, String),
}

impl User {
    fn print_user_detail(&self) {
        println!("name: {}, balance: {}, currency: {}", self.name, self.balance.0, self.balance.1);
    }
}

fn accrue_interest(user: &mut User, interest: f32) {
    user.balance.0 *= 1.0 + interest / 100.0;
    user.print_user_detail();
}

fn main() {
    let name = "John".to_owned();
    let ccy = "SGD".to_owned();
    let balance = (100.0, ccy);
    let mut user = User{name, balance};
    accrue_interest(&mut user, 10.0);
    accrue_interest(&mut user, 10.0);
    accrue_interest(&mut user, 10.0);
    accrue_interest(&mut user, 10.0);
}
