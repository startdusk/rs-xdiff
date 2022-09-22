use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use rs_xdiff::{
    cli::{Action, Args, RunArgs},
    DiffConfig, DiffProfile, ExtraArgs, RequestProfile, ResponseProfile, highlight_text,
};
use std::io::Write;

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
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;
    let extra_args = args.extra_params.into();
    let output = profile.diff(extra_args).await?;
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", output)?;

    Ok(())
}

async fn parse() -> anyhow::Result<()> {
    let default = ColorfulTheme::default();
    let url1: String = Input::with_theme(&default)
        .with_prompt("Url1")
        .interact_text()?;
    let url2: String = Input::with_theme(&default)
        .with_prompt("Url2")
        .interact_text()?;

    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;

    let name: String = Input::with_theme(&default)
        .with_prompt("Profile")
        .interact_text()?;
    let resp = req1.send(&ExtraArgs::default()).await?;
    let headers = resp.get_header_keys();
    let chosen = MultiSelect::with_theme(&default)
        .with_prompt("Select headers to skip")
        .items(&headers)
        .interact()?;
    let skip_headers = chosen.iter().map(|i| headers[*i].to_string()).collect();
    let resp = ResponseProfile::new(skip_headers, vec![]);
    let profile = DiffProfile::new(req1, req2, resp);
    let config = DiffConfig::new(vec![(name, profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "---\n{}", highlight_text(&result, "yaml")?)?;
    Ok(())
}
