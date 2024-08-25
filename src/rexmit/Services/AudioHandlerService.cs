// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Collections.ObjectModel;
using Discord.WebSocket;
using System.Threading.Tasks;
using rexmit.Handlers;
using System.Linq;

namespace rexmit.Services
{
    public class AudioHandlerService
    {
        private readonly DiscordShardedClient _discordShardedClient;
        private readonly DiscordService _discordService;
        private readonly FFmpegService _ffmpegService;
        public AudioHandlerService(
            DiscordShardedClient discordShardedClient,
            DiscordService discordService,
            FFmpegService ffmpegService)
        {
            _discordShardedClient = discordShardedClient;
            _discordService = discordService;
            _ffmpegService = ffmpegService;
            AudioHandlers = [];
        }

        public Collection<AudioHandler> AudioHandlers { get; set; }

        public async Task HandleAudio(string userId, string path)
        {
            var guild = _discordService.GetGuildByUserId(userId);
            var user = _discordShardedClient.GetGuild(guild.Id).GetUser(Convert.ToUInt64(userId));
            if (user is not null)
            {
                var userVoiceChannel = user.VoiceState?.VoiceChannel;
                if (userVoiceChannel is not null)
                {
                    var audioHandler = AudioHandlers.FirstOrDefault(x => x.VoiceChannelId == userVoiceChannel.Id);
                    if (audioHandler is not null)
                    {
                        Console.WriteLine("got handle");
                        await audioHandler.PlayAudioAsync(path);
                    }
                    else
                    {
                        var voice = user.VoiceState?.VoiceChannel;
                        var client = await voice.ConnectAsync();
                        audioHandler = new AudioHandler(voice.Id, _ffmpegService, client);

                        await audioHandler.PlayAudioAsync(path);
                        AudioHandlers.Add(audioHandler);
                    }
                }
            }
        }
    }
}
