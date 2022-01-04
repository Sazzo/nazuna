<h1 align="center">Nazuna</h1>
<p align="center">🐦 Download Twitter videos using your terminal! </p>

<p align="center">
<img src="https://i.imgur.com/91vHedl.gif">
</p>
  
## Installation

### Binary

Download the desired file for your OS (Windows, Mac, Linux) from https://github.com/Sazzo/nazuna/releases

### AUR
[![AUR maintainer](https://img.shields.io/aur/maintainer/nazuna?logo=arch-linux&style=flat-square)](https://aur.archlinux.org/packages/nazuna)  

```
$ yay -S nazuna
```

## Usage

```
$ nazuna <Tweet URL>
```

By default, the output file will be `output.mp4`, if you want to specify how this file will look like, you need to use the `--output` option:

```
$ nazuna <Tweet URL> --output=<File>
```

## Configuration

Nazuna requires a Twitter API Key and API Secret to work properly, to obtain these keys you need to register a application on the Twitter Developer Portal.

### Getting Started

Go to https://developer.twitter.com/en/portal/dashboard, if it's your first time accessing the dashboard, you'll need to fill a form with some details about your usage and then verify your email (ps: you need a verified phone number in your twitter account).

### Creating the Application

If again, it's your first time accessing the dashboard, the first thing that should appear after filling the usage form is the application creationg form, that should ask about the app name, app description and etc.

If it's not your first time accessing the dashboard, you need to create a new project which if you have no project in your account, a big blue button should appear to you on the home page, see below for a example. Fill the project name, use case and project description, after this create a new app like described above.

<img width="900px" src="https://user-images.githubusercontent.com/39680458/147893345-8da5f41f-087b-4a0a-8002-0723beb699b0.png">

After filling the basic app information, the keys should appear to you (especially, the API Key and API Secret), copy them and proceed to the next step.

### Using the API Keys on Nazuna

Now, try to download a video using Nazuna and you should be asked about the API Key and API Secret. Paste first the API Key and then, click enter to paste the API Secret. These keys will be stored at your config directory in `nazuna/credentials.json` for future uses.

![image](https://user-images.githubusercontent.com/39680458/147893437-3f8f6b55-63c3-4003-a9b0-4a0d18e2d5d9.png)

## Notes

- Yep, I'm a noob in Rust and I'm trying to improve. If you want to give some feedback, feel free to open a issue.
