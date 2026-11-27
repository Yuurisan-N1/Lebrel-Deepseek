use crate::step::wallet::Wallet;
use crate::support::device::Device;
use crate::support::log::lg;

pub fn run() -> i32 {
    let wallet = Wallet::generate().unwrap();
    let device = Device::generate();
    let message = format!(
        "lebrel.ai wants you to sign in with your Solana account {}",
        wallet.address
    );
    let signature = wallet.sign(&message);

    lg(&format!(
        "A fresh wallet was built at address {}",
        wallet.address
    ));
    lg(&format!(
        "The device profile reports {} on {}",
        device.platform, device.locale
    ));
    lg(&format!(
        "The signature over the challenge is {} bytes long",
        signature.len()
    ));

    if wallet.address.len() < 32 {
        crate::support::log::lr("The wallet address looks too short to be valid");
        return 1;
    }
    0
}
