package wallet

type Check struct {
	URL string `json:"url"`
}
type Send struct {
	Address string `json:"address"`
	Color   string `json:"color"`
	Amount  uint64 `json:"amount"`
}
type Coin struct {
	Color  string `json:"color"`
	Name   string `json:"name"`
	Symbol string `json:"symbol"`
	Amount uint64 `json:"amount"`
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
type AddressRes struct {
	Address   string `json:"address"`
	Index     uint64 `json:"index"`
	IsSpent   bool   `json:"is_spent"`
	IsReceive bool   `json:"is_receive"`
}
type BalanceRes map[string]uint64
