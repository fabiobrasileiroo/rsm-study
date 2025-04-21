mod balances;

fn main() {
    let mut pallet = balances::Pallet::new();
    pallet.set_balance("Fabio".to_string(), 2);

    let amountAccount = pallet.balance("Fabio".to_string());

    print!("{}",amountAccount);
}
