fn main() {
  let raw_key = [249,71,148,217,221,59,200,212,240,137,46,204,100,158,22,129,239,213,34,228,219,84,27,142,165,146,177,195,172,153,113,61,33,215,71,137,33,34,63,92,244,211,34,36,221,122,99,244,69,138,198,244,69,0,27,215,226,233,234,42,148,107,199,219];
  
  // Split into private and public keys
  let private_key = &raw_key[..32];  // First 32 bytes
  let public_key = &raw_key[32..];   // Last 32 bytes
  
  // Print both keys as hex strings
  println!("Private key (hex): {}", private_key.iter()
      .map(|b| format!("{:02x}", b))
      .collect::<String>());
      
  println!("Public key (hex): {}", public_key.iter()
      .map(|b| format!("{:02x}", b))
      .collect::<String>());
}