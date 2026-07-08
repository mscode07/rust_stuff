#[derive(Debug,PartialEq)]
enum AccountStatus{
    Active,
    Frozen,
    Closed,
}

enum TransactionType{
    Deposit,
    Withdraw,
    Transfer,
}

struct BankAccount{
    owner : String,
    balance: u64,
    status: AccountStatus,
    created_at: u64,
    last_transaction_type: Option<TransactionType>,
    last_transaction_amount: Option<u64>,
    last_transaction_time: Option<u64>,
    authority: String,
}

impl BankAccount {
    
    fn new(owner:String,created_at:u64,authority:String
    ) -> Self{
            Self{owner,
            balance: 0,
            status: AccountStatus::Active,
            created_at,
            last_transaction_type: None,
            last_transaction_amount: None,
            last_transaction_time: None,
            authority,
                }
            }

    fn deposit(&mut self, amount:u64, time:u64) -> Result<(),String>{

        if self.status != AccountStatus::Active{
            return Err("Sorry, Something went wrong".to_string())
        }

        if amount<0{
            return Err("Sorry, Something went wrong".to_string())
        } 
            if amount > 0{
                self.balance = self.balance + amount;
                self.last_transaction_amount = Some(amount);
                self.last_transaction_time = Some(time);
                self.last_transaction_type = Some(TransactionType::Deposit);
            }             
        Ok(())
    }

    fn show(&self){

        println!("The owner of the account is {}, with the balance of {}, 
        the status of the last Transaction is {:?}, the time of the latest transaction is {:?}",self.owner,self.balance,self.status, self.last_transaction_time)
    }

    fn withdraw(&mut self, amount:u64,authority:String) -> Result<(),String>{

        if self.status != AccountStatus::Active{
            return Err("The account is not active".to_string())
        }

        if amount < 0 {
            return Err("The amount should be greater then 0".to_string());
        }

        if authority != self.authority{
            return Err("Unauthorised".to_string());
        }

        if self.balance < amount {
            return Err("Sorry, you don't have enough Balance.".to_string());
        }

        self.balance -= amount;
        self.last_transaction_type = Some(TransactionType::Withdraw);


        Ok(())
    }


}

fn main(){
    let mut account = BankAccount::new(String::from("Abhi"), 123500, String::from("abhi_wallet"));

    match account.deposit(500, 123500) {
        Ok(()) => println!("Deposit Successful"),
        Err(error) => println!("Error {}", error)
    }

    match account.withdraw(200,String::from("wrong_wallet")) {
        Ok(()) => println!("Withdraw Successful"),
        Err(error) => println!("Error here{}", error)
    }
    match account.withdraw(200,String::from("abhi_wallet")) {
        Ok(()) => println!("Withdraw Successful"),
        Err(error) => println!("Error {}", error)
    }

    account.show();

println!("HHHH")
}