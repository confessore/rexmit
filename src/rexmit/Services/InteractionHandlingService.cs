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
using Discord.Interactions;
using Discord.WebSocket;
using Microsoft.Extensions.DependencyInjection;

namespace rexmit.Services;

public class InteractionHandlingService
{
    private readonly InteractionService _service;
    private readonly DiscordShardedClient _client;
    private readonly IServiceProvider _provider;

    public InteractionHandlingService(IServiceProvider services)
    {
        _service = services.GetRequiredService<InteractionService>();
        _client = services.GetRequiredService<DiscordShardedClient>();
        _provider = services;

        _service.Log += LogAsync;
        _client.InteractionCreated += OnInteractionAsync;
        _client.ShardReady += ReadyAsync;
        // For examples on how to handle post execution,
        // see the InteractionFramework samples.
    }

    // Register all modules, and add the commands from these modules to either guild or globally depending on the build state.
    public async Task InitializeAsync()
    {
        await _service.AddModulesAsync(typeof(InteractionHandlingService).Assembly, _provider);
    }

    private async Task OnInteractionAsync(SocketInteraction interaction)
    {
        _ = Task.Run(async () =>
        {
            var context = new ShardedInteractionContext(_client, interaction);
            await _service.ExecuteCommandAsync(context, _provider);
        });
        await Task.CompletedTask;
    }

    private Task LogAsync(LogMessage log)
    {
        Console.WriteLine(log.ToString());

        return Task.CompletedTask;
    }

    private async Task ReadyAsync(DiscordSocketClient _)
    {
#if DEBUG
        await _service.RegisterCommandsToGuildAsync(
            1100799461581668372 // rexmit
        );
        await _service.RegisterCommandsToGuildAsync(
            900113002198626337 // cupcake
        );
#else
        await _service.RegisterCommandsGloballyAsync();
#endif
    }
}
