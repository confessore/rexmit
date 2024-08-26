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
