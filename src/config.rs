use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub unwanted_domains: Vec<String>,
}

impl Config {
  pub fn load_default() -> Self {
    let contents = include_str!("default_email_list.toml");
    toml::from_str(contents).expect("Error loading the default_email_list.toml file")
  }
}

// Config load_default works
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config_load_default() {
    let config = Config::load_default();

    // ensure unwanted_domains is not empty
    assert!(!config.unwanted_domains.is_empty());

    // check domain names from default_email_list.toml are loaded
    let expected_domains = vec!["quackquack.com", "throwawayemailaddress.com"];

    for domain in expected_domains {
        assert!(config.unwanted_domains.contains(&domain.to_string()));
    }

    // ensure 10,224 domains in list

    assert_eq!(config.unwanted_domains.len(), 10234);
  }
}