<picture>
  <img height="500" width="500" src="https://github.com/JSH5000/RUSTY_LLM/assets/39937127/de2ce28a-8408-4ebc-9615-1a4fec2103a1" alt="Rusty LLM" />
</picture>

# Give me the gist
This project is aimed at creating a web chat interface using Rust. The user should be able to prompt a model and get a response back *in a reasonable amount of time*.

## Based upon the Best!
This project is currently a copy-pasta (or more of type-pasta) of the great [**CODE TO THE MOON**](https://www.youtube.com/watch?v=vAjle3c9Xqc&t=2184s)!

## Leptos
There are parts of the base template from Leptos used in this project, please refer to their page for docs: [Leptos](https://github.com/leptos-rs/leptos)

## Rustformers
This is the glue of the project, the rustformers allow us to LLM models into rust. Please refer to their docs: [RUSTFormers](https://github.com/rustformers)

# Getting Started
If you don't have rust installed, please do so first. Follow along on the rust setup guide [here](https://www.rust-lang.org/tools/install).
<br/>
<br/>
Next, you will want to clone the repo. Before doing anything you will need to create a **.env** file. Once that file is created, you will need to add the following to it:
<br/>
<br/>
MODEL_PATH="" <-- Don't worry about the empty string, you will fill that in soon!
<br/>
<br/>
Next, you will need to download a model for the project to interact with. I am using the example model *WIZARD-VICUNA-7B-Uncensored*. These models can be located from [Hugging-Face](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML/tree/main)
<br/>
Now! You can replace that empty string above in the *.env* file with the path to the model bin file.
<br/>
<br/>
Ok, we are almost done with setup! Because the *.toml* file is included, you will only have to run the build command for rust! Below are the steps:
<br/>
1.) CD into the working directory (based on your OS)
<br/>
2.) Then run the command: `cargo build` <-- this will use the *.toml* file to rebuild dependencies.
<br/>
3.) Once finished, you should be able to run the web server. To do so, run the command: `cargo leptos watch`

# Additional Notes
There may be broken bits of this project, currently it may not compile properly due to a reliance on the `nightly-rust` build.
