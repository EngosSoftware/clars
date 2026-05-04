# ADR

## Resolving CLI items

CLI items are of the following types:
- options,
- subcommands,
- arguments.

The meaning is as commonly understand in Unix command line.

CLI items are resolved in homogenous groups. Groups are resolved sequentially, for example:

```
command [options] [arguments]
                       ^------ group of arguments
            ^----------------- group of options
                       
```

Supported group combinations:
- command {options}
- command {arguments}
- command <subcommands>
- command {options} {arguments}
- command {options} <subcommands>

Each subcommand may be followed by:
- subcommand {options}
- subcommand {arguments}
- subcommand <subcommands>
- subcommand {options} {arguments}
- subcommand {options} <subcommands>

In this case options in a group will be resolved before arguments in a group.

### Resolving options

Options in group are resolved sequentially, only options defined in group may be resolved.

### Resolving arguments

Arguments are resolved sequentially in the same order as they are defined in a group. 
Required and optional arguments can not be mixed.
Required arguments MUST be defined before optional arguments.
