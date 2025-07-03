impl crate::Search {
  pub fn test() {
    let mut result = Self::default();
    result = result
      .test_with_pattern()
      .test_with_patterns()
      .test_with_limit();

    println!("{result:#?}")
  }
  pub fn test_with_pattern(mut self) -> Self {
    self = self.with_pattern("pop");
    self = self.with_pattern(String::from("lock"));

    self
  }

  pub fn test_with_patterns(mut self) -> Self {
    self = self.with_patterns(vec!["stop", "lol", "love"]);
    self =
      self.with_patterns(vec!["dress", "rehearsal", &String::from("funny")]);

    self
  }

  fn test_with_limit(mut self) -> Self {
    self = self.with_limit(10);
    // self = self.with_limit(Some(49));
    // self = self.with_limit(None);
    self = self.with_limit("one");
    self = self.with_limit("fiftieth-first");
    self = self.with_limit("forty-seventh");
    self = self.with_limit("twelfth");
    self = self.with_limit("eleven");

    self
  }
}
