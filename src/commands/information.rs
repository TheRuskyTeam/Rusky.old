use crate::apis::covid19_brazil_api;
use crate::constants::colors::DISCORD_BLUE;
use crate::constants::emotes::*;
use crate::constants::errors_codes::*;
use crate::containers::RuskyConfigContainer;
use crate::containers::ShardManagerContainer;
use crate::util::misc::get_rust_version;
use humansize::{file_size_opts, FileSize};
use serenity::builder::CreateEmbed;
use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{
    help_commands,
    macros::{command, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;
use std::process;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

#[command]
#[description("Pong! veja o ping do bot e shard atual.")]
pub async fn ping(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let mut message = message
        .reply(context, &format!("{} **·** Carregando...", LOADING_EMOTE))
        .await?;
    let data = context.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            message
                .edit(context, |message| {
                    message.content(&format!(
                        "{} | Ocorreu uma falha ao pegar informações. Código: `{}`",
                        ERROR_EMOTE, SHARD_MANAGER_GET_FAILED
                    ))
                })
                .await?;
            return Ok(());
        }
    };

    let config = data.get::<RuskyConfigContainer>().unwrap();
    let shard_amount = config.discord.shard_amount;
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let shard = match runners.get(&ShardId(context.shard_id)) {
        Some(runner) => runner,
        None => {
            message
                .edit(context, |message| {
                    message.content(&format!(
                        "{} | Ocorreu uma falha ao pegar informações. Código: `{}`",
                        ERROR_EMOTE, SHARD_NOT_FOUND
                    ))
                })
                .await?;
            return Ok(());
        }
    };
    let websocket_ping = match shard.latency {
        Some(latency) => format!("{:?}", latency),
        None => "...".into(),
    };
    let now = Instant::now();

    let _ = isahc::get_async("https://discord.com/api/guilds/854345266625642516/widget.json").await;
    let api_ping = now.elapsed();
    message
        .edit(context, |message| {
            message.content("").embed(|builder| {
                builder
                    .title(&format!("{} **·** Pong!", PING_PONG_EMOTE))
                    .color(0x5865F2)
                    .description(&format!(
                    "{} **·** Shard Atual: `{}/{}`\n{} **·** Latencia do WebSocket: `{}`\n{} **·** Latencia da API: `{:?}`",
                    SATELLITE_OBITAL_EMOTE,
                    context.shard_id + 1,
                    shard_amount,
                    STOPWATCH_EMOTE,
                    websocket_ping,
                    ZAP_EMOTE,
                    api_ping
                ))
            })
        })
        .await?;
    Ok(())
}
#[command]
#[description("Minhas informações")]
pub async fn botinfo(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let mut message = message
        .reply(context, &format!("{} **·** Carregando...", LOADING_EMOTE))
        .await?;
    let system = System::new_all();

    let current_process = system.get_process(process::id() as i32);
    if let Some(process) = current_process {
        let rust_version = get_rust_version();
        let memory_usage = (process.memory() * 1024).file_size(file_size_opts::BINARY);
        let cpu_usage = format!("{}%", process.cpu_usage());
        message.edit(context, |builder| {
            builder.content("").embed(|embed| {
                embed
                    .title("Minhas informações")
                    .description(
                        format!("{} **·** Versão: `v{}`\n{} **·** Versão do Rust: `v{}`\n{} **·** Uso de CPU: `{}`\n{} **·** Uso de Ram: `{}`",
                        MAG_EMOTE,
                        env!("CARGO_PKG_VERSION"),
                        RUST_CUSTOM_EMOTE,
                        rust_version,
						COMPUTER_EMOTE,
                        cpu_usage,
                        COMPUTER_EMOTE,
                        memory_usage.unwrap_or_else(|_| String::from("...")),
)
                    )
                    .color(DISCORD_BLUE)
            })
        }).await?;
    } else {
        message
            .edit(context, |builder| {
                builder.content(&format!(
                    "{} **·** Falha ao carregar informações. Código: `{}`",
                    ERROR_EMOTE, FAILED_TO_GET_PROC
                ))
            })
            .await?;
    }

    Ok(())
}

#[help]
#[ungrouped_label("Sem grupo")]
#[strikethrough_commands_tip_in_dm("Os comandos ~~tachados~~ não estão disponíveis porque requerem permissões, requerem uma função específica, requerem certas condições ou não podem ser executados em mensagens diretas.")]
#[strikethrough_commands_tip_in_guild("Os comandos ~~tachados~~ não estão disponíveis porque requerem permissões, requerem uma função específica, requerem certas condições ou não podem ser executados em Guildas.")]
#[individual_command_tip(
    "Olá se você quiser mais informações sobre um comando passe o nome do comando como argumento."
)]
#[command_not_found_text("O comando `{}` não existe.")]
#[dm_only_text("Na DM.")]
#[no_help_available_text("Esse comando não existe ou não foi configurado.")]
#[aliases_label("Sinônimos")]
#[description_label("Descrição")]
#[sub_commands_label("SubComandos")]
#[usage_label("Exemplo de uso")]
#[grouped_label("Grupo")]
#[available_text("Somente")]
#[dm_and_guild_text("Em Guildas e DM.")]
#[suggestion_text("Você quis dizer `{}`?")]
#[guild_only_text("Em Guildas")]
#[usage_sample_label("Exemplo(s) de uso")]
#[wrong_channel("Strike")]
#[max_levenshtein_distance(5)]
async fn help(
    context: &Context,
    message: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, message, args, help_options, groups, owners).await;
    Ok(())
}
#[command]
#[aliases("covid-status", "coronavirus-status")]
async fn covidstatus(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let mut message = message
        .reply(context, &format!("{} **·** Carregando...", LOADING_EMOTE))
        .await?;
    let status = covid19_brazil_api::fetch_data().await?;
    let mut embed = CreateEmbed::default();
    embed.color(DISCORD_BLUE);
    embed.title(format!("{} **·** Covid status", MAG_EMOTE));
    let mut total_cases = 0;
    let mut total_deaths = 0;
    let mut total_suspects = 0;
    for state in status.data {
        total_cases += state.cases;
        total_deaths += state.deaths;
        total_suspects += state.suspects;
        embed
            .field(
                state.state,
                format!(
                    "{} **·** Casos: {}\n {} **·** Mortes: {}\n{} **·** Suspeitas: {}",
                    MAG_EMOTE,
                    state.cases,
                    SKULL_CROSSBONES_EMOTE,
                    state.deaths,
                    WARNING_EMOTE,
                    state.suspects
                ),
                true,
            )
            .footer(|f| f.text("Powered by https://covid19-brazil-api.vercel.app/"));
    }
    embed.description(format!("{} **·** Total de casos: {}\n{} **·** Total de mortes: {}\n{} **·** Total de suspeitos: {}", MAG_EMOTE, total_cases, SKULL_CROSSBONES_EMOTE, total_deaths, WARNING_EMOTE, total_suspects));
    message
        .edit(context, |builder| builder.content("").set_embed(embed))
        .await?;

    Ok(())
}
