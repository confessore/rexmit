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
using System.Threading.Tasks;
using Discord;
using Discord.WebSocket;
using Microsoft.AspNetCore.Builder;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using rexmit.Contexts;
using rexmit.Services;

namespace rexmit.Extensions;

internal static class WebApplicationExtensions
{
    public static async Task<WebApplication> MigrateApplicationDbContextAsync(
        this WebApplication webApplication
    )
    {
        using var scope = webApplication
            .Services.GetRequiredService<IServiceScopeFactory>()
            .CreateScope();
        using var context = scope.ServiceProvider.GetRequiredService<ApplicationDbContext>();
        await context.Database.MigrateAsync();
        return webApplication;
    }

    public static async Task StartDiscordBotAsync(this WebApplication webApplication)
    {
        var client = webApplication.Services.GetRequiredService<DiscordShardedClient>();

        // The Sharded Client does not have a Ready event.
        // The ShardReady event is used instead, allowing for individual
        // control per shard.
        client.ShardReady += ReadyAsync;
        client.Log += LogAsync;

        await webApplication
            .Services.GetRequiredService<InteractionHandlingService>()
            .InitializeAsync();

        await webApplication
            .Services.GetRequiredService<CommandHandlingService>()
            .InitializeAsync();

        // Tokens should be considered secret data, and never hard-coded.
        await client.LoginAsync(TokenType.Bot, Environment.GetEnvironmentVariable("DISCORD_TOKEN"));
        await client.StartAsync();
#if !DEBUG
        await client.SetCustomStatusAsync("rexmit.balasolu.com");
#elif DEBUG
        await client.SetCustomStatusAsync("carrington.balasolu.com");
#endif
    }

    private static Task ReadyAsync(DiscordSocketClient shard)
    {
        Console.WriteLine($"Shard Number {shard.ShardId} is connected and ready!");
        return Task.CompletedTask;
    }

    private static Task LogAsync(LogMessage log)
    {
        Console.WriteLine(log.ToString());
        return Task.CompletedTask;
    }
}
