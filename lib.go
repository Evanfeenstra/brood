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

func loadWallet() error {
	if walletState != nil {
		return nil // already loaded, good to go
	}
	configDirs := configdir.New(vendorName, appName)
	folder := configDirs.QueryFolderContainsFile(walletPath)
	if folder == nil {
		return errors.New("no file")
	}
	err := reloadWallet(shimmerURL, folder)
	if err != nil {
		return err
	}
	return nil
}
