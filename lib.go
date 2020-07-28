package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"unsafe"

	"github.com/mr-tron/base58"

	"github.com/iotaledger/goshimmer/client/wallet"
	walletseed "github.com/iotaledger/goshimmer/client/wallet/packages/seed"

	"github.com/iotaledger/hive.go/bitmask"
	"github.com/iotaledger/hive.go/crypto/ed25519"
	"github.com/iotaledger/hive.go/marshalutil"
)

func loadWallet() *wallet.Wallet {
	seed, lastAddressIndex, spentAddresses, assetRegistry, err := importWalletStateFile("wallet.dat")
	if err != nil {
		panic(err)
	}

	return wallet.New(
		wallet.WebAPI(config.WebAPI),
		wallet.Import(seed, lastAddressIndex, spentAddresses, assetRegistry),
	)
}

func importWalletStateFile(filename string) (seed *walletseed.Seed, lastAddressIndex uint64, spentAddresses []bitmask.BitMask, assetRegistry *wallet.AssetRegistry, err error) {
	walletStateBytes, err := ioutil.ReadFile(filename)
	if err != nil {
		if !os.IsNotExist(err) {
			return
		}

		seed = walletseed.NewSeed()
		lastAddressIndex = 0
		spentAddresses = []bitmask.BitMask{}
		err = nil

		fmt.Println("GENERATING NEW WALLET ...                                 [DONE]")
		fmt.Println()
		fmt.Println("================================================================")
		fmt.Println("!!!            PLEASE CREATE A BACKUP OF YOUR SEED           !!!")
		fmt.Println("!!!                                                          !!!")
		fmt.Println("!!!       " + base58.Encode(seed.Bytes()) + "       !!!")
		fmt.Println("!!!                                                          !!!")
		fmt.Println("!!!            PLEASE CREATE A BACKUP OF YOUR SEED           !!!")
		fmt.Println("================================================================")

		return
	}

	marshalUtil := marshalutil.New(walletStateBytes)

	seedBytes, err := marshalUtil.ReadBytes(ed25519.SeedSize)
	seed = walletseed.NewSeed(seedBytes)
	if err != nil {
		return
	}

	lastAddressIndex, err = marshalUtil.ReadUint64()
	if err != nil {
		return
	}

	assetRegistry, _, err = wallet.ParseAssetRegistry(marshalUtil)

	spentAddressesBytes := marshalUtil.ReadRemainingBytes()
	spentAddresses = *(*[]bitmask.BitMask)(unsafe.Pointer(&spentAddressesBytes))

	return
}

func writeWalletStateFile(wallet *wallet.Wallet, filename string) {
	var skipRename bool
	info, err := os.Stat(filename)
	if err != nil {
		if !os.IsNotExist(err) {
			panic(err)
		}

		skipRename = true
	}
	if err == nil && info.IsDir() {
		panic("found directory instead of file at " + filename)
	}

	if !skipRename {
		err = os.Rename(filename, filename+".bkp")
		if err != nil && os.IsNotExist(err) {
			panic(err)
		}
	}

	err = ioutil.WriteFile(filename, wallet.ExportState(), 0644)
	if err != nil {
		panic(err)
	}
}
