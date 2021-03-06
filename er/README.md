# -er

## Zenn Article Initializer

### Usage

```bash
$ cargo build
```

Place `target/debug/zai.exe` directly under the folder where you keep your Zenn articles, and run the following command.

```bash
$ zai <title> <topics>
```

The following three patterns can be executed.

- Nothing
- Title only
- With topics

```bash
$ zai title one two three
```

If you run the above command, you will get the following results.

**`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx.md`**

```md
---
title: title
emoji: 🐒
type: tech
topics: [one, two, three]
published: false
---
```

https://github.com/ogty/article-continuous-integration

Work with article-coutinuous-integration.
We want to make it possible to create a code-based system from an article-based system.
I think it is very important to continuously update the information contained in the article content.

***

## Lambda Function Zipper

From the path of the directory where the specified `lambda_function.py` is located, 
install the external modules written in `requirements.txt` directly under it in this directory.

### Usage

```bash
$ cargo build
```

The `/target/debug/lfzipper` created by the `cargo build` command should be placed directly under your desktop.

```bash
$ lfzipper <path>   # windows
$ ./lfzipper <path> # macos and linux
```
