# Pro-cli rewrite

Complete rewrite of https://github.com/tyrkinn/pro-cli

** WARNING **
Now it works only on UNIX machines

## Get Started

1. Define your config

You can find config example in config_example/pro/config.ron
Config Should be located at $HOME/.config/pro/config.ron

2. Define your templates or copy default

Default templates located at config_example/pro/templates
Templates should be located at $HOME/.config/pro/templates/{index.hbs, project.hbs}


3. Building executable

Use this in project directory

```shell
$ cargo install --path .

```

4. Enjoy!
