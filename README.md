# detox

🧹 no toxic names anymore.

## 🚀 Installation

To install **detox**, simply clone the repository and follow the instructions below:

```sh
git clone https://github.com/trinhminhtriet/detox.git
cd detox

cargo install --path .

detox --do my_path
```

> By default, detox will only print the names that would be renamed. Use the `--do` or `-d` option to actually rename the files.

## 💡 Options

| Option                | Description                  |
| --------------------- | ---------------------------- |
| `-v`, `--version`     | Prints version information   |
| `-d`, `--do`          | Do the actions               |
| `-q`, `--quiet`       | No output                    |
| `-j`, `--json`        | Output as JSON               |
| `-p`, `--json-pretty` | Output as JSON (prettified)  |
| `-e`, `--json-error`  | Output as JSON (only errors) |

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Information

- [CHANGELOG](CHANGELOG.md)
