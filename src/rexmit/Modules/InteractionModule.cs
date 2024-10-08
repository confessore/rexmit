﻿// rexmit retransmits audio to discord voice channels
//
// Copyright (C) 2024  Steven Confessore
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY, without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.Interactions;
using rexmit.Services;
using RunMode = Discord.Interactions.RunMode;

namespace rexmit.Modules;

// A display of portability, which shows how minimal the difference between the 2 frameworks is.
public class InteractionModule(FFmpegService ffmpegService, AudioHandlerService audioHandlerService)
    : InteractionModuleBase<ShardedInteractionContext>
{
    [SlashCommand("completion", "chat completion")]
    public async Task CompletionAsync([Remainder] string prompt)
    {
        await DeferAsync(false);

        await FollowupAsync("NOT IMPLEMENTED");
    }

    [SlashCommand("help", "help")]
    public async Task HelpAsync()
    {
        await DeferAsync(false);
        var embed = new EmbedBuilder().AddField("/help", "displays this menu").Build();
        await FollowupAsync(embed: embed);
    }

    // The command's Run Mode MUST be set to RunMode.Async, otherwise, being connected to a voice channel will block the gateway thread.
    [SlashCommand("join", "joins the voice channel", runMode: RunMode.Async)]
    public async Task JoinChannel(IVoiceChannel channel = null)
    {
        await DeferAsync(false);
        channel = channel ?? (Context.User as IGuildUser)?.VoiceChannel;
        if (channel == null)
        {
            await Context.Channel.SendMessageAsync(
                "User must be in a voice channel, or a voice channel must be passed as an argument."
            );
            return;
        }

        // For the next step with transmitting audio, you would want to pass this Audio Client in to a service.
        _ = await channel.ConnectAsync();
        await FollowupAsync($"joined voice channel {channel}");
    }

    // The command's Run Mode MUST be set to RunMode.Async, otherwise, being connected to a voice channel will block the gateway thread.
    [SlashCommand("leave", "leaves the voice channel", runMode: RunMode.Async)]
    public async Task LeaveChannel(IVoiceChannel channel = null)
    {
        await DeferAsync(false);
        channel = channel ?? (Context.User as IGuildUser)?.VoiceChannel;
        if (channel == null)
        {
            await Context.Channel.SendMessageAsync(
                "User must be in a voice channel, or a voice channel must be passed as an argument."
            );
            return;
        }

        // For the next step with transmitting audio, you would want to pass this Audio Client in to a service.
        await channel.DisconnectAsync();
        await FollowupAsync($"left voice channel {channel}");
    }

    // The command's Run Mode MUST be set to RunMode.Async, otherwise, being connected to a voice channel will block the gateway thread.
    [SlashCommand("okay", "okay", runMode: RunMode.Async)]
    public async Task Okay(IVoiceChannel channel = null)
    {
        await DeferAsync(false);
        channel ??= (Context.User as IGuildUser)?.VoiceChannel;
        if (channel == null)
        {
            await Context.Channel.SendMessageAsync(
                "User must be in a voice channel, or a voice channel must be passed as an argument."
            );
            return;
        }

        var client = await channel.ConnectAsync();

        await ffmpegService.SendFFmpegAsync(client, "./okbabybyebye.mp3");
        await channel.DisconnectAsync();

        /*var threadHandler = threadHandlerService.ThreadHandlers.FirstOrDefault(x => x.ChannelId == channel.Id);
if (threadHandler == null)
{
    var currentUser = Context.Guild.GetUser(Context.Client.CurrentUser.Id);
    if (currentUser.VoiceState?.VoiceChannel == null)
    {
        var client = await channel.ConnectAsync();
        threadHandler = new ThreadHandler(this, ffmpegService, client);

        threadHandler.OnTrackStart += () =>
        {
            Console.WriteLine("TRACK START");
        };
        threadHandler.OnTrackEnd += () =>
        {
            Console.WriteLine("TRACK END");
        };
        threadHandlerService.ThreadHandlers.Add(threadHandler);
        threadHandler.Queue("./okbabybyebye.mp3");
    }
}
else
{
    threadHandler.Queue("./okbabybyebye.mp3");
}*/
        // For the next step with transmitting audio, you would want to pass this Audio Client in to a service.
        await FollowupAsync($"ok baby bye bye");
    }
}
