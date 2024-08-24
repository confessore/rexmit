// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Linq;
using Discord;
using Discord.Commands;
using Discord.Interactions;
using Discord.WebSocket;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.DataProtection;
using Microsoft.AspNetCore.DataProtection.AuthenticatedEncryption;
using Microsoft.AspNetCore.DataProtection.AuthenticatedEncryption.ConfigurationModel;
using Microsoft.AspNetCore.ResponseCompression;
using Microsoft.Extensions.DependencyInjection;
using rexmit.Contexts;
using rexmit.Services;

namespace rexmit.Extensions;

public static class ServiceCollectionExtensions
{
    public static void ConfigureSignalR(this IServiceCollection services)
    {
        var signalRServerBuilder = services.AddSignalR();
        services.AddResponseCompression(options =>
        {
            options.MimeTypes = ResponseCompressionDefaults.MimeTypes.Concat(
                ["application/octet-stream"]
            );
        });

        if (
            Environment.GetEnvironmentVariable("REDIS_URL") is string redisConnectionString
            && redisConnectionString.Length > 0
        )
        {
            signalRServerBuilder.AddStackExchangeRedis(
                redisConnectionString,
                options =>
                {
                    options.Configuration.AbortOnConnectFail = false;
                }
            );

            services
                .AddDataProtection()
                .PersistKeysToDbContext<ApplicationDbContext>()
                .UseCryptographicAlgorithms(
                    new AuthenticatedEncryptorConfiguration()
                    {
                        EncryptionAlgorithm = EncryptionAlgorithm.AES_256_CBC,
                        ValidationAlgorithm = ValidationAlgorithm.HMACSHA256
                    }
                );
        }
    }

    public static void ConfigureDiscordBot(this IServiceCollection services)
    {
        // You specify the amount of shards you'd like to have with the
        // DiscordSocketConfig. Generally, it's recommended to
        // have 1 shard per 1500-2000 guilds your bot is in.
        var config = new DiscordSocketConfig()
        {
            TotalShards = 1,
            GatewayIntents = GatewayIntents.AllUnprivileged | GatewayIntents.MessageContent
        };

        // You should dispose a service provider created using ASP.NET
        // when you are finished using it, at the end of your app's lifetime.
        // If you use another dependency injection framework, you should inspect
        // its documentation for the best way to do this.
        services.AddSingleton(new DiscordShardedClient(config));
        services.AddSingleton<CommandService>();
        services.AddSingleton(x => new InteractionService(
            x.GetRequiredService<DiscordShardedClient>()
        ));
        services.AddSingleton<CommandHandlingService>();
        services.AddSingleton<InteractionHandlingService>();
    }
}
