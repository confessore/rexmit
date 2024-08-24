// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.Interactions;
using rexmit.Services;
using RunMode = Discord.Interactions.RunMode;

namespace rexmit.Modules;

// A display of portability, which shows how minimal the difference between the 2 frameworks is.
public class InteractionModule(
    FFmpegService ffmpegService,
    ThreadHandlerService threadHandlerService
) : InteractionModuleBase<ShardedInteractionContext>
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
