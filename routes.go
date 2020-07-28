package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"unsafe"

	"github.com/iotaledger/goshimmer/client/wallet"
	walletseed "github.com/iotaledger/goshimmer/client/wallet/packages/seed"
	"github.com/iotaledger/hive.go/bitmask"
	"github.com/iotaledger/hive.go/crypto/ed25519"
	"github.com/iotaledger/hive.go/marshalutil"

	"github.com/mr-tron/base58"
	"github.com/shibukawa/configdir"
)

var walletState *wallet.Wallet

const (
	vendorName = "evanfeenstra"
	appName    = "brood"
	walletPath = "wallet.dat"
)

type Check struct {
	URL string `json:"url"`
}
type InfoRes struct {
	Version string `json:"version"`
	Synced  bool   `json:"synced"`
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
	fmt.Println(check.URL)

	req, err := http.Get(check.URL + "/info")
	if err != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}
	defer req.Body.Close()

	info := InfoRes{}
	res, err := ioutil.ReadAll(req.Body)
	err = json.Unmarshal(res, &info)
	if err != nil {
		w.WriteHeader(http.StatusNotAcceptable)
		return
	}

	// configDirs := configdir.New(vendorName, appName)
	// folder := configDirs.QueryFolderContainsFile(walletPath)
	// if folder == nil { // no file
	// 	w.WriteHeader(http.StatusBadRequest)
	// 	return
	// }
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(info)
}

func createWallet(w http.ResponseWriter, r *http.Request) {
	configDirs := configdir.New(vendorName, appName)

	folder := configDirs.QueryFolderContainsFile(walletPath)
	if folder == nil { // no file
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	data, _ := folder.ReadFile(walletPath)

	marshalUtil := marshalutil.New(data)

	seedBytes, err := marshalUtil.ReadBytes(ed25519.SeedSize)
	seed := walletseed.NewSeed(seedBytes)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	lastAddressIndex, err := marshalUtil.ReadUint64()
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	assetRegistry, _, err := wallet.ParseAssetRegistry(marshalUtil)

	spentAddressesBytes := marshalUtil.ReadRemainingBytes()
	spentAddresses := *(*[]bitmask.BitMask)(unsafe.Pointer(&spentAddressesBytes))

	walletState = wallet.New(
		wallet.WebAPI(config.WebAPI),
		wallet.Import(seed, lastAddressIndex, spentAddresses, assetRegistry),
	)

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(base58.Encode(seed.Bytes()))
}

func getBalance(w http.ResponseWriter, r *http.Request) {

}
