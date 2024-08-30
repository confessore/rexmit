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

using System;
using System.Linq;
using Discord.WebSocket;

namespace rexmit.Services
{
    public class DiscordService(DiscordShardedClient discordShardedClient)
    {
        public SocketGuild? GetGuildByUserId(ulong userId)
        {
            var guild = discordShardedClient.Guilds.FirstOrDefault(x =>
                x.VoiceChannels.FirstOrDefault(y =>
                    y.Users.FirstOrDefault(z => z.Id == userId) != null
                ) != null
            );
            return guild;
        }
    }
}
