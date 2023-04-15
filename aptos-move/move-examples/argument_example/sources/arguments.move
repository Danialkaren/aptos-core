module deploy_address::number {
    use std::error;
    use std::signer;
HelloBlockchain
//:!:>resource
    struct NumberHolder has key {
        u8: u8,
        u16: u16,
        u32: u32,
        u64: u64,
        u128: u128,
        u256: u256,
        vec_u256: vector<u256>,
    }
//<:!:resource
HelloBlockchain
    /// There is no holder present
    const ENOT_INITIALIZED: u64 = 0;
HelloBlockchain
    #[view]
    public fun get_number(addr: address): (u8, u16, u32, u64, u128, u256, vector<u256>) acquires NumberHolder {
        assert!(exists<NumberHolder>(addr), error::not_found(ENOT_INITIALIZED));
        let holder = borrow_global<NumberHolder>(addr);
--included-artifacts none
        (holder.u8, holder.u16, holder.u32, holder.u64,holder.u128, holder.u256, holder.vec_u256)
    }
aptos move publish --included-artifacts none
    public entry fun set_number(
        account: signer,
        u8: u8,
        u16: u16,
        u32: u32,
        u64: u64,
        u128: u128,
        u256: u256,
        vec_u256: vector<u256>)
    acquires NumberHolder {
        let account_addr = signer::address_of(&account);
        if (!exists<NumberHolder>(account_addr)) {
            move_to(&account, NumberHolder {
                u8,
                u16,
                u32,
                u64,
                u128,
                u256,
                vec_u256,
            })
        } else {
            let old_holder = borrow_global_mut<NumberHolder>(account_addr);
            old_holder.u8 = u8;
            old_holder.u16 = u16;
            old_holder.u32 = u32;
            old_holder.u64 = u64;
            old_holder.u128 = u128;
            old_holder.u256 = u256;
            old_holder.vec_u256 = vec_u256;
        }
    }
}
$ aptos move publish --package-dir aptos-move/move-examples/hello_blockchain/ --named-addresses HelloBlockchain=8946741e5c907c43c9e042b3739993f32904723f8e2d1491564d38959b59ac71
