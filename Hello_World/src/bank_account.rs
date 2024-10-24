pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method

        //adds money to balance of self
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        // Implement this method

        //checks if balance is smaller than the amount withdrawed
        if self.balance >= amount{
            //if it smaller removes amount and gives result ok
            self.balance -= amount;
            Ok(())
        }
        else{
            //if larger, gives an error of not enough funs
            Err(String::from("Insufficient funds"))
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method

        //prints self's balance
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        //test for creating a new account
        let account = BankAccount::new(100.0);

        //asserts it is made with a balance of 100
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        //Test for depositing money

        //creates account
        let mut account = BankAccount::new(30.0);
        
        //calls deposit to add amount
        account.deposit(30.0);

        //asserts it is equal to both of the values added
        assert_eq!(account.balance(),60.0);
    }

    #[test]
    fn test_withdraw() {
        // Test for withdrawing money: success

        //creates account
        let mut account = BankAccount::new(100.0);
        
        //has the result be the new balance of account when money is removed
        let result =account.withdraw(30.0);
        
        //double checks that result has given ok
        assert!(result.is_ok());

        //checks if account is equal to its new balance
        assert_eq!(account.balance(),70.0);
    }

    fn test_withdraw_fail(){
        // Test for withdrawing money: failure

        //creates account
        let mut account = BankAccount::new(20.0);

        //has the result be the new balance of account when money is removed
        let result =account.withdraw(30.0);

        //checks that an error is given
        assert!(result.is_err());

        //checks if that the balance has not changed
        assert_eq!(account.balance(),20.0);
    }
}