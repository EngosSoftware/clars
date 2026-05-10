use crate::errors::ClarError;
use crate::model::{ClarArgument, ClarItem, ClarOption, find_command};
use crate::{ClarCommand, ClarTerminator};
use antex::{ColorMode, StyledText, Text};
use std::fmt::Write;

const INDENT: &str = "  ";
const DISTANCE: &str = "  ";
const NL: char = '\n';

pub fn get_error(reason: &ClarError, app: &str, items: &[ClarItem], cm: ColorMode) -> Text {
  let names = reason.source().split("/").collect::<Vec<&str>>();
  error_label("error", cm)
    + reason.as_text(cm)
    + NL
    + NL
    + get_usage(app, &names[..names.len().saturating_sub(1)], items, cm)
}

pub fn get_help(app: &str, description: &Option<String>, items: &[ClarItem], detailed: bool, cm: ColorMode) -> Text {
  get_description(description, cm) + get_usage(app, &[], items, cm) + get_details(items, detailed, cm)
}

pub fn get_help_command(app: &str, path: &str, items: &[ClarItem], detailed: bool, cm: ColorMode) -> Text {
  let names = path.split("/").collect::<Vec<&str>>();
  if let Some(command) = find_command(&names, items) {
    let description = command.get_help();
    let items = command.get_items();
    get_description(description, cm) + get_usage(app, &names, items, cm) + get_details(items, detailed, cm)
  } else {
    error_label("configuration error", cm)
      + Text::new(cm)
        .s("help for command '")
        .yellow()
        .bold()
        .s(path)
        .reset()
        .s("' not found")
  }
}

pub fn get_description(description: &Option<String>, cm: ColorMode) -> Text {
  if let Some(description) = description {
    Text::new(cm).s(description).s("\n\n")
  } else {
    Text::new(cm)
  }
}

pub fn get_usage(app: &str, commands: &[&str], items: &[ClarItem], cm: ColorMode) -> Text {
  let mut usage = Text::new(cm)
    .bright_green()
    .bold()
    .s("Usage:")
    .reset()
    .s(' ')
    .bright_cyan()
    .bold()
    .s(app)
    .reset();
  for name in commands {
    usage += Text::new(cm).s(' ').bright_cyan().bold().s(name).reset();
  }
  for item in items {
    match item {
      ClarItem::Options(options) => {
        usage += " " + get_usage_for_options(options, cm);
      }
      ClarItem::Commands(subcommands) => {
        usage += " " + get_usage_for_subcommands(subcommands, cm);
      }
      ClarItem::Arguments(arguments) => {
        usage += " " + get_usage_for_arguments(arguments, cm);
      }
      ClarItem::Terminator(option_terminator) => {
        usage += " " + get_usage_for_option_terminator(option_terminator, cm);
      }
    }
  }
  usage + NL
}

fn get_usage_for_options(_options: &[ClarOption], cm: ColorMode) -> Text {
  Text::new(cm).cyan().s("[OPTIONS]")
}

fn get_usage_for_subcommands(_subcommands: &[ClarCommand], cm: ColorMode) -> Text {
  Text::new(cm).cyan().s("<COMMAND>")
}

fn get_usage_for_arguments(arguments: &[ClarArgument], cm: ColorMode) -> Text {
  let mut has_optional_arguments = false;
  let mut text = Text::new(cm);
  for (index, argument) in arguments.iter().enumerate() {
    if argument.is_required() {
      if index > 0 {
        text += " ";
      }
      text += Text::new(cm).cyan().s('<').s(argument.get_caption()).s('>').reset();
    } else {
      has_optional_arguments = true;
    }
  }
  if has_optional_arguments {
    text += Text::new(cm).s(' ').cyan().s("[ARGS]").reset();
  }
  text
}

fn get_usage_for_option_terminator(option_terminator: &ClarTerminator, cm: ColorMode) -> Text {
  let mut text = Text::new(cm);
  if option_terminator.is_required() {
    text += Text::new(cm).cyan().s("-- [ARG]...").reset();
  } else {
    text += Text::new(cm).cyan().s("[-- [ARG]...]").reset();
  }
  text
}

fn get_details(items: &[ClarItem], detailed: bool, cm: ColorMode) -> Text {
  let mut text = Text::new(cm);
  for item in items {
    if let ClarItem::Commands(subcommands) = item {
      text += get_details_for_subcommands(subcommands, detailed, cm);
    }
  }
  for item in items {
    if let ClarItem::Arguments(arguments) = item {
      text += get_details_for_arguments(arguments, detailed, cm);
    }
  }
  for item in items {
    if let ClarItem::Options(options) = item {
      text += get_details_for_options(options, detailed, cm);
    }
  }
  text
}

fn get_details_for_options(options: &[ClarOption], detailed: bool, cm: ColorMode) -> Text {
  let mut text = get_group_label("Options:", cm);
  // Prepare captions for all options.
  let captions = options
    .iter()
    .map(|option| get_caption_for_option(option, cm))
    .collect::<Vec<Text>>();
  // Calculate the maximum with of the column where all captions fit.
  let column_width = captions.iter().map(|text| text.count()).max().unwrap_or(0);
  // Prepare detailed descriptions for all options.
  for (option, caption) in options.iter().zip(captions) {
    // Prepare possible values clause.
    let option_hints = get_option_hints(option, detailed);
    let details = if detailed {
      format!("{}{option_hints}", option.get_help_long().clone().unwrap_or_default())
    } else {
      format!("{}{option_hints}", option.get_help().clone().unwrap_or_default())
    };
    if details.is_empty() {
      text += Text::new(cm) + INDENT + caption + NL;
    } else {
      text += Text::new(cm)
        + INDENT
        + caption.fill(' ', column_width)
        + DISTANCE
        + get_indented_column(details, column_width)
        + NL;
    }
  }
  text
}

fn get_details_for_subcommands(commands: &[ClarCommand], detailed: bool, cm: ColorMode) -> Text {
  let mut text = get_group_label("Commands:", cm);
  // Prepare all captions.
  let captions = commands
    .iter()
    .map(|subcommand| get_caption_for_command(subcommand, cm))
    .collect::<Vec<Text>>();
  // Calculate the maximum with of the column where all captions fit.
  let column_width = captions.iter().map(|t| t.count()).max().unwrap_or(0);
  // Prepare detailed descriptions.
  for (command, caption) in commands.iter().zip(captions) {
    let details = if detailed {
      get_indented_column(command.get_help_long().clone().unwrap_or_default(), column_width)
    } else {
      get_indented_column(command.get_help().clone().unwrap_or_default(), column_width)
    };
    text += Text::new(cm) + INDENT + caption.fill(' ', column_width) + DISTANCE + details + NL;
  }
  text
}

fn get_details_for_arguments(arguments: &[ClarArgument], detailed: bool, cm: ColorMode) -> Text {
  // Prepare a label displayed before argument list.
  let mut text = get_group_label("Arguments:", cm);
  // Prepare all captions.
  let captions = arguments
    .iter()
    .map(|a| get_caption_for_argument(a, cm))
    .collect::<Vec<Text>>();
  // Calculate the maximum with of the column where all captions fit.
  let column_width = captions.iter().map(|t| t.count()).max().unwrap_or(0);
  // Prepare detailed descriptions.
  for (argument, caption) in arguments.iter().zip(captions) {
    let details = if detailed {
      get_indented_column(argument.get_help_long().clone().unwrap_or_default(), column_width)
    } else {
      get_indented_column(argument.get_help().clone().unwrap_or_default(), column_width)
    };
    text += Text::new(cm) + INDENT + caption.fill(' ', column_width) + DISTANCE + details + NL;
  }
  text
}

/// Returns colored caption for the given option.
fn get_caption_for_option(option: &ClarOption, cm: ColorMode) -> Text {
  let mut text = Text::new(cm);
  // Prepare short label if present.
  if let Some(short_name) = option.get_short_label() {
    text += Text::new(cm).bright_cyan().bold().s("-").s(short_name).reset();
  } else {
    text += INDENT;
    text += "  ";
  }
  // Prepare long label if present.
  if let Some(long_label) = option.get_long_label() {
    text += Text::new(cm).s(", ").bright_cyan().bold().s("--").s(long_label).reset();
  }
  // Prepare value caption if present.
  if let Some(caption) = option.get_takes_value() {
    if option.get_default_missing_value().is_some() {
      text += Text::new(cm).s(' ').cyan().s("[<").s(caption).s(">]").reset();
    } else {
      text += Text::new(cm).s(' ').cyan().s("<").s(caption).s(">").reset();
    }
  }
  text
}

/// Returns colored caption for the given command.
fn get_caption_for_command(subcommand: &ClarCommand, cm: ColorMode) -> Text {
  Text::new(cm).bright_cyan().s(subcommand.get_name()).reset()
}

/// Returns colored caption for the given argument.
fn get_caption_for_argument(argument: &ClarArgument, cm: ColorMode) -> Text {
  if argument.is_required() {
    Text::new(cm).cyan().s('<').s(argument.get_caption()).s('>').reset()
  } else {
    Text::new(cm).cyan().s('[').s(argument.get_caption()).s(']').reset()
  }
}

fn get_indented_column(input: String, column_width: usize) -> String {
  let mut text = String::new();
  for (index, line) in input.lines().enumerate() {
    if index > 0 {
      _ = write!(&mut text, "\n{INDENT}{}{DISTANCE}", " ".repeat(column_width));
    }
    _ = write!(&mut text, "{line}");
  }
  text
}

/// Returns colored group label displayed before the usage description
/// for specific items like options, arguments and commands.
fn get_group_label(label: &str, cm: ColorMode) -> Text {
  Text::new(cm).s('\n').bright_green().bold().s(label).reset().s('\n')
}

/// Returns colored error label.
fn error_label(label: &str, cm: ColorMode) -> Text {
  Text::new(cm).bright_red().bold().s(label).s(':').reset().s(' ')
}

fn get_option_hints(option: &ClarOption, long: bool) -> String {
  let mut hints = String::new();
  let mut prefix = "";
  if let Some(value) = option.get_default_value() {
    _ = write!(&mut hints, "[default: {value}]");
    prefix = " ";
  }
  if let Some(value) = option.get_default_missing_value() {
    _ = write!(&mut hints, "{prefix}[implicit: {value}]");
    prefix = " ";
  }
  if !option.get_possible_values().is_empty() {
    _ = write!(
      &mut hints,
      "{prefix}(values: {})",
      option.get_possible_values().join(", ")
    );
  }
  if hints.is_empty() {
    hints
  } else {
    prefix = match (long, option.get_help().is_some(), option.get_help_long().is_some()) {
      (false, false, _) => "",
      (false, true, _) => " ",
      (true, _, false) => "",
      (true, _, true) => "\n  ",
    };
    format!("{prefix}{hints}")
  }
}
