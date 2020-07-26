package main

import "fmt"

func main() {
	loadConfig()
	// load wallet
	wallet := loadWallet()
	defer writeWalletStateFile(wallet, "wallet.dat")

	confirmedBalance, pendingBalance, err := wallet.Balance()
	if err != nil {
		fmt.Println(err)
	}

	fmt.Printf("confirmed: %+v\n", confirmedBalance)
	fmt.Printf("pending: %+v\n", pendingBalance)
}
