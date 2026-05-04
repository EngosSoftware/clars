/// Command line option definition.
#[derive(Debug, Default, Clone)]
pub struct CliOption {
  /// Option name (unique).
  pub name: String,
  /// Name of the short label.
  pub short_name: Option<char>,
  /// Name of the long label.
  pub long_name: Option<String>,
  /// Indicates whether this option must be used on its own,
  /// without any other options, arguments, or subcommands.
  pub standalone: bool,
  /// Default value used if the option is not provided on the command line.
  /// Only applicable when `takes_value` is `true`.
  pub default_value: Option<String>,
  /// The value used when the option is provided without an explicit value.
  /// Only applicable when `takes_value` is `true`.
  pub default_missing_value: Option<String>,
  /// Indicates whether this option expects an associated value.
  /// If `true`, the option takes a value; otherwise, it is a flag.
  pub takes_value: bool,
}

/// Command line argument definition.
#[derive(Debug, Default, Clone)]
pub struct CliArgument {
  /// Argument name (unique).
  pub name: String,
  /// Value assigned to argument when it does not appear in the command line.
  pub default_value: Option<String>,
  /// indicates if this argument is required and must appear in the command line.
  pub required: bool,
}

/// Command line subcommand definition.
#[derive(Debug, Default, Clone)]
pub struct CliSubcommand {
  /// Subcommand name (unique).
  pub name: String,
  /// Optional value associated with the subcommand.
  pub value: Option<String>,
  /// Default value used if the subcommand is not provided on the command line.
  pub default_value: Option<String>,
  /// Child command line items for subcommand.
  pub items: Vec<CliItem>,
}

/// Command line item enumeration.
#[derive(Debug, Clone)]
pub enum CliItem {
  Options(Vec<CliOption>),
  Subcommands(Vec<CliSubcommand>),
  Arguments(Vec<CliArgument>),
}
