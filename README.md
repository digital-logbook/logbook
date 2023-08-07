# Logbook

A command-line application for tracking your computer usage times.


## Intended usage

Please note that since *The Digital Logbook* is in an early stage of development, a lot of the commands do not yet work. The actual usage may even change a little. Nonetheless I will show you my current plan regarding the functionality:

### 1. Showing help information

```shell
$ logbook help
$ logbook -h
$ logbook --help
```

### Creating a new entry

```shell
$ logbook create
# Or if you need to overwrite an existing (but not yet finished) entry
$ logbook create --force  # Or...
$ logbook create -f  # ... if you want a shorter command
```

### Finishing an existing entry

```shell
$ logbook finish  # This will prompt you for the required values
$ logbook finish --description "Working on logbook"  # Or provide these values directly

# Maybe even allow the user to provide an (optional) tag to allow
# grouping/filtering entries by tags?
$ logbook finish --description "Adding new features" --tag "digital-logbook"
```

### Showing all entries or filter for specific ones

```shell
$ logbook show  # Displays all entries
$ logbook show --all  # More explicit version of the last command
$ logbook show --date "2023-08-06"

$ logbook show --month "August"  # Month values should be generic about their
# type, so "August", "august", "Aug", "aug", and 8 are all valid months
$ logbook show --month "August" --description "Working on logbook"
```

### Reporting

```shell
# This is a command that I am currently struggling to implement. It should
# display a report of the computer usage times for the current day, the
# current week and maybe even the current month and year. I am dreaming of
# a nice graph or something similar, however, I need to do some research on
# how to achieve that.
$ logbook report
```

### Entering an interacive REPL

```shell
# I think that always having to preceed a sucommand with the name of the program,
# i.e. `logbook`, is quite tedious. This is why you can also enter a so called
# Read-Eval-Print-Loop to simplify that.
$ logbook
(logbook) # This prompt indicates that you have successfully entered the REPL
(logbook) create  # Simply write the subcommands without the preceding `logbook`
(logbook) report
(logbook) help
(logbook) quit
$
```
