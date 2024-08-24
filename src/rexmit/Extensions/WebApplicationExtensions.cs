// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

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
        await client.SetCustomStatusAsync("type /help for commands");
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
