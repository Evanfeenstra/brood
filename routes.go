package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"sort"
	"strings"

	"github.com/iotaledger/goshimmer/client/wallet"
	walletseed "github.com/iotaledger/goshimmer/client/wallet/packages/seed"
	"github.com/iotaledger/goshimmer/dapps/valuetransfers/packages/address"
	"github.com/iotaledger/goshimmer/dapps/valuetransfers/packages/balance"
	"github.com/iotaledger/hive.go/bitmask"

	"github.com/mr-tron/base58"
	"github.com/shibukawa/configdir"
)

var shimmerURL string

const (
	vendorName = "evanfeenstra"
	appName    = "brood"
	walletPath = "wallet.dat"
)

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

	emptyState := wallet.New(
		wallet.WebAPI(check.URL),
	)
	info, err := emptyState.ServerStatus()
	if err != nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	hasWallet := false
	configDirs := configdir.New(vendorName, appName)
	folder := configDirs.QueryFolderContainsFile(walletPath)
	if folder != nil { // has file
		hasWallet = true
	}
	fmt.Println(folder)

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(InfoRes{
		IdentityID: info.ID,
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

	walletState := wallet.New(
		wallet.WebAPI(check.URL),
		wallet.Import(seed, lastAddressIndex, spentAddresses, assetRegistry),
	)

	err = writeWalletState(walletState)
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
	walletState, err := loadWallet()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

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
	sort.Slice(coins, func(i, j int) bool {
		return strings.Compare(coins[i].Name, coins[j].Name) < 0
	})
	sort.Slice(coins, func(i, j int) bool {
		return coins[i].Name == "IOTA"
	})
	writeWalletState(walletState)

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]interface{}{
		"confirmed_balance": cb,
		"pending_balance":   pb,
		"coins":             coins,
		"addresses":         addys,
	})
}

func faucet(w http.ResponseWriter, r *http.Request) {
	walletState, err := loadWallet()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	err = walletState.RequestFaucetFunds(false)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	writeWalletState(walletState)

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
	coinName := coin.Name
	if coin.Name != "IOTA" && coin.Name == coin.Color {
		coinName = "••••••" // unregistered asset
	}
	return append(coins, Coin{
		Name:   coinName,
		Color:  coin.Color,
		Symbol: coin.Symbol,
	})
}

func send(w http.ResponseWriter, r *http.Request) {
	walletState, err := loadWallet()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	send := Send{}
	body, err := ioutil.ReadAll(r.Body)
	r.Body.Close()
	err = json.Unmarshal(body, &send)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	if len(send.Address) == 0 || len(send.Color) == 0 || send.Amount < 1 {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	destinationAddress, err := address.FromBase58(send.Address)
	if err != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	var color balance.Color
	switch send.Color {
	case "IOTA":
		color = balance.ColorIOTA
	case "NEW":
		color = balance.ColorNew
	default:
		colorBytes, parseErr := base58.Decode(send.Color)
		if parseErr != nil {
			w.WriteHeader(http.StatusNotAcceptable)
			return
		}

		color, _, parseErr = balance.ColorFromBytes(colorBytes)
		if parseErr != nil {
			w.WriteHeader(http.StatusNotAcceptable)
			return
		}
	}

	_, err = walletState.SendFunds(
		wallet.Destination(destinationAddress, send.Amount, color),
	)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	writeWalletState(walletState)

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]bool{
		"success": true,
	})
}

func createCoin(w http.ResponseWriter, r *http.Request) {
	walletState, err := loadWallet()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	coin := Coin{}
	body, err := ioutil.ReadAll(r.Body)
	r.Body.Close()
	err = json.Unmarshal(body, &coin)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	if coin.Name == "" || coin.Symbol == "" || coin.Amount < 1 {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	assetColor, err := walletState.CreateAsset(wallet.Asset{
		Name:   coin.Name,
		Symbol: coin.Symbol,
		Amount: coin.Amount,
	})

	writeWalletState(walletState)

	coinRes := coin
	coinRes.Color = assetColor.String()

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]Coin{
		"coin": coinRes,
	})
}

func registerCoin(w http.ResponseWriter, r *http.Request) {
	walletState, err := loadWallet()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	coin := Coin{}
	body, err := ioutil.ReadAll(r.Body)
	r.Body.Close()
	err = json.Unmarshal(body, &coin)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	if coin.Color == "" || coin.Name == "" || coin.Symbol == "" || coin.Amount != 0 {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	colorBytes, parseErr := base58.Decode(coin.Color)
	if parseErr != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	color, _, parseErr := balance.ColorFromBytes(colorBytes)
	if parseErr != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	walletState.AssetRegistry().RegisterAsset(color, wallet.Asset{
		Color:  color,
		Name:   coin.Name,
		Symbol: coin.Symbol,
	})
	fmt.Println("registered!")

	writeWalletState(walletState)

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]Coin{
		"coin": coin,
	})
}
