# Flipkart - Mini-Wallet 
You are tasked with designing a Flipkart Wallet system. This system allows users to perform various operations related to their wallet, such as loading money, sending money, and managing fixed deposits. The following are the requirements and assumptions for the system.

## Requirements: 
1. User Registration 
    - Users need to register on Flipkart to use this wallet. 
2. Deposit Money (Add money to waliet) 
    - Users can load money into their wallet via various sources (Credit Card, Debit Card, UPI, etc.). 
    - No need for integration from sources; assume the operation is always successful. 
    - The minimum a amount of load money should be greater than 0 
    - Rules on deposit based on instrument 
        - Credit Card 3 transactions per Credit Card allowed 
        - UPI 1000 Rs max per transaction allowed
        - Debit Card: Transaction penalty of 1 Rs after 3 transactions with debit card
        - Should be made configurable and extensible.
        - If the above conditions are violated, we should block the transaction.
3. Send Money 
    - Users can send money to other users from their wallet.
    - The minimum amount for a transaction is always greater than 0. 
    - Users must have sufficient balance in their wallet to complete the transaction. 
4. Fixed Deposil (FD):
    - Users can block a certain amount as a fixed deposit from the wallet. This amount will be reduced from the wallet balance. 
    - A user can have one deposit for a particular deposit type at a time. 
    - Users should have the ability to dissolve the FD. After doing so, the FD amount should be unblocked for regular transactions.
5. Wallet Balance
    - Users can fetch their wallet balance at any point in time. This should consider both credit & debit tvne transactions
6. Transaction History: 
    - Users can get their transaction history sorted by: 
        - Amount ("amount") 
        - Transaction DateTime ("time") 
    - Users can filter their transaction history based on: Credit or Debit Amount

## BONUS
The transactions (user to user) will be eligible for cashback if they meet their respective criteria. Let's say criteria can be, based on "after each 5 transactions" or "With 0.5 probability for each transaction", provide amount as cashback and added to the user's wallet.

## Capabilities 
Below are various functions that should be supported with the necessary parameters passed. These are just suggestions, the signatures can be altered as long as the functionalities are provided.
1. registerUser() - Register user to use wallet 
2. topUpWallet() - Add money to the wallet for users with any of the resources. 
3. fetchBalance() - Fetch balance available in the wallet for user 
4. sendMoney() - Send money to the user. Each money transfer will be considered as a transactions
5. getTransactions(filter, sorter) - fetches transactions history based on filter and sorting criteria 
6. createDeposit() - Add money from wallet to deposit of the type mentioned. 
7. bookDeposit() - Dissolves amount present in deposit and adds the amount to wallet
        

## my notes before coding : 
type of queries
    - register_user 
    - top_up
    - fetch_bal
    - send_money
    - get_transections
    - create_deposite
    - book_deposite

user {
    - name: String
    - wallet: Wallet
    - FD: u32
    - Transections: Vec<Transection>
}

wallet {
    - balance: u32,
}

Transection {
    - type: deposite/ top_up/ send/ deposite
    - opposite_party: Option<String>
    - ammount: u32
}

## Sample Commands
1. REGISTER nonu
2. REGISTER toni
3. FETCH_BAL nonu
4. TOP_UP nonu CC 100
5. CREATE_DEPOSITE nonu FD 10
6. BOOK_DEPOSITE nonu FD
7. SEND_MONEY nonu toni 20
8. GET_TRANSACTIONS nonu credit time




