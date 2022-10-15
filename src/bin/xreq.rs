// 解决使用同名的trait的方式 as _
use std::fmt::Write as _;
use std::io::Write as _;

use anyhow::Ok;
use dialoguer::{theme::ColorfulTheme, Input};
use rs_xdiff::{
    cli::{Action, Args, RunArgs},
    get_body_text, get_header_text, get_status_text, highlight_text, LoadConfig, RequestConfig,
    RequestProfile,
};

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.action {
        Action::Run(args) => run(args).await?,
        Action::Parse => parse().await?,
        _ => panic!("Not implemented"),
    }

    Ok(())
}

async fn run(args: RunArgs) -> anyhow::Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xreq.yml".to_string());
    let config = RequestConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;
    let extra_args = args.extra_params.into();
    let url = profile.get_url(&extra_args)?;
    let res = profile.send(&extra_args).await?.into_inner();

    let mut output = String::new();
    let status = get_status_text(&res)?;
    let headers = get_header_text(&res, &[])?;
    let body = get_body_text(res, &[]).await?;
    writeln!(&mut output, "Url: {}\n", url)?;
    write!(&mut output, "{}", status)?;
    write!(
        &mut output,
        "{}",
        highlight_text(&headers, "yaml", Some("InspiredGitHub"))?
    )?;
    write!(&mut output, "{}", highlight_text(&body, "json", None)?)?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", output)?;
    Ok(())
}

async fn parse() -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    let url: String = Input::with_theme(&theme)
        .with_prompt("Url")
        .interact_text()?;
    let profile: RequestProfile = url.parse()?;

    let name: String = Input::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    let config = RequestConfig::new(vec![(name, profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    if atty::is(atty::Stream::Stdout) {
        write!(stdout, "---\n{}", highlight_text(&result, "yaml", None)?)?;
    } else {
        write!(stdout, "{}", result)?;
    }

    Ok(())
}
