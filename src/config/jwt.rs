pub fn init() -> String {
  dotenv::dotenv().ok();
  std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}