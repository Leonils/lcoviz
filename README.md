# Lcoviz

Lcoviz is a simple rust tool to visualize the coverage report generated by `lcov` in different formats.

## Installation

### From github releases

Download the latest release from [releases](https://github.com/Leonils/lcoviz/releases). Give the executable a nice short name, such as `lcoviz`. You can copy it to a location known by your path, such as `/usr/local/bin` or `~/.local/bin`.

### Using cargo

You can also build the tool from source. You need to have `cargo` installed. You can either clone the repo manually and install it with cargo, or let cargo do it for you:

```bash
# Let cargo clone repo for you
cargo install --git 'https://github.com/Leonils/lcoviz.git'

# Manually clone the repo
git clone https://github.com/Leonils/lcoviz.git
cd lcoviz
cargo install --path .
```

In either case it will be installed as a cargo bin, see `cargo help install` for more information.

## Usage

### Report

The main command of the tool is `report`. It allows you to create a visualization of one or several lcov files. The command has the following options:

- `--name` or `-n`: The name of the report. This will be used as the title of the report.
- `--reporter` or `-r`: The format of the report. The available formats are `html-full-light` (aka `html`, `html-full`) and `text-summary`.
- `--output` or `-o`: The output directory where the report will be generated. Note that if there are files in this directory, they might be overriden.
- `--input` or `-i`: The input lcov file. You can specify multiple input files in a single run. An input has a name (as a title for the section of the visualization related to this input), a prefix (from which visualization build the paths to files), and the path to the lcov input file. Only the path is required: You can provide up to 3 values for each input:
  - `<PATH TO LCOV>`: the prefix will be the common parts of all tested files, the name will be the last folder of prefix
  - `<NAME> <PATH TO LCOV>`: you can force the name
  - `<NAME> <PREFIX> <PATH TO LCOV>`: you can force the prefix to be higher if you want to display more levels in the visualisation (for instance all tested files are in `~/my_project/src/lib`, you may want to force prefix `~/my_project` so you see the `src` and `lib` folders in the visualisation)

For instance, you might have to run a command like this one:

```bash
lcoviz report
    --name 'Coverage report for my 3 projects'
    --reporter html
    --input ./project_1.lcov
    --input MyProject ./project_2.lcov
    --input AnotherProject /with/some/custom/prefix/ project_3.lcov
    --output ./coverage_report
```

### Save to file, report from file

As there are a lot of options to pass to the CLI, you may want to save them to a configuration file. You can do so using the `to-file` and the `from-file` commands.

The `to-file` command takes a path directly after the command, and then the same options as the `report` command, but instead of generating a report, it saves the options to the file:

```bash
lcoviz to-file ./config.toml
    --name 'Coverage report for my 3 projects'
    --reporter html
    --input ./project_1.lcov
    --output ./coverage_report
```

The `from-file` command takes a path to the config file, and then generates the report using the options saved in the file:

```bash
lcoviz from-file ./config.toml
```

## Screenshots

### HTML report

![HTML report](https://github.com/Leonils/lcoviz/raw/main/docs/screenshots/html-full-light.png)

## Text report

![Text report](https://github.com/Leonils/lcoviz/raw/main/docs/screenshots/text-summary.png)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
