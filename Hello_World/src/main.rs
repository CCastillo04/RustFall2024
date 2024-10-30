pub mod bank_account;

use bank_account::BankAccount;
fn main(){

    //new account made
    let mut myacc = BankAccount::new(300.0);
    println!("New Account Created!");
    
    //print current balance
    println!("Current Balance: {}", myacc.balance());

    //adds money and prints new balance
    myacc.deposit(50.0);
    println!("New Balance after deposit: {}", myacc.balance());

    //removes money and matches if the result is okay or err. This case should be ok
    match myacc.withdraw(70.0){
        Ok(_) => println!("Withdrawal was successful, New Balance: {}", myacc.balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    };

    //does the same as the previous match, this case fails
    match myacc.withdraw(500.0){
        Ok(_) => println!("Withdrawal was successful, New Balance: {}", myacc.balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }
}