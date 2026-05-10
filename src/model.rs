/// CLI option.
#[derive(Debug, Default, Clone)]
pub struct ClarOption {
  /// Option name.
  name: String,
  /// Short label.
  short_label: Option<char>,
  /// Long label.
  long_label: Option<String>,
  /// Indicates whether option must be used without other options, arguments, or commands.
  standalone: bool,
  /// Default value used when the option is not specified.
  default_value: Option<String>,
  /// Default value used when option is provided without value.
  default_missing_value: Option<String>,
  /// List of possible values for option.
  possible_values: Vec<String>,
  /// Indicates whether option takes a value.
  takes_value: Option<String>,
  /// Help content.
  help: Option<String>,
  /// Long help content.
  help_long: Option<String>,
  /// Path in definition tree.
  path: String,
}

impl ClarOption {
  pub fn new(name: impl AsRef<str>, short: char, long: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      short_label: Some(short),
      long_label: Some(long.as_ref().to_string()),
      ..Default::default()
    }
  }

  pub fn new_short(name: impl AsRef<str>, label: char) -> Self {
    Self {
      name: name.as_ref().to_string(),
      short_label: Some(label),
      ..Default::default()
    }
  }

  pub fn new_long(name: impl AsRef<str>, label: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      long_label: Some(label.as_ref().to_string()),
      ..Default::default()
    }
  }

  pub fn short(mut self, label: char) -> Self {
    self.short_label = Some(label);
    self
  }

  pub fn long(mut self, label: impl AsRef<str>) -> Self {
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

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_short_label(&self) -> &Option<char> {
    &self.short_label
  }

  pub fn get_long_label(&self) -> &Option<String> {
    &self.long_label
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

  pub fn get_help(&self) -> &Option<String> {
    &self.help
  }

  pub fn get_help_long(&self) -> &Option<String> {
    &self.help_long
  }

  fn set_path(&mut self, parent_path: impl AsRef<str>) {
    self.path = make_path(parent_path.as_ref(), &self.name);
  }

  pub(crate) fn get_path(&self) -> &str {
    &self.path
  }
}

/// CLI argument.
#[derive(Debug, Default, Clone)]
pub struct ClarArgument {
  /// Argument name.
  name: String,
  /// Caption displayed in usage instead of the name.
  caption: String,
  /// Default value used when argument is not specified.
  default_value: Option<String>,
  /// Indicates whether argument is required.
  required: bool,
  /// Help content.
  help: Option<String>,
  /// Long help content.
  help_long: Option<String>,
  /// Path in definition tree.
  path: String,
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

  pub fn get_name(&self) -> &str {
    &self.name
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
    if self.help_long.is_some() {
      &self.help_long
    } else {
      self.get_help()
    }
  }

  fn set_path(&mut self, parent_path: impl AsRef<str>) {
    self.path = make_path(parent_path.as_ref(), &self.name);
  }

  pub(crate) fn get_path(&self) -> &str {
    &self.path
  }
}

/// CLI command.
#[derive(Debug, Default, Clone)]
pub struct ClarCommand {
  /// Command name.
  name: String,
  /// Child definition items.
  items: Vec<ClarItem>,
  /// Help content.
  help: Option<String>,
  /// Long help content.
  help_long: Option<String>,
  /// Path definition tree.
  path: String,
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

  /// Returns command resolver that recognizes an **option terminator**.
  pub fn terminator(&mut self, terminator: ClarTerminator) {
    self.items.clear();
    self.items.push(ClarItem::Terminator(terminator));
  }

  /// Returns command resolver that recognizes **options**.
  pub fn options<O>(mut self, options: O) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self
  }

  /// Returns command resolver that recognizes **options** followed by **option terminator**.
  pub fn options_t<O>(&mut self, options: O, t: ClarTerminator)
  where
    O: IntoIterator<Item = ClarOption>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(options.into_iter().collect()));
    self.items.push(ClarItem::Terminator(t));
  }

  /// Returns command resolver that recognizes **arguments**.
  pub fn arguments<A>(mut self, a: A) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self
  }

  /// Returns command resolver that recognizes **arguments** followed by **option terminator**.
  pub fn arguments_t<A>(mut self, a: A, t: ClarTerminator) -> Self
  where
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self.items.push(ClarItem::Terminator(t));
    self
  }

  /// Returns command resolver that recognizes **options** followed by **arguments**.
  pub fn options_arguments<O, A>(mut self, o: O, a: A) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self
  }

  /// Returns command resolver that recognizes **options** followed by **arguments** and **option terminator**.
  pub fn options_arguments_t<O, A>(mut self, o: O, a: A, t: ClarTerminator) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    A: IntoIterator<Item = ClarArgument>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self.items.push(ClarItem::Arguments(a.into_iter().collect()));
    self.items.push(ClarItem::Terminator(t));
    self
  }

  /// Returns command resolver that recognizes **commands**.
  pub fn commands<S>(mut self, s: S) -> Self
  where
    S: IntoIterator<Item = ClarCommand>,
  {
    self.items.clear();
    self.items.push(ClarItem::Commands(s.into_iter().collect()));
    self
  }

  /// Builds subcommand resolver that recognizes **options** followed by **subcommands**.
  pub fn options_subcommands<O, S>(mut self, o: O, s: S) -> Self
  where
    O: IntoIterator<Item = ClarOption>,
    S: IntoIterator<Item = ClarCommand>,
  {
    self.items.clear();
    self.items.push(ClarItem::Options(o.into_iter().collect()));
    self.items.push(ClarItem::Commands(s.into_iter().collect()));
    self
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub(crate) fn get_items(&self) -> &Vec<ClarItem> {
    &self.items
  }

  pub fn get_help(&self) -> &Option<String> {
    &self.help
  }

  pub fn get_help_long(&self) -> &Option<String> {
    if self.help_long.is_some() {
      &self.help_long
    } else {
      self.get_help()
    }
  }

  fn set_path(&mut self, parent_path: impl AsRef<str>) {
    self.path = make_path(parent_path.as_ref(), &self.name);
  }

  pub(crate) fn get_path(&self) -> &str {
    &self.path
  }
}

/// Command line option terminator (`--`).
///
/// When present in the argument list, everything after it is treated
/// as a positional argument, even if it looks like an option.
#[derive(Debug, Default, Clone)]
pub struct ClarTerminator {
  /// Terminator name.
  pub name: String,
  /// Indicated whether terminator is required.
  required: bool,
  /// Path definition tree.
  path: String,
}

impl ClarTerminator {
  /// Creates a new terminator with the given name.
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

  /// Returns the option terminator name.
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn is_required(&self) -> bool {
    self.required
  }

  fn set_path(&mut self, parent_path: impl AsRef<str>) {
    self.path = make_path(parent_path.as_ref(), &self.name);
  }
  pub(crate) fn get_path(&self) -> &str {
    &self.path
  }
}

/// Command line item enumeration.
#[derive(Debug, Clone)]
pub enum ClarItem {
  Options(Vec<ClarOption>),
  Commands(Vec<ClarCommand>),
  Arguments(Vec<ClarArgument>),
  Terminator(ClarTerminator),
}

pub fn update_paths(items: &mut Vec<ClarItem>, segments: &mut Vec<String>) {
  let parent_path = segments.join("/");
  for item in items {
    match item {
      ClarItem::Commands(subcommands) => {
        for subcommand in subcommands {
          subcommand.set_path(&parent_path);
          segments.push(subcommand.get_name().to_string());
          update_paths(&mut subcommand.items, segments);
          segments.pop();
        }
      }
      ClarItem::Options(options) => options.iter_mut().for_each(|o| o.set_path(&parent_path)),
      ClarItem::Arguments(arguments) => arguments.iter_mut().for_each(|a| a.set_path(&parent_path)),
      ClarItem::Terminator(option_terminator) => option_terminator.set_path(&parent_path),
    }
  }
}

pub fn display_tree(items: &[ClarItem]) {
  for item in items {
    match item {
      ClarItem::Commands(subcommands) => {
        for subcommand in subcommands {
          println!("{}", subcommand.get_path());
          display_tree(&subcommand.items);
        }
      }
      ClarItem::Options(options) => options.iter().for_each(|o| println!("{}", o.get_path())),
      ClarItem::Arguments(arguments) => arguments.iter().for_each(|a| println!("{}", a.get_path())),
      ClarItem::Terminator(option_terminator) => println!("{}", option_terminator.get_path()),
    }
  }
}

pub fn find_command<'a>(command_names: &'a [&str], items: &'a [ClarItem]) -> Option<&'a ClarCommand> {
  let searched_name = command_names.first()?;
  let command = items
    .iter()
    .filter_map(|item| {
      if let ClarItem::Commands(commands) = item {
        Some(commands)
      } else {
        None
      }
    })
    .flatten()
    .find(|command| command.get_name() == *searched_name)?;
  let remaining_names = &command_names[1..];
  if remaining_names.is_empty() {
    Some(command)
  } else {
    find_command(remaining_names, command.get_items())
  }
}

fn make_path(parent_path: &str, name: &str) -> String {
  if parent_path.is_empty() {
    name.to_string()
  } else {
    format!("{}/{}", parent_path, name)
  }
}
