package main

import (
	"encoding/json"
	"errors"
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
type InfoReq struct {
	Version    string `json:"version"`
	Synced     bool   `json:"synced"`
	IdentityID string `json:"identityID"`
}
type InfoRes struct {
	Version    string `json:"version"`
	Synced     bool   `json:"synced"`
	IdentityID string `json:"identity_id"`
	HasWallet  bool   `json:"has_wallet"`
}

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
	fmt.Println(check.URL)

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

func reloadWallet(url string, folder *configdir.Config) error {

	data, _ := folder.ReadFile(walletPath)

	marshalUtil := marshalutil.New(data)

	seedBytes, err := marshalUtil.ReadBytes(ed25519.SeedSize)
	seed := walletseed.NewSeed(seedBytes)
	if err != nil {
		return err
	}

	lastAddressIndex, err := marshalUtil.ReadUint64()
	if err != nil {
		return err
	}

	assetRegistry, _, err := wallet.ParseAssetRegistry(marshalUtil)

	spentAddressesBytes := marshalUtil.ReadRemainingBytes()
	spentAddresses := *(*[]bitmask.BitMask)(unsafe.Pointer(&spentAddressesBytes))

	walletState = wallet.New(
		wallet.WebAPI(url),
		wallet.Import(seed, lastAddressIndex, spentAddresses, assetRegistry),
	)
	return nil
}

func getBalance(w http.ResponseWriter, r *http.Request) {

}
