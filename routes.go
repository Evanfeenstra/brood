package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/iotaledger/goshimmer/client/wallet"
	walletseed "github.com/iotaledger/goshimmer/client/wallet/packages/seed"
	"github.com/iotaledger/goshimmer/dapps/valuetransfers/packages/balance"
	"github.com/iotaledger/hive.go/bitmask"

	"github.com/mr-tron/base58"
	"github.com/shibukawa/configdir"
)

var walletState *wallet.Wallet

var shimmerURL string

const (
	vendorName = "evanfeenstra"
	appName    = "brood"
	walletPath = "wallet.dat"
)

// run this after every update to state
func writeWalletState() error {
	state := walletState.ExportState()

	configDirs := configdir.New(vendorName, appName)
	folders := configDirs.QueryFolders(configdir.Global)
	if len(folders) == 0 {
		return errors.New("no file")
	}
	folders[0].WriteFile(walletPath, state)
	return nil
}

func checkWallet(w http.ResponseWriter, r *http.Request) {

	check := Check{}
	body, err := ioutil.ReadAll(r.Body)
	r.Body.Close()
	err = json.Unmarshal(body, &check)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	shimmerURL = check.URL

	req, err := http.Get(check.URL + "/info")
	if err != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	defer req.Body.Close()

	info := InfoReq{}
	res, err := ioutil.ReadAll(req.Body)
	err = json.Unmarshal(res, &info)
	if err != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	hasWallet := false
	configDirs := configdir.New(vendorName, appName)
	folder := configDirs.QueryFolderContainsFile(walletPath)
	if folder != nil { // has file
		hasWallet = true
		err = reloadWallet(check.URL, folder)
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		fmt.Printf("wallet state: %+v\n", walletState)
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(InfoRes{
		IdentityID: info.IdentityID,
		Synced:     info.Synced,
		Version:    info.Version,
		HasWallet:  hasWallet,
	})
}

func createWallet(w http.ResponseWriter, r *http.Request) {

	check := Check{}
	body, err := ioutil.ReadAll(r.Body)
	r.Body.Close()
	err = json.Unmarshal(body, &check)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	seed := walletseed.NewSeed()
	lastAddressIndex := uint64(0)
	spentAddresses := []bitmask.BitMask{}
	assetRegistry := wallet.NewAssetRegistry()

	walletState = wallet.New(
		wallet.WebAPI(check.URL),
		wallet.Import(seed, lastAddressIndex, spentAddresses, assetRegistry),
	)

	err = writeWalletState()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{
		"seed": base58.Encode(seed.Bytes()),
	})
}

func getState(w http.ResponseWriter, r *http.Request) {
	err := loadWallet()

	confirmedBalance, pendingBalance, err := walletState.Balance()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	colorIOTA := balance.ColorIOTA

	coins := []Coin{}
	cb := BalanceRes{colorIOTA.String(): 0} // init with IOTa=0
	for color, amount := range confirmedBalance {
		cb[color.String()] = amount
		coins = addToCoins(coins, Coin{
			Color:  color.String(),
			Name:   walletState.AssetRegistry().Name(color),
			Symbol: walletState.AssetRegistry().Symbol(color),
		})
	}
	pb := BalanceRes{}
	for color, amount := range pendingBalance {
		pb[color.String()] = amount
		coins = addToCoins(coins, Coin{
			Color:  color.String(),
			Name:   walletState.AssetRegistry().Name(color),
			Symbol: walletState.AssetRegistry().Symbol(color),
		})
	}

	addys := []AddressRes{}

	receiveAddy := walletState.ReceiveAddress().String()
	for _, addr := range walletState.AddressManager().Addresses() {
		addys = append(addys, AddressRes{
			Address:   addr.String(),
			Index:     addr.Index,
			IsSpent:   walletState.AddressManager().IsAddressSpent(addr.Index),
			IsReceive: addr.String() == receiveAddy,
		})
	}

	hasIOTA := false
	for _, c := range coins {
		if c.Color == colorIOTA.String() {
			hasIOTA = true
		}
	}
	if !hasIOTA {
		coins = addToCoins(coins, Coin{
			Color:  colorIOTA.String(),
			Name:   walletState.AssetRegistry().Name(colorIOTA),
			Symbol: walletState.AssetRegistry().Symbol(colorIOTA),
		})
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]interface{}{
		"confirmed_balance": cb,
		"pending_balance":   pb,
		"coins":             coins,
		"addresses":         addys,
	})
}

func faucet(w http.ResponseWriter, r *http.Request) {
	err := loadWallet()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	err = walletState.RequestFaucetFunds(true)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]bool{
		"success": true,
	})
}

func addToCoins(coins []Coin, coin Coin) []Coin {
	alreadyHas := false
	for _, c := range coins {
		if c.Color == coin.Color {
			alreadyHas = true
		}
	}
	if alreadyHas {
		return coins
	}
	return append(coins, coin)
}
