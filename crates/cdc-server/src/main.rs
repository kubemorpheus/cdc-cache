fn main() {
    println!("cdc-core: {}", cdc_core::greeting());
    println!("cdc-storage: {}", cdc_storage::greeting());
    println!("cdc-ingest: {}", cdc_ingest::greeting());
}
