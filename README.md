# bad_email

![Docs.rs badge](https://docs.rs/bad_email/badge.svg)

This crate can be used to check email domains against a list of 10234 known [disposable email domains](http://en.wikipedia.org/wiki/Disposable_email_address). <!-- The first line of the file has no domain, so it doesn't count. @SpellignErr 03Mar2024 -->

The crate has one function that returns true or false based on a full email being passed in as a copy of str slice and then splinting the str on '@' and then comparing the domain name to the list of unwanted domain names. 

Returns true for an email address passed in with a domain name that is on the unwanted domain list

Returns false for an email address passed in with a domain name that is not on the unwanted domain list

# Motivation

The creation of this crate was inspired by creating a website they allows users to sign up for email updates and seeing disposable email domains sign up. 

The list of disposable email domains is from the [disposable-email-domains](https://www.npmjs.com/package/disposable-email-domains) npm package. Thank you to the creators and contributors to the package. 


## Usage

To use the crate add it to your project's cargo.toml file:

```
[dependencies]
bad_email = "0.1.0"
```

# Example

Example of how to use remembering email_address variable is str

```
use bad_email::is_email_unwanted;

if is_email_unwanted(email_address) {
  return error message or other functionality
}

```

# Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
