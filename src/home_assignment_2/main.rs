#[derive(Debug, PartialEq, Clone, Copy)]
enum PaymentType {
    DigitalToken,
    Cash,
}

#[derive(Debug)]
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

#[derive(Debug)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

#[derive(Debug)]
struct BuyerGroup {
    members: Vec<Buyer>,
}

impl BuyerGroup {
    fn add_member(&mut self, buyer: Buyer) {
        self.members.push(buyer);
    }

    fn find_buyer(&mut self, payment_type: PaymentType) -> i32 {
        let mut index: i32 = 0;
        for buyer in &self.members {
            if buyer.payment_type == payment_type {
                return index;
            }
            index += 1;
        }
        return -1;
    }

    fn buy(&mut self, buyer_index: i32, seller: &mut Seller) {
        if buyer_index < 0 {
            return;
        }
        println!("buyer index: {:?}", buyer_index);
        let buyer = &mut self.members[buyer_index as usize];
        while buyer.balance >= seller.price {
            buyer.balance -= seller.price;
            seller.balance += seller.price;
        }
        println!("{:?} {:?}", buyer, seller);
    }
}


fn main() {
    let john = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };
    let sally = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00,
    };
    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };
    let mut buyer_group = BuyerGroup{members: Vec::new() };
    buyer_group.add_member(john);
    buyer_group.add_member(sally);
    let buyer_index = buyer_group.find_buyer(seller.payment_type);
    buyer_group.buy(buyer_index, &mut seller);
}
