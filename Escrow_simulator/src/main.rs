#[derive(PartialEq)]

enum EscrowStatus{
    Created,
    Funded,
    Released,
    Cancelled,
}

struct Escrow{
    buyer: String,
    seller: String,
    amount: u64,
    status: EscrowStatus
}

impl Escrow {
    
    fn new(buyer:String, seller: String, amount:u64,status:EscrowStatus) -> Self{
        return Self { buyer, seller, amount, status };
    }

    fn fund(&mut self) -> Result<(),String>{
        match self.status {
            EscrowStatus::Created => {
                self.status = EscrowStatus::Funded;
                Ok(())
            }
            _ => Err("Escrow can only be funded from created state".to_string())
        }
    }

    fn release(&mut self) -> Result<(),String> {

        match self.status {
             EscrowStatus::Funded => {
                self.status = EscrowStatus::Released;
                Ok(())
             }
             _ => Err("Erro".to_string())
            
        }

        // if self.status == EscrowStatus::Funded{
        //     self.status = EscrowStatus::Released
        // }
        // Ok(())
    }

    fn cancelled(&mut self) -> Result<(),String>{
        if self.status == EscrowStatus::Created{
            self.status = EscrowStatus::Cancelled
        }
        Ok(())
    } 
}

fn main() {
    let mut escrow = Escrow::new(String::from("Abhi"), String::from("mscode"), 500, EscrowStatus::Created);

    match escrow.fund() {
        Ok(()) => println!("Escrow funded"),
        Err(e) => println!("{}",e)
    };

    match escrow.release(){
        Ok(()) => println!("Escrow release"),
        Err(e)=> println!("{}",e)
    }

    match escrow.cancelled() {
        Ok(()) => println!("Escrow cancelled"),
        Err(e)=> println!("{}",e)
    }

}
