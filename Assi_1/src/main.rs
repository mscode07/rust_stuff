use std::ops::Index;

#[derive(Debug)]
struct Wallet{
    owner: String,
    balance: u64,
}

fn main(){

    let mut wallets:Vec<Wallet> = Vec::new();

    let user1 = String::from("Abhi1");
    let user2 = String::from("Abhi2");
    let user3 = String::from("Abhi3");

    wallets.push(Wallet { owner: user1, balance: 200 });
    wallets.push(Wallet { owner: user2, balance: 200 });
    wallets.push(Wallet { owner: user3, balance: 300 });

    for wallet in &wallets{
        println!("The owner is {} and the balance is {}", wallet.owner,wallet.balance);
    }

    println!("Total Wallets {}", wallets.len());

    println!("owner of first wallet is {:?}", wallets.index(0));

    let bal = total_balance(&wallets);

    println!("Total balance is {}", bal);

    fn total_balance(wallets: &Vec<Wallet>) -> u64{

        let mut total=0;
        for wallet in wallets{            
            total = total + wallet.balance
        }
        total

    }
}