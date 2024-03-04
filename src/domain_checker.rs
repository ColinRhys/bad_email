use crate::config::Config;

// Doc Comment Section - Start

/// Checks if the domain of an email address is on the list of unwanted domains.
///
/// # Arguments
///
/// * `email_address` - A string slice that holds the email address to check.
///
/// # Returns
///
/// Returns `true` if the domain of the email address is on the unwanted list, and `false` otherwise.
///
/// # Examples
///
/// This example shows an email address with a domain that is considered unwanted, thus returning `true`:
/// ```
/// # use bad_email::is_email_unwanted;
/// let email = "user_name@mostlysunny.com";
/// assert!(is_email_unwanted(email)); // "mostlysunny.com" is in the unwanted list
/// ```
///
/// This example shows an email address with a domain that is not considered unwanted, thus returning `false`:
/// ```
/// # use bad_email::is_email_unwanted;
/// let email = "user_name@yahoo.com";
/// assert!(!is_email_unwanted(email)); // "yahoo.com" is not in the unwanted list
/// ```

// Doc Comment Section - End

// Functions - Start

pub fn is_email_unwanted(email_address: &str) -> bool {
  if !email_address.contains("@") || email_address.len() < 3 {
    return false;
  }
  let config = Config::load_default();
  match extract_domain(email_address) {
    Some(domain) => config.unwanted_domains.contains(&domain.to_string()),
    None => false,
  }
}

pub fn extract_domain(email_address: &str) -> Option<&str> {
  // Remove trailing dot from email address domain. Domains all end in a literal dot (period) character
  // therefore john@example.com is identical to john@example.com.
  // This new code will remove te trailing dot. See RFC1034 and subsequent updates for more info
  // https://datatracker.ietf.org/doc/html/rfc1034
  //
  // @SpellignErr 03Mar2024
  email_address.strip_suffix(".").unwrap_or(email_address).split('@').nth(1)
}

// Functions - End


// Tests
#[cfg(test)]
mod test {
use super::{is_email_unwanted, extract_domain};
  // Test is email unwanted - TRUE
  #[test]
  fn is_email_unwanted_true() {
    let email = "user_name@mostlysunny.com";
    assert!(is_email_unwanted(email))
  }

  // Test is email unwanted - FALSE
  #[test]
  fn is_email_unwanted_false() {
    let email = "user_name@yahoo.com";
    assert!(!is_email_unwanted(email))
  }

  // Test is email domain contains unwanted_domain but is not equal to a unwanted_domain name
  #[test]
  fn is_email_unwanted_contains_not_equal() {
    let email = "user_name@mostlysunny_and_this.com";
    assert!(!is_email_unwanted(email))
  }

  // email_address without "@" returns false
  #[test]
  fn is_email_unwanted_domain_no_split_symbol() {
    let email = "user_nameyahoo.com";
    assert!(!is_email_unwanted(email))
  }

    // email_address is ""
    #[test]
    fn is_email_unwanted_blank_email_address() {
      let email = "";
      assert!(!is_email_unwanted(email))
    }

  // extract domain - works like a charm
  #[test]
  fn extract_domain_extracts_domain() {
    let email = "user_name@yahoo.com";
    assert_eq!(extract_domain(email), Some("yahoo.com"))
  }

  // extract domain - strips trailing "."
  #[test]
  fn extract_domain_strips_trailing_period() {
    let email = "user_name@mostlysunny.com.";
    assert_eq!(extract_domain(email), Some("mostlysunny.com"))
  }
}
