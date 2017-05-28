#[macro_use]
extern crate pest;

use pest::prelude::*;

impl_rdp! {
    grammar! {
        query      = { ["query"]? ~ query_name? ~ brace ~ fragment* }
        query_name = { word ~ def? }
        brace      = { ["{"] ~ fields ~ ["}"] }
        fragment   = { ["fragment"] ~ word ~ ["on"] ~ word ~ brace }
        fields     = { reference | field* }
        reference  = { ["..."] ~ word }
        field      = { word ~ alias? ~ arguments? ~ brace? }
        alias      = { [":"] ~ word }
        def        = { ["("] ~ variable ~ [":"] ~ model ~ [")"] }
        arguments  = { ["("] ~ parameter ~ [":"] ~ value ~ [")"] }
        model      = { word }
        parameter  = { word }
        value      = { string | variable }
        string     = { ["\""]? ~ (!(["\""] | [")"]) ~ any)*  ~ ["\""]? }
        variable   = @{ ["$"] ~ ['A'..'z']+ }
        word       = @{ ['A'..'z']+ }
        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"]+ }
    }
}

/// Parse GraphQL query and return a vector of tokens
pub fn parse(query: &str) -> Vec<pest::Token<Rule>> {
    let mut parser = Rdp::new(StringInput::new(query));
    assert!(parser.query());
    assert!(parser.end());
    parser.queue().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// http://graphql.org/learn/queries/#fields
    #[test]
    fn test_parse_with_fields() {
        parse(r#"{
  hero {
    name
  }
}"#);
    }
    
    /// http://graphql.org/learn/queries/#arguments
    #[test]
    fn test_parse_with_arguments() {
        parse(r#"{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}"#);
    }
     
    /// http://graphql.org/learn/queries/#aliases
    #[test]
    fn test_parse_with_aliases() {
        parse(r#"{
  empireHero: hero(episode: EMPIRE) {
    name
  }
  jediHero: hero(episode: JEDI) {
    name
  }
}"#);
    } 

    /// http://graphql.org/learn/queries/#fragments
    #[test]
    fn test_parse_with_fragments() {
        parse(r#"{
  leftComparison: hero(episode: EMPIRE) {
    ...comparisonFields
  }
  rightComparison: hero(episode: JEDI) {
    ...comparisonFields
  }
}

fragment comparisonFields on Character {
  name
  appearsIn
  friends {
    name
  }
}"#);
    }
    
    /// http://graphql.org/learn/queries/#variables
    #[test]
    fn test_parse_with_variables() {
        parse(r#"query HeroNameAndFriends($episode: Episode) {
  hero(episode: $episode) {
    name
    friends {
      name
    }
  }
}"#);
    }
}
