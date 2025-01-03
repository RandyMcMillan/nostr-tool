//use crate::sub_commands::custom_event::CustomEventCommand;
//use crate::Commands::CustomEvent;
use clap::{Parser, Subcommand};
use nostr_sdk::Result;
mod sub_commands;
mod utils;

/// Simple CLI application to interact with nostr
#[derive(Parser)]
#[command(name = "gnostr")]
#[command(author = "gnostr <admin@gnostr.org>, 0xtr. <oxtrr@protonmail.com")]
#[command(version = "0.0.1")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Hex or bech32 formatted private key
    #[arg(short, long, action = clap::ArgAction::Append, default_value = "0000000000000000000000000000000000000000000000000000000000000001")]
    sec: Option<String>,
    /// Relay to connect to
    #[arg(short, long, action = clap::ArgAction::Append, default_values_t = ["wss://relay.damus.io".to_string(),"wss://e.nos.lol".to_string()])]
    relays: Vec<String>,
    /// Proof of work difficulty target
    #[arg(short, long, action = clap::ArgAction::Append, default_value_t = 0)]
    difficulty_target: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Ngit sub commands
    Ngit(sub_commands::ngit::NgitSubCommand),
    /// Set metadata. Be aware that this will simply replace your current kind 0 event.
    SetMetadata(sub_commands::set_metadata::SetMetadataSubCommand),
    /// Send text note
    TextNote(sub_commands::text_note::TextNoteSubCommand),
    /// Publish contacts from a CSV file
    PublishContactListCsv(sub_commands::publish_contactlist_csv::PublishContactListCsvSubCommand),
    /// Delete an event
    DeleteEvent(sub_commands::delete_event::DeleteEventSubCommand),
    /// Delete a profile
    DeleteProfile(sub_commands::delete_profile::DeleteProfileSubCommand),
    /// React to an event
    React(sub_commands::react::ReactionSubCommand),
    /// Get all events
    ListEvents(sub_commands::list_events::ListEventsSubCommand),
    /// Generate a new keypair
    GenerateKeypair(sub_commands::generate_keypair::GenerateKeypairSubCommand),
    /// Convert key from bech32 to hex or hex to bech32
    ConvertKey(sub_commands::convert_key::ConvertKeySubCommand),
    /// Vanity public key mining
    Vanity(sub_commands::vanity::VanitySubCommand),
    /// Create a new public channel
    CreatePublicChannel(sub_commands::create_public_channel::CreatePublicChannelSubCommand),
    /// Update channel metadata
    SetChannelMetadata(sub_commands::set_channel_metadata::SetChannelMetadataSubCommand),
    /// Send a message to a public channel
    SendChannelMessage(sub_commands::send_channel_message::SendChannelMessageSubCommand),
    /// Hide a message in a public chat room
    HidePublicChannelMessage(
        sub_commands::hide_public_channel_message::HidePublicChannelMessageSubCommand,
    ),
    /// Mute a public key
    MutePublicKey(sub_commands::mute_publickey::MutePublickeySubCommand),
    /// Broadcast events from file
    BroadcastEvents(sub_commands::broadcast_events::BroadcastEventsSubCommand),
    /// Create a new badge
    CreateBadge(sub_commands::create_badge::CreateBadgeSubCommand),
    /// Publish award badge event
    AwardBadge(sub_commands::award_badge::AwardBadgeSubCommand),
    /// Set profile badges
    ProfileBadges(sub_commands::profile_badges::ProfileBadgesSubCommand),
    /// Create custom event
    CustomEvent(sub_commands::custom_event::CustomEventCommand),
    /// Create a user status event
    SetUserStatus(sub_commands::user_status::UserStatusSubCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse input
    let args: Cli = Cli::parse();

    // Post event
    match &args.command {
        Some(Commands::Ngit(sub_command_args)) => sub_commands::ngit::ngit(sub_command_args).await,
        Some(Commands::SetMetadata(sub_command_args)) => {
            {
                sub_commands::set_metadata::set_metadata(
                    args.sec,
                    args.relays,
                    args.difficulty_target,
                    sub_command_args,
                )
            }
            .await
        }
        Some(Commands::TextNote(sub_command_args)) => {
            sub_commands::text_note::broadcast_textnote(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::PublishContactListCsv(sub_command_args)) => {
            sub_commands::publish_contactlist_csv::publish_contact_list_from_csv_file(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::DeleteEvent(sub_command_args)) => {
            sub_commands::delete_event::delete(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::DeleteProfile(sub_command_args)) => {
            sub_commands::delete_profile::delete(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::React(sub_command_args)) => {
            sub_commands::react::react_to_event(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::ListEvents(sub_command_args)) => {
            sub_commands::list_events::list_events(args.relays, sub_command_args).await
        }
        Some(Commands::GenerateKeypair(sub_command_args)) => {
            sub_commands::generate_keypair::get_new_keypair(sub_command_args).await
        }
        Some(Commands::ConvertKey(sub_command_args)) => {
            sub_commands::convert_key::convert_key(sub_command_args).await
        }
        Some(Commands::Vanity(sub_command_args)) => {
            sub_commands::vanity::vanity(sub_command_args).await
        }
        Some(Commands::CreatePublicChannel(sub_command_args)) => {
            sub_commands::create_public_channel::create_public_channel(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::SetChannelMetadata(sub_command_args)) => {
            sub_commands::set_channel_metadata::set_channel_metadata(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::SendChannelMessage(sub_command_args)) => {
            sub_commands::send_channel_message::send_channel_message(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::HidePublicChannelMessage(sub_command_args)) => {
            sub_commands::hide_public_channel_message::hide_public_channel_message(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::MutePublicKey(sub_command_args)) => {
            sub_commands::mute_publickey::mute_publickey(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::BroadcastEvents(sub_command_args)) => {
            sub_commands::broadcast_events::broadcast_events(args.relays, sub_command_args).await
        }
        Some(Commands::CreateBadge(sub_command_args)) => {
            sub_commands::create_badge::create_badge(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::AwardBadge(sub_command_args)) => {
            sub_commands::award_badge::award_badge(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::ProfileBadges(sub_command_args)) => {
            sub_commands::profile_badges::set_profile_badges(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::CustomEvent(sub_command_args)) => {
            sub_commands::custom_event::create_custom_event(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        Some(Commands::SetUserStatus(sub_command_args)) => {
            sub_commands::user_status::set_user_status(
                args.sec,
                args.relays,
                args.difficulty_target,
                sub_command_args,
            )
            .await
        }
        None => {
            {
                println!("None");
            };
            Ok(())
        }
    }
}
