#[derive(Deserialize, Debug)]
pub struct Request {
  pub timestamp: String,
  // time_elapsed: u8,
  // client_ip: String,
  // client_continent: String,
  // client_country: String,
  // client_region: String,
  // client_city: String,
  // client_latitude: String,
  // client_longitude: String,
  // pub client_timezone: String,
  // client_connection: String,
  // request: String,
  // request_host: String,
  pub request_path: String,
  pub request_query: String,
  // request_bytes: u16,
  pub user_agent: String,
  pub http2: bool,
  // pub tls: Option<bool>,
  pub tls_version: String,
  pub tls_cipher: String,
  // response_status: String,
  // response_text: String,
  // response_bytes: u16,
  // response_cache: String,
  // cache_state: String,
  // cache_lastuse: f32,
  // cache_hits: u16,
  pub server_region: String,
  // server_datacenter: String,
}
