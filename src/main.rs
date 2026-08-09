fn main() {
    if let Err(err) = ofac_sanctioned_digital_currency_addresses::run_from_env() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
