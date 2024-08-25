// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Linq;
using Discord.WebSocket;

namespace rexmit.Services
{
    public class DiscordService(DiscordShardedClient discordShardedClient)
    {

        public SocketGuild? GetGuildByUserId(string userId)
        {
            var guild = discordShardedClient.Guilds.FirstOrDefault(x =>
                x.VoiceChannels.FirstOrDefault(y =>
                    y.Users.FirstOrDefault(z => z.Id == Convert.ToUInt64(userId)) != null) != null);
            return guild;
        }
    }
}
