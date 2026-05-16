use crate::errors::*;
use crate::path::ClarPath;
use std::collections::HashMap;
use std::fmt::Write;

/// Command-line option.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClarOption {
  /// Option name.
  name: String,
  /// Short label.
  short_label: Option<char>,
  /// Long label.
  long_label: Option<String>,
  /// Indicates whether this option must be used alone.
  standalone: bool,
  /// Indicates whether this option takes a value.
  takes_value: Option<String>,
  /// Default value used when the option is not specified.
  default_value: Option<String>,
  /// Default value used when the option is provided without value.
  default_missing_value: Option<String>,
  /// List of possible values for this option.
  possible_values: Vec<String>,
  /// Maximum number of option occurrences.
  max_occurrences: Option<usize>,
  /// Help content.
  help: Option<String>,
  /// Long help content.
  help_long: Option<String>,
  /// Path in the definition tree.
  path: ClarPath,
}

impl ClarOption {
  /// Creates a new command-line option with the given name and labels.
  pub fn new(name: impl AsRef<str>, short: char, long: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      short_label: Some(short),
      long_label: Some(long.as_ref().to_string()),
      ..Default::default()
    }
  }

  pub fn short(name: impl AsRef<str>, label: char) -> Self {
    Self {
      name: name.as_ref().to_string(),
      short_label: Some(label),
      ..Default::default()
    }
  }

  pub fn long(name: impl AsRef<str>, label: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      long_label: Some(label.as_ref().to_string()),
      ..Default::default()
    }
  }

  pub fn short_label(mut self, label: char) -> Self {
    self.short_label = Some(label);
    self
  }

  pub fn long_label(mut self, label: impl AsRef<str>) -> Self {
    self.long_label = Some(label.as_ref().to_string());
    self
  }

  pub fn standalone(mut self) -> Self {
    self.standalone = true;
    self
  }

  pub fn default_value(mut self, value: impl AsRef<str>) -> Self {
    self.default_value = Some(value.as_ref().to_string());
    self
  }

  pub fn default_missing_value(mut self, value: impl AsRef<str>) -> Self {
    self.default_missing_value = Some(value.as_ref().to_string());
    self
  }

  pub fn possible_values<I, S>(mut self, values: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    self.possible_values = values.into_iter().map(|value| value.as_ref().to_owned()).collect();
    self
  }

  pub fn max_occurrences(mut self, max_occurrences: usize) -> Self {
    self.max_occurrences = Some(max_occurrences);
    self
  }

  pub fn takes_value(mut self, caption: impl AsRef<str>) -> Self {
    self.takes_value = Some(caption.as_ref().to_string());
    self
  }

  pub fn help(mut self, content: impl AsRef<str>) -> Self {
    self.help = Some(content.as_ref().to_string());
    self.help_long = Some(content.as_ref().to_string());
    self
  }

  pub fn help_long(mut self, content: impl AsRef<str>) -> Self {
    self.help_long = Some(content.as_ref().to_string());
    self
  }

  pub(crate) fn get_short_label(&self) -> &Option<char> {
    &self.short_label
  }

  pub fn get_long_label(&self) -> &Option<String> {
    &self.long_label
  }

  pub(crate) fn get_synopsis(&self) -> String {
    let mut label = String::new();
    if let Some(short) = self.get_short_label() {
      _ = write!(&mut label, "-{}", short);
    }
    if let Some(long) = self.get_long_label() {
      label.clear();
      _ = write!(&mut label, "--{}", long);
    }
    if let Some(value_caption) = &self.takes_value {
      if self.default_missing_value.is_some() {
        _ = write!(&mut label, " [<{}>]", value_caption);
      } else {
        _ = write!(&mut label, " <{}>", value_caption);
      }
    }
    label
  }

  pub fn is_standalone(&self) -> bool {
    self.standalone
  }

  pub fn get_default_value(&self) -> &Option<String> {
    &self.default_value
  }

  pub fn get_default_missing_value(&self) -> &Option<String> {
    &self.default_missing_value
  }

  pub fn get_takes_value(&self) -> &Option<String> {
    &self.takes_value
  }

  pub fn get_possible_values(&self) -> &Vec<String> {
    &self.possible_values
  }

  pub fn get_max_occurrences(&self) -> usize {
    self.max_occurrences.unwrap_or(1)
  }

  pub fn get_help(&self) -> &Option<String> {
    &self.help
  }

  pub fn get_help_long(&self) -> &Option<String> {
    &self.help_long
  }

  fn set_path(&mut self, parent: &ClarPath) {
    self.path = parent.join(&self.name);
  }

  pub(crate) fn get_path(&self) -> &ClarPath {
    &self.path
  }

  pub(crate) fn validate(&self) -> ClarResult<()> {
    if let Some(label) = self.get_short_label() {
      Self::validate_short_label(*label)?;
    }
    if let Some(label) = self.get_long_label() {
      Self::validate_long_label(label)?;
    }
    if self.takes_value.is_none() {
      // Default value MAY be set only when the option is not a flag.
      if self.default_value.is_some() {
        return Err(err_default_value_for_flag_option(self.get_synopsis()));
      }
      // Default missing value MAY be set only when the option is not a flag.
      if self.default_missing_value.is_some() {
        return Err(err_default_missing_value_for_flag_option(self.get_synopsis()));
      }
      // Possible values MAY be used only when the option is not a flag.
      if !self.possible_values.is_empty() {
        return Err(err_possible_values_for_flag_option(self.get_synopsis()));
      }
    }
    if self.takes_value.is_some() && !self.possible_values.is_empty() {
      if let Some(default_value) = &self.default_value
        && !self.possible_values.contains(default_value)
      {
        return Err(err_invalid_default_value_for_option(
          self.get_synopsis(),
          default_value.to_owned(),
          self.possible_values.to_owned(),
        ));
      }
      if let Some(default_missing_value) = &self.default_missing_value
        && !self.possible_values.contains(default_missing_value)
      {
        return Err(err_invalid_default_missing_value_for_option(
          self.get_synopsis(),
          default_missing_value.to_owned(),
          self.possible_values.to_owned(),
        ));
      }
    }
    Ok(())
  }

  pub(crate) fn validate_long_label(label: &str) -> ClarResult<()> {
    for (index, ch) in label.chars().enumerate() {
      if index == 0 {
        if !ch.is_ascii_alphabetic() {
          return Err(err_long_option_must_start_with_letter(label.to_string()));
        }
      } else {
        if !(ch.is_ascii_alphanumeric() || ch == '-') {
          return Err(err_long_option_must_contain_letters_digits_hyphens(label.to_string()));
        }
      }
    }
    Ok(())
  }

  pub(crate) fn validate_short_label(ch: char) -> ClarResult<()> {
    if !ch.is_ascii_alphanumeric() {
      return Err(err_short_option_must_be_letter_or_digit(ch));
    }
    Ok(())
  }
}

/// Command-line argument.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClarArgument {
  /// Argument name.
  name: String,
  /// Caption is displayed in usage instead of the name.
  caption: String,
  /// Default value used when the argument is not specified.
  default_value: Option<String>,
  /// Indicates whether this argument is required.
  required: bool,
  /// Help content.
  help: Option<String>,
  /// Long help content.
  help_long: Option<String>,
  /// Path in the definition tree.
  path: ClarPath,
}

impl ClarArgument {
  pub fn new(name: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      caption: name.as_ref().to_uppercase(),
      ..Default::default()
    }
  }

  pub fn caption(mut self, caption: impl AsRef<str>) -> Self {
    self.caption = caption.as_ref().to_string();
    self
  }

  pub fn default_value(mut self, default_value: impl AsRef<str>) -> Self {
    self.default_value = Some(default_value.as_ref().to_string());
    self
  }

  pub fn required(mut self) -> Self {
    self.required = true;
    self
  }

  pub fn help(mut self, help: impl AsRef<str>) -> Self {
    self.help = Some(help.as_ref().to_string());
    self
  }

  pub fn help_long(mut self, help_long: impl AsRef<str>) -> Self {
    self.help_long = Some(help_long.as_ref().to_string());
    self
  }

  pub fn get_caption(&self) -> &str {
    &self.caption
  }

  pub fn get_default_value(&self) -> &Option<String> {
    &self.default_value
  }

  pub fn is_required(&self) -> bool {
    self.required
  }

  pub fn get_help(&self) -> &Option<String> {
    &self.help
  }

  pub fn get_help_long(&self) -> &Option<String> {
    if self.help_long.is_some() { &self.help_long } else { self.get_help() }
  }

  fn set_path(&mut self, parent: &ClarPath) {
    self.path = parent.join(&self.name);
  }

  pub(crate) fn get_path(&self) -> &ClarPath {
    &self.path
  }

  pub(crate) fn validate(&self) -> ClarResult<()> {
    // When the argument is required, the default value MUST NOT be set.
    if self.required && self.default_value.is_some() {
      return Err(err_default_value_for_required_argument(self.name.clone()));
    }
    Ok(())
  }
}

/// Command-line command or subcommand.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClarCommand {
  /// Command name.
  name: String,
  /// Command-line definition for this command.
  definition: ClarDefinition,
  /// Help content.
  help: Option<String>,
  /// Long help content.
  help_long: Option<String>,
  /// Path in the definition tree.
  path: ClarPath,
}

impl ClarCommand {
  /// Creates a new command with the given name.
  pub fn new(name: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      ..Default::default()
    }
  }

  pub fn help(mut self, help: impl AsRef<str>) -> Self {
    self.help = Some(help.as_ref().to_string());
    self
  }

  pub fn help_long(mut self, help_long: impl AsRef<str>) -> Self {
    self.help_long = Some(help_long.as_ref().to_string());
    self
  }

  /// Returns a resolver that recognizes an option terminator.
  pub fn terminator(mut self, option_terminator: ClarTerminator) -> Self {
    self.definition.terminator(option_terminator);
    self
  }

  /// Returns a resolver that recognizes options.
  pub fn options<O>(mut self, options: O) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.definition.options(options);
    self
  }

  /// Returns a resolver that recognizes options followed by option terminator.
  pub fn options_terminator<O>(mut self, options: O, option_terminator: ClarTerminator) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.definition.options_terminator(options, option_terminator);
    self
  }

  /// Returns a resolver that recognizes arguments.
  pub fn arguments<A>(mut self, arguments: A) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.arguments(arguments);
    self
  }

  /// Returns a resolver that recognizes arguments followed by option terminator.
  pub fn arguments_terminator<A>(mut self, arguments: A, option_terminator: ClarTerminator) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.arguments_terminator(arguments, option_terminator);
    self
  }

  /// Returns a resolver that recognizes options followed by arguments.
  pub fn options_arguments<O, A>(mut self, options: O, arguments: A) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.options_arguments(options, arguments);
    self
  }

  /// Returns a resolver that recognizes options followed by arguments and option terminator.
  pub fn options_arguments_terminator<O, A>(mut self, options: O, arguments: A, option_terminator: ClarTerminator) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.definition.options_arguments_terminator(options, arguments, option_terminator);
    self
  }

  /// Returns a resolver that recognizes commands.
  pub fn commands<C>(mut self, commands: C) -> Self
  where
    C: IntoIterator<Item = ClarCommand>,
  {
    self.definition.commands(commands);
    self
  }

  /// Returns a resolver that recognizes options followed by commands.
  pub fn options_commands<O, C>(mut self, options: O, commands: C) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    C: IntoIterator<Item = ClarCommand>,
  {
    self.definition.options_commands(options, commands);
    self
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub(crate) fn get_definition(&self) -> &ClarDefinition {
    &self.definition
  }

  pub fn get_help(&self) -> &Option<String> {
    &self.help
  }

  pub fn get_help_long(&self) -> &Option<String> {
    if self.help_long.is_some() { &self.help_long } else { self.get_help() }
  }

  fn set_path(&mut self, parent: &ClarPath) {
    self.path = parent.join(&self.name);
  }

  pub(crate) fn get_path(&self) -> &ClarPath {
    &self.path
  }

  pub(crate) fn validate(&self) -> ClarResult<()> {
    self.validate_name()?;
    Ok(())
  }

  fn validate_name(&self) -> ClarResult<()> {
    for (index, ch) in self.name.chars().enumerate() {
      if index == 0 {
        if !self.is_valid_name_start(ch) {
          return Err(err_command_must_start_with_letter(self.name.to_string()));
        }
      } else {
        if !self.is_valid_name_char(ch) {
          return Err(err_command_must_contain_letters_digits_hyphens(self.name.to_string()));
        }
      }
    }
    Ok(())
  }

  fn is_valid_name_start(&self, ch: char) -> bool {
    ch.is_ascii_alphabetic()
  }

  fn is_valid_name_char(&self, ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-'
  }
}

/// Command-line option terminator.
///
/// When present in the argument list, everything after it is treated
/// as a positional argument, even if it looks like an option.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClarTerminator {
  /// Terminator name.
  pub name: String,
  /// Indicated whether this option terminator is required.
  required: bool,
  /// Path in the definition tree.
  path: ClarPath,
}

impl ClarTerminator {
  /// Creates a new option terminator with the given name.
  pub fn new(name: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      ..Default::default()
    }
  }

  /// Marks this option terminator as required.
  ///
  /// If the option terminator is not present in the argument list, resolving will fail.
  pub fn required(mut self) -> Self {
    self.required = true;
    self
  }

  pub fn is_required(&self) -> bool {
    self.required
  }

  fn set_path(&mut self, parent: &ClarPath) {
    self.path = parent.join(&self.name);
  }

  pub(crate) fn get_path(&self) -> &ClarPath {
    &self.path
  }
}

/// Command-line item enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClarItem {
  Options(Vec<ClarOption>),
  Commands(Vec<ClarCommand>),
  Arguments(Vec<ClarArgument>),
  Terminator(ClarTerminator),
}

/// Command-line definition.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClarDefinition {
  items: Vec<ClarItem>,
}

impl ClarDefinition {
  /// Returns a collection of command-line items.
  pub fn items(&self) -> &Vec<ClarItem> {
    &self.items
  }

  /// Configures a command-line definition for recognizing option terminator.
  pub fn terminator(&mut self, option_terminator: ClarTerminator) {
    self.items.clear();
    self.items.push(ClarItem::Terminator(option_terminator));
  }

  /// Configures a command-line definition for recognizing options.
  pub fn options<O>(&mut self, options: O)
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
  }

  /// Configures a command-line definition for recognizing options followed by option terminator.
  pub fn options_terminator<O>(&mut self, options: O, option_terminator: ClarTerminator)
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self.items.push(ClarItem::Terminator(option_terminator));
  }

  /// Configures a command-line definition for recognizing arguments.
  pub fn arguments<A>(&mut self, arguments: A)
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Arguments(arguments.into_iter().collect()));
  }

  /// Configures a command-line definition for recognizing arguments followed by option terminator.
  pub fn arguments_terminator<A>(&mut self, arguments: A, option_terminator: ClarTerminator)
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Arguments(arguments.into_iter().collect()));
    self.items.push(ClarItem::Terminator(option_terminator));
  }

  /// Configures a command-line definition for recognizing options followed by arguments.
  pub fn options_arguments<O, A>(&mut self, options: O, arguments: A)
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self.items.push(ClarItem::Arguments(arguments.into_iter().collect()));
  }

  /// Configures a command-line definition for recognizing options followed by arguments and option terminator.
  pub fn options_arguments_terminator<O, A>(&mut self, options: O, arguments: A, option_terminator: ClarTerminator)
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self.items.push(ClarItem::Arguments(arguments.into_iter().collect()));
    self.items.push(ClarItem::Terminator(option_terminator));
  }

  /// Configured a command-line for recognizing commands.
  pub fn commands<C>(&mut self, commands: C)
  where
    C: IntoIterator<Item = ClarCommand>,
  {
    self.items.clear();
    self.items.push(ClarItem::Commands(commands.into_iter().collect()));
  }

  /// Configured a command-line for recognizing options followed by commands.
  pub fn options_commands<O, C>(&mut self, options: O, commands: C)
  where
    O: IntoIterator<Item = ClarOption>,
    C: IntoIterator<Item = ClarCommand>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self.items.push(ClarItem::Commands(commands.into_iter().collect()));
  }

  /// Validates command-line definition.
  pub fn validate(&mut self) -> ClarResult<()> {
    update_paths(&mut self.items, &ClarPath::default());
    self.validate_items(self)?;
    Ok(())
  }

  /// Validates command-line items in definition.
  fn validate_items(&self, def: &ClarDefinition) -> ClarResult<()> {
    for item in def.items() {
      match item {
        ClarItem::Commands(commands) => {
          for command in commands {
            command.validate()?;
            self.validate_items(&command.definition)?;
          }
        }
        ClarItem::Options(options) => {
          for option in options {
            option.validate()?;
          }
        }
        ClarItem::Arguments(arguments) => {
          for argument in arguments {
            argument.validate()?;
          }
        }
        ClarItem::Terminator(_option_terminator) => {}
      }
    }
    Ok(())
  }

  pub fn get_options(&self) -> HashMap<&ClarPath, &ClarOption> {
    let mut option_map = HashMap::new();
    self.collect_options(&self.items, &mut option_map);
    option_map
  }

  fn collect_options<'a>(&self, items: &'a [ClarItem], map: &mut HashMap<&'a ClarPath, &'a ClarOption>) {
    for item in items {
      match item {
        ClarItem::Options(options) => {
          for option in options {
            map.insert(option.get_path(), option);
          }
        }
        ClarItem::Commands(commands) => {
          for command in commands {
            self.collect_options(command.get_definition().items(), map);
          }
        }
        _ => {}
      }
    }
  }

  pub fn get_commands(&self) -> HashMap<&ClarPath, &ClarCommand> {
    let mut map = HashMap::new();
    self.collect_commands(&self.items, &mut map);
    map
  }

  fn collect_commands<'a>(&self, items: &'a [ClarItem], map: &mut HashMap<&'a ClarPath, &'a ClarCommand>) {
    for item in items {
      if let ClarItem::Commands(commands) = item {
        for command in commands {
          map.insert(command.get_path(), command);
          self.collect_commands(command.get_definition().items(), map);
        }
      }
    }
  }

  pub fn find_command<'a>(&'a self, path: &'a ClarPath) -> Option<&'a ClarCommand> {
    self.get_commands().get(path).cloned()
  }
}

fn update_paths(items: &mut [ClarItem], parent: &ClarPath) {
  for item in items {
    match item {
      ClarItem::Commands(commands) => {
        for command in commands {
          command.set_path(parent);
          let parent = parent.join(command.get_name());
          update_paths(&mut command.definition.items, &parent);
        }
      }
      ClarItem::Options(options) => options.iter_mut().for_each(|o| o.set_path(parent)),
      ClarItem::Arguments(arguments) => arguments.iter_mut().for_each(|a| a.set_path(parent)),
      ClarItem::Terminator(option_terminator) => option_terminator.set_path(parent),
    }
  }
}
