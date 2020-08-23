package main

import (
	"errors"
	"unsafe"

	"github.com/iotaledger/goshimmer/client/wallet"
	walletseed "github.com/iotaledger/goshimmer/client/wallet/packages/seed"

	"github.com/iotaledger/hive.go/bitmask"
	"github.com/iotaledger/hive.go/crypto/ed25519"
	"github.com/iotaledger/hive.go/marshalutil"

	"github.com/shibukawa/configdir"
)

const (
	vendorName = "brood"
	appName    = "wallet"
	walletPath = "state.dat"
)

// cd "/Users/evanfeenstra/Library/Application Support/brood/wallet"

func reloadWalletFromFile(url string, folder *configdir.Config) (*wallet.Wallet, error) {

	data, _ := folder.ReadFile(walletPath)

	marshalUtil := marshalutil.New(data)

	seedBytes, err := marshalUtil.ReadBytes(ed25519.SeedSize)
	seed := walletseed.NewSeed(seedBytes)
	if err != nil {
		return nil, err
	}

	lastAddressIndex, err := marshalUtil.ReadUint64()
	// fmt.Println("lastAddressIndex", lastAddressIndex)
	if err != nil {
		return nil, err
	}

	assetRegistry, _, err := wallet.ParseAssetRegistry(marshalUtil)

	spentAddressesBytes := marshalUtil.ReadRemainingBytes()
	spentAddresses := *(*[]bitmask.BitMask)(unsafe.Pointer(&spentAddressesBytes))

	walletState := wallet.New(
		wallet.WebAPI(url),
		wallet.Import(seed, lastAddressIndex, spentAddresses, assetRegistry),
	)
	return walletState, nil
}

func loadWallet() (*wallet.Wallet, error) {
	if len(shimmerURL) == 0 {
		return nil, errors.New("no url")
	}
	configDirs := configdir.New(vendorName, appName)
	folder := configDirs.QueryFolderContainsFile(walletPath)
	if folder == nil {
		return nil, errors.New("no file")
	}
	walletState, err := reloadWalletFromFile(shimmerURL, folder)
	if err != nil {
		return nil, err
	}
	// if err := walletState.Refresh(); err != nil {
	// 	return err
	// }
	return walletState, nil
}

// run this after every update to state
func writeWalletState(walletState *wallet.Wallet) error {
	configDirs := configdir.New(vendorName, appName)
	folders := configDirs.QueryFolders(configdir.Global)
	if len(folders) == 0 {
		return errors.New("no file")
	}
	// fmt.Println("WRITE TO", folders[0])
	err := folders[0].WriteFile(walletPath, walletState.ExportState())
	return err
}
