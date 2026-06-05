use std::collections::HashMap;

use crate::error::AppError;

pub fn normalize_address(address: &str) -> String {
    address.trim().to_lowercase()
}

// pub fn is_valid_eth_address(address: &str) -> bool {
//     let address = address.trim();
//     if !address.starts_with("0x") {
//         return false;
//     }
//     if address.len() != 42 {
//         return false;
//     }

//     address[2..]
//         .chars()
//         .all(|c| c.is_ascii_hexdigit())
// }

//better version
pub fn is_valid_eth_address(address: &str) -> bool {
    let address = address.trim();

    let Some(rest) = address.strip_prefix("0x") else {
        return false;
    };
    
    rest.len() == 40 && rest.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn validate_address(address: &str) -> Result<String, AppError> {
    let normalized_address = normalize_address(address);
    if !is_valid_eth_address(&normalized_address) {
        return Err(AppError::InvalidAddress);
    }
    Ok(normalized_address)
}

pub fn parse_chain_id(input: &str) -> Result<u64, AppError> {
    input.parse::<u64>().map_err(|_| AppError::InvalidChainId)
}

pub fn parse_block_number(input: &str) -> Result<u64, AppError> {
    input.parse::<u64>().map_err(|_| AppError::InvalidBlockNumber)
}
pub fn count_by_address(addresses: Vec<String>) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for address in addresses {
        let normalized_addr = normalize_address(&address);
        *counts.entry(normalized_addr).or_insert(0) += 1;
    }
    counts

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_address() {
        let input = "  0xABCDEFabcdefABCDEFabcdefABCDEFabcdefABCD";

        assert_eq!(normalize_address(input), "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd");
    }

    #[test]
    fn test_is_valid_address() {
        let input = "0x1111111111111111111111111111111111111111";
        assert!(is_valid_eth_address(input));
    }

    #[test]
    fn test_is_invalid_address_without_prefix() {
        let input = "1111111111111111111111111111111111111111";
        assert!(!is_valid_eth_address(input));
    }

    #[test]
    fn test_invalid_eth_address_wrong_length() {
        assert!(!is_valid_eth_address("0x1234"));
    }

    #[test]
    fn test_invalid_eth_address_non_hex() {
        assert!(!is_valid_eth_address(
            "0xzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"
        ));
    }    

    #[test]
    fn test_count_by_address() {
        let addresses = vec![
            "0x1111111111111111111111111111111111111111".to_string(),
            "0x3333333333333333333333333333333333333333".to_string(),
            "0x1111111111111111111111111111111111111111".to_string(),
            "0x2222222222222222222222222222222222222222".to_string(),  
            "0x3333333333333333333333333333333333333333".to_string(),
            "0x3333333333333333333333333333333333333333".to_string(),

        ];
        let counts = count_by_address(addresses);
        assert_eq!(counts.get("0x1111111111111111111111111111111111111111"), Some(&2));
        assert_eq!(counts.get("0x2222222222222222222222222222222222222222"), Some(&1));
        assert_eq!(counts.get("0x3333333333333333333333333333333333333333"), Some(&3));
    }

    #[test]
    fn test_parse_chain_id_ok() {
        let input = "12312";
        assert_eq!(parse_chain_id(input).unwrap(), 12312u64);
    }

    #[test]
    fn test_parse_chain_id_err() {
        let input = "12312a";
        assert!(parse_chain_id(input).is_err());
        assert_eq!(parse_chain_id(input).unwrap_err(), AppError::InvalidChainId);
    }

    #[test]
    fn test_parse_block_number() {
        let input = "15";
        assert_eq!(parse_block_number(input).unwrap(), 15u64);
    }

    #[test]
    fn test_parse_block_number_err() {
        let input = "15a";
        assert!(parse_block_number(input).is_err());
        assert_eq!(parse_block_number(input).unwrap_err(), AppError::InvalidBlockNumber);
    }

}
    
