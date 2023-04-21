// Define an Enum PaymentType with variants:
// DigitalToken
// Cash
#[derive(Debug, PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash,
}
// Define a Seller struct which contains 3 fields:
// payment_type (PaymentType)
// price (f32)
// balance (f32)
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}
// Define a Buyer struct which contains 3 fields:
// name (String)
// payment_type (PaymentType)
// balance (f32)
#[derive(Debug)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}
// Define a BuyerGroup struct which contains:
// a vector of members (a vector of Buyer struct).
struct BuyerGroup {
    members: Vec<Buyer>,
}
// Implement methods on BuyerGroup:
// define method add_member
// which adds a Buyer into members vector
impl BuyerGroup {
    fn add_member(&mut self, h: Buyer) {
        self.members.push(h);
    }
    // define method find_buyer which accepts a PaymentType input
    // that finds returns index of Buyer with matching payment_type, otherwise return -1
    fn find_buyer(&self, payment_type: &PaymentType) -> i32 {
        println!("Searching for Buyer with payment_type {:?}", payment_type);
        let mut pos = 0;
        for i in &self.members {
            if i.payment_type == *payment_type {
                println!(
                    "Matching Buyer using PaymentType {:?} was found at index {}",
                    payment_type, pos
                );
                println!("{:?}", i);
                return pos;
            }
            pos = pos + 1;
        }
        println!("Buyer with PaymentType {:?} NOT found", payment_type);
        return -1;
    }

    // define buy method which accepts a buyer index and a reference to a seller
    // keeps transferring value of seller's price from buyer to seller, until buyer's balance is insufficient
    fn buy(&mut self, buyer_index: i32, seller: &mut Seller) {
        let mut buyer = &mut self.members[buyer_index as usize];
        loop {
            if buyer.balance >= seller.price {
                seller.balance += seller.price;
                buyer.balance -= seller.price;
                println!(
                    "{} balance : {}. Seller balance: {}",
                    buyer.name, buyer.balance, seller.balance
                );
            } else {
                println!(
                    "{} balance {} insufficient. Seller balance: {}",
                    buyer.name, buyer.balance, seller.balance
                );
                break;
            }
        }
    }
}

fn main() {
    // Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and balance of 100.00 and 100.00 respectively
    let buyer1 = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };

    let buyer2 = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00,
    };
    // Create an empty BuyerGroup
    let mut buyer_group = BuyerGroup {
        members: Vec::new(),
    };
    // Add 2 buyers (John and Sally) into buyer_group sequentially

    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

    // Create 1 seller with payment_type of Cash, price of 10, balance of 0

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };
    // Call find_buyer method on the buyer group to get index of buyer with Cash payment type

    let buyer_index = buyer_group.find_buyer(&seller.payment_type);
    if buyer_index >= 0 {
        // Call buy method on the buyer group passing the index of we have obtained right before and the seller

        buyer_group.buy(buyer_index, &mut seller);
    }
}
