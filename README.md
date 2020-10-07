# BuildlinksRedbag4Near contract

BuildlinksRedbag4Near contract allows any user to create a redbag that their online group friends can grab to claim tokens even if they don't have an account yet.

The way it works:

Sender, that has NEAR:
- Creates a new key pair `(pk1, privkey1)`.
- Calls `redbag.send_redbag(pk1, count, mode, slogan)` with attached balance of NEAR that they want to wrap into the redbag.
- Sends a redbag link to any online group with `privkey1` behind the link.

note: `count` indicates the number of receivers that can share the redbag; `mode` decides a random share with value 1 or an even share with value 0; `slogan` is a blessing words that would be the caption of that redbag.

Receiver, that doesn't have NEAR:
- Receives link to the wallet with `privkey1`.
- Wallet creates new key pair for this user (or they generate it via HSM) `(pk2, privkey2)`.
- Enters the `new_account_id` receiver want for their new account.
- Wallet creates a transaction to `redbag.create_account_and_claim(new_account_id, pk2)`.
- Contract creates new account with `new_account_id` name and `pk2` as full access key and transfers his share of the redbag.

If Receiver already has account:
- Sign tx with `privkey1` to call `redbag.claim()`, which transfers his share of the redbag to signer's account.

If Sender wants to revoke the redbag):
- Sign tx with `privkey1` to call `redbag.revoke()`, which transfers remaining money back to sender's account.

