// rexmit retransmits audio to discord voice channels
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

using System;
using System.Collections.ObjectModel;
using System.Linq;
using System.Threading.Tasks;
using Discord.WebSocket;
using rexmit.Handlers;

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
            FFmpegService ffmpegService
        )
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
                    var audioHandler = AudioHandlers.FirstOrDefault(x =>
                        x.VoiceChannelId == userVoiceChannel.Id
                    );
                    if (audioHandler is not null)
                    {
                        Console.WriteLine("got handle");
                        audioHandler.Queue(path);
                    }
                    else
                    {
                        var voice = user.VoiceState?.VoiceChannel;
                        var client = await voice.ConnectAsync();
                        audioHandler = new AudioHandler(voice.Id, _ffmpegService, client);

                        audioHandler.Queue(path);
                        AudioHandlers.Add(audioHandler);
                    }
                }
            }
        }
    }
}
