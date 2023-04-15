module DebugDemo::Message {
    use std::string;
    use std::signer;
    use aptos_std::debug;
debug::print
    struct MessageHolder has key {
        message: string::String,
    }
debug::print_stack_trace

    public entry fun set_message(account: signer, message_bytes: vector<u8>)
    acquires MessageHolder {
        debug::print_stack_trace();
        let message = string::utf8(message_bytes);
        let account_addr = signer::address_of(&account);
        if (!exists<MessageHolder>(account_addr)) {
            move_to(&account, MessageHolder {
                message,
            })
        } else {
            let old_message_holder = borrow_global_mut<MessageHolder>(account_addr);
            old_message_holder.message = message;
        }
    }
$ aptos move test --package-dir crates/aptos/debug-move-example
    #[test(account = @0x1)]
    public entry fun sender_can_set_message(account: signer) acquires MessageHolder {
        let addr = signer::address_of(&account);
        debug::print<address>(&addr);
        set_message(account,  b"Hello, Blockchain");
    }
}
Running Move unit tests
[debug] 0000000000000000000000000000000000000000000000000000000000000001
Call Stack:
    [0] 0000000000000000000000000000000000000000000000000000000000000001::Message::sender_can_set_message

        Code:
            [4] CallGeneric(0)
            [5] MoveLoc(0)
            [6] LdConst(0)
          > [7] Call(1)
            [8] Ret

        Locals:
            [0] -
            [1] 0000000000000000000000000000000000000000000000000000000000000001


Operand Stack:
